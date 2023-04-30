use crate::fs;
use core::cell::RefCell;
use alloc::{boxed::Box, string::String, rc::Rc};
use core::mem;
use embedded_sdmmc::{BlockSpi, Controller, Mode, File, BlockDevice, Error, VolumeIdx};
use rp_pico::hal::{
    gpio::{bank0::Gpio21, Output, Pin, PushPull},
    pac::SPI0,
    spi::Enabled,
    Spi,
};
use shared_bus::{NullMutex, SpiProxy};
use genio::{Read, Write};
type BlockSpiType<'a> = BlockSpi<'a, SpiProxy<'a, NullMutex<Spi<Enabled, SPI0, 8>>>, Pin<Gpio21, Output<PushPull>>>;
pub struct FSClock {}
impl embedded_sdmmc::TimeSource for FSClock {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp::from_fat(0, 0)
    }
}

pub struct SPIFS<'a> {
    controller: RefCell<Controller<BlockSpiType<'a>,
                           FSClock,
                           4,
                           4>>,
    volume: RefCell<embedded_sdmmc::Volume>,
    root: embedded_sdmmc::Directory,
}

#[derive(Clone)]
struct RefSPIFS<'a>(Rc<SPIFS<'a>>);

impl<'a> SPIFS<'a> {
    pub fn new(
        controller: Controller<
            BlockSpi<
                'a,
                SpiProxy<'a, NullMutex<Spi<Enabled, SPI0, 8>>>,
                Pin<Gpio21, Output<PushPull>>,
            >,
            FSClock,
            4,
            4,
        >,
        volume: embedded_sdmmc::Volume,
        root: embedded_sdmmc::Directory,
    ) -> Self {
        Self {
            controller: RefCell::new(controller),
            volume: RefCell::new(volume),
            root,
        }
    }
}

struct ReadFile<'a> {
    file: File,
    spifs: RefSPIFS<'a>,
}

impl<'a> Read for ReadFile<'a> {
    type ReadError = Error<<BlockSpiType<'a> as BlockDevice>::Error>;
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::ReadError> {
        let spifs = &self.spifs.0;
        let mut controller = spifs.controller.borrow_mut();

        let volume = spifs.volume.borrow();
        controller.read(&volume, &mut self.file, buf)
    }
}

impl<'a> Drop for ReadFile<'a> {
    fn drop(&mut self) {
        let spifs = &self.spifs.0;
        let volume = spifs.volume.borrow();
        let dummy : mem::MaybeUninit<File> = mem::MaybeUninit::uninit();
        // let file = mem::replace(&mut self.file, unsafe { mem::uninitialized() });
        let file = mem::replace(&mut self.file, unsafe { dummy.assume_init() });
        spifs.controller
            .borrow_mut()
            .close_file(&volume,
                        file)
            .expect("Unable to close file");
    }
}

struct WriteFile<'a> {
    file: File,
    spifs: SPIFS<'a>,
}

// impl<'a> Write for WriteFile<'a> {
//     type WriteError = Error<<BlockSpiType<'a> as BlockDevice>::Error>;
//     type FlushError = ();
//     fn write(&mut self, buf: &[u8]) -> Result<usize, Self::WriteError> {
//         self.spifs.controller.write(&mut self.spifs.volume, &mut self.file, buf)
//     }

//     fn flush(&mut self) -> Result<(), Self::FlushError> {
//         // There doesn't appear to be any flush possible.
//         Ok(())
//     }

//     fn size_hint(&mut self, _bytes: usize) {
//     }

//     fn uses_size_hint(&self) -> bool {
//         false
//     }
// }

impl<'a> RefSPIFS<'a> {

    fn _file_exists(&mut self, name: &str) -> Result<bool,
                                                     <ReadFile<'a> as Read>::ReadError> {
        let spifs = &self.0;

        spifs.controller
             .borrow_mut()
            .find_directory_entry(&spifs.volume.borrow_mut(), &spifs.root, name)
            .map(|_| true)
    }

    fn _read_file(&mut self, name: &str) -> Result<ReadFile<'a>,
                                                   <ReadFile<'a> as Read>::ReadError> {

        let spifs = &self.0;
        let mut controller = spifs.controller.borrow_mut();

        let mut volume = controller.get_volume(VolumeIdx(0))?;
        let root = controller.open_root_dir(&volume)?;
        controller
            .open_file_in_dir(&mut volume, &root, name, Mode::ReadOnly)
            .map(|f| ReadFile { file: f, spifs: self.clone() })
    }
}

impl fs::FS for SPIFS<'_> {
    fn file_exists(&mut self, name: &str) -> bool {

        // let mut spifs = self.0;
        // let mut controller = self.controller.borrow_mut();
        self.controller
            .borrow_mut()
            .find_directory_entry(&self.volume.borrow_mut(), &self.root, name)
            .is_ok()
    }


    fn read_file(&mut self, name: &str) -> Option<(usize, Box<[u8]>)> {
        let mut volume = self.volume.borrow_mut();
        let file =
            self.controller
            .borrow_mut()
                .open_file_in_dir(&mut volume, &self.root, name, Mode::ReadOnly);

        let mut file = file.ok()?;
        let mut buf = vec![0u8; file.length() as usize];

        file.seek_from_start(0).unwrap();
        let bytes_read = self
            .controller
            .borrow_mut()
            .read(&mut volume, &mut file, &mut buf)
            .ok();
        self.controller
            .borrow_mut()
            .close_file(&volume, file).unwrap();
        let bytes_read = bytes_read?;
        Some((bytes_read, buf.into_boxed_slice()))
    }

    fn write_file(&mut self, name: &str, data: &[u8], mode: fs::WriteMode) -> bool {
        let mut volume = self.volume.borrow_mut();
        let mut file = self
            .controller
            .borrow_mut()
            .open_file_in_dir(
                &mut volume,
                &self.root,
                name,
                match mode {
                    fs::WriteMode::Append => Mode::ReadWriteCreateOrAppend,
                    fs::WriteMode::Truncate => Mode::ReadWriteCreateOrTruncate,
                },
            )
            .expect("Failed to open file");
        let ret = self
            .controller
            .borrow_mut()
            .write(&mut volume, &mut file, data)
            .is_ok();
        self.controller
            .borrow_mut()
            .close_file(&volume, file).unwrap();
        ret
    }

    fn delete_file(&mut self, name: &str) -> bool {
        let mut volume = self.volume.borrow_mut();
        self.controller
            .borrow_mut()
            .delete_file_in_dir(&mut volume, &self.root, name)
            .is_ok()
    }

    fn list_files(&mut self) -> alloc::vec::Vec<String> {
        let mut volume = self.volume.borrow_mut();
        let mut names = alloc::vec::Vec::new();
        self.controller
            .borrow_mut()
            .iterate_dir(&mut volume, &self.root, |entry| {
                names.push(format!("{}", entry.name));
            })
            .unwrap();
        names
    }
}
