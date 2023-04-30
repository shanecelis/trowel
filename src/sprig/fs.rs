use crate::fs;
use core::cell::RefCell;
use alloc::{boxed::Box, string::String, rc::Rc};
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
    controller: Controller<BlockSpiType<'a>,
                           FSClock,
                           4,
                           4>,
    volume: embedded_sdmmc::Volume,
    root: embedded_sdmmc::Directory,
}

#[derive(Clone)]
struct RefSPIFS<'a>(Rc<RefCell<SPIFS<'a>>>);

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
            controller,
            volume,
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
        let mut spifs = self.spifs.0.borrow_mut();

        let volume = spifs.controller.get_volume(VolumeIdx(0))?;
            // let root = cont.open_root_dir(&volume).unwrap();
        // spifs.controller.read(&spifs.volume, &mut self.file, buf)
        spifs.controller.read(&volume, &mut self.file, buf)
    }
}

struct WriteFile<'a> {
    file: File,
    spifs: SPIFS<'a>,
}

impl<'a> Write for WriteFile<'a> {
    type WriteError = Error<<BlockSpiType<'a> as BlockDevice>::Error>;
    type FlushError = ();
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::WriteError> {
        self.spifs.controller.write(&mut self.spifs.volume, &mut self.file, buf)
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

impl<'a> RefSPIFS<'a> {

    fn _file_exists(&mut self, name: &str) -> Result<bool,
                                                     <ReadFile<'a> as Read>::ReadError> {
        let mut spifs = self.0.borrow_mut();
        spifs.controller
            .find_directory_entry(&mut self.volume, &self.root, name)
            .map(|_| true)
    }

    fn _read_file(&mut self, name: &str) -> Result<ReadFile<'a>,
                                                   <ReadFile<'a> as Read>::ReadError> {
        let mut spifs = self.0.borrow_mut();

        let mut volume = spifs.controller.get_volume(VolumeIdx(0))?;
        let root = spifs.controller.open_root_dir(&volume)?;
        spifs.controller
            .open_file_in_dir(&mut volume, &root, name, Mode::ReadOnly)
            .map(|f| ReadFile { file: f, spifs: self.clone() })
    }
}

impl fs::FS for SPIFS<'_> {
    fn file_exists(&mut self, name: &str) -> bool {
        self.controller
            .find_directory_entry(&mut self.volume, &self.root, name)
            .is_ok()
    }


    fn read_file(&mut self, name: &str) -> Option<(usize, Box<[u8]>)> {
        let file =
            self.controller
                .open_file_in_dir(&mut self.volume, &self.root, name, Mode::ReadOnly);

        let mut file = file.ok()?;
        let mut buf = vec![0u8; file.length() as usize];

        file.seek_from_start(0).unwrap();
        let bytes_read = self
            .controller
            .read(&mut self.volume, &mut file, &mut buf)
            .ok();
        self.controller.close_file(&self.volume, file).unwrap();
        let bytes_read = bytes_read?;
        Some((bytes_read, buf.into_boxed_slice()))
    }

    fn write_file(&mut self, name: &str, data: &[u8], mode: fs::WriteMode) -> bool {
        let mut file = self
            .controller
            .open_file_in_dir(
                &mut self.volume,
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
            .write(&mut self.volume, &mut file, data)
            .is_ok();
        self.controller.close_file(&self.volume, file).unwrap();
        ret
    }

    fn delete_file(&mut self, name: &str) -> bool {
        self.controller
            .delete_file_in_dir(&mut self.volume, &self.root, name)
            .is_ok()
    }

    fn list_files(&mut self) -> alloc::vec::Vec<String> {
        let mut names = alloc::vec::Vec::new();
        self.controller
            .iterate_dir(&mut self.volume, &self.root, |entry| {
                names.push(format!("{}", entry.name));
            })
            .unwrap();
        names
    }
}
