use crate::fs;
use alloc::{boxed::Box, string::String};
use embedded_sdmmc::{BlockSpi, Controller, Mode};
use rp2040_hal::{
    gpio::{bank0::Gpio21, Output, Pin, PushPull},
    pac::SPI0,
    spi::Enabled,
    Spi,
};
use shared_bus::{NullMutex, SpiProxy};

pub struct FSClock {}
impl embedded_sdmmc::TimeSource for FSClock {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp::from_fat(0, 0)
    }
}

pub struct SPIFS<'a> {
    controller: Controller<
        BlockSpi<'a, SpiProxy<'a, NullMutex<Spi<Enabled, SPI0, 8>>>, Pin<Gpio21, Output<PushPull>>>,
        FSClock,
        4,
        4,
    >,
    volume: embedded_sdmmc::Volume,
    root: embedded_sdmmc::Directory,
}

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
