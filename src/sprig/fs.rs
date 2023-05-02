use crate::fs;
use core::cell::RefCell;
use alloc::{string::String, rc::Rc};
use core::mem;
use embedded_sdmmc::{BlockSpi, Controller, Mode, File, BlockDevice, Error, VolumeIdx, SdMmcError};
use rp_pico::hal::{
    gpio::{bank0::Gpio21, Output, Pin, PushPull},
    pac::SPI0,
    spi::Enabled,
    Spi,
};
use shared_bus::{NullMutex, SpiProxy};
use genio::{Read, Write};
use crate::sprig::SdMmcSpi0;

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
pub struct RefSPIFS<'a>(pub Rc<SPIFS<'a>>);

impl<'a> SPIFS<'a> {
    pub fn new(
        // controller: Controller<BlockSpiType<'a>,
        //                        FSClock,
        //                        4,
        //                        4>,
        // volume: embedded_sdmmc::Volume,
        // root: embedded_sdmmc::Directory,
        sd_spi: &'a mut SdMmcSpi0<'a>,
    ) -> Result<Self, Error<SdMmcError>> {

        let time_source = crate::sprig::fs::FSClock {};
        let block = sd_spi.acquire().map_err(|e| Error::DeviceError(e))?;
        let mut cont = Controller::new(block, time_source);
        let volume = cont.get_volume(VolumeIdx(0))?;
        let root = cont.open_root_dir(&volume)?;
        Ok(Self {
            controller: RefCell::new(cont),
            volume: RefCell::new(volume),
            root,
        })
    }
}

pub struct SdFile<'a> {
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

    fn open_file(&mut self, name: &str, mode: fs::Mode) -> Result<SdFile<'a>, Self::FileError> {

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

impl From<fs::Mode> for Mode {

    fn from(mode: fs::Mode) -> Mode {
        match mode {
            fs::Mode::ReadOnly => Mode::ReadOnly,
            fs::Mode::Append => Mode::ReadWriteCreateOrAppend,
            fs::Mode::Truncate => Mode::ReadWriteCreateOrTruncate,
        }
    }
}

