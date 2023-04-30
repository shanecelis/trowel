use crate::fs;
use core::cell::RefCell;
use alloc::{boxed::Box, string::String, rc::Rc};
use core::mem;
use embedded_sdmmc::{BlockSpi, Controller, Mode, File, BlockDevice, Error};
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

struct SdFile<'a> {
    file: File,
    spifs: RefSPIFS<'a>,
}

impl<'a> Read for SdFile<'a> {
    type ReadError = Error<<BlockSpiType<'a> as BlockDevice>::Error>;
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::ReadError> {
        let spifs = &self.spifs.0;
        let volume = spifs.volume.borrow();
        spifs.controller
            .borrow_mut()
            .read(&volume, &mut self.file, buf)
    }
}

impl<'a> Write for SdFile<'a> {
    type WriteError = Error<<BlockSpiType<'a> as BlockDevice>::Error>;
    type FlushError = ();
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::WriteError> {
        let spifs = &self.spifs.0;
        let mut volume = spifs.volume.borrow_mut();
        spifs.controller
            .borrow_mut()
            .write(&mut volume, &mut self.file, buf)
    }

    fn flush(&mut self) -> Result<(), Self::FlushError> {
        // There doesn't appear to be any flush possible.
        Ok(())
    }

    fn size_hint(&mut self, _bytes: usize) {
    }

    fn uses_size_hint(&self) -> bool {
        false
    }
}

impl<'a> Drop for SdFile<'a> {
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

impl<'a> fs::FileSys for RefSPIFS<'a> {
    type FileError = <SdFile<'a> as Read>::ReadError;
    type File = SdFile<'a>;

    fn file_exists(&mut self, name: &str) -> Result<bool, Self::FileError> {
        let spifs = &self.0;
        spifs.controller
             .borrow_mut()
            .find_directory_entry(&spifs.volume.borrow_mut(), &spifs.root, name)
            .map(|_| true)
    }

    fn open_file(&mut self, name: &str, mode: fs::WriteMode) -> Result<SdFile<'a>, Self::FileError> {

        let spifs = &self.0;
        let mut volume = spifs.volume.borrow_mut();
        spifs.controller
            .borrow_mut()
            .open_file_in_dir(&mut volume, &spifs.root, name, mode.into())
            .map(|f| SdFile { file: f, spifs: self.clone() })
    }

    fn delete_file(&mut self, name: &str) -> Result<(), Self::FileError> {

        let spifs = &self.0;
        let mut volume = spifs.volume.borrow_mut();
        spifs.controller
            .borrow_mut()
            .delete_file_in_dir(&mut volume, &spifs.root, name)
    }

    fn list_files(&mut self) -> Result<alloc::vec::Vec<String>, Self::FileError> {
        let spifs = &self.0;
        let mut volume = spifs.volume.borrow_mut();
        let mut names = alloc::vec::Vec::new();
        spifs.controller
            .borrow_mut()
            .iterate_dir(&mut volume, &spifs.root, |entry| {
                names.push(format!("{}", entry.name));
            })?;
        Ok(names)
    }
}

impl From<fs::WriteMode> for Mode {

    fn from(mode: fs::WriteMode) -> Mode {
        match mode {
            fs::WriteMode::ReadOnly => Mode::ReadOnly,
            fs::WriteMode::Append => Mode::ReadWriteCreateOrAppend,
            fs::WriteMode::Truncate => Mode::ReadWriteCreateOrTruncate,
        }
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
                mode.into())
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
