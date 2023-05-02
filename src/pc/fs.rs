use crate::{Mode};
use std::{fs, io, env};
use genio::std_impls::GenioIo;

pub struct PCFS {
    prefix: String,
}

impl PCFS {
    pub fn new(prefix: Option<String>) -> Self {
        Self {
            prefix: prefix.or(env::var_os("TROWEL_FS")
                .map(|s| s.into_string().expect("Invalid utf-8 string in TROWEL_FS")))
                .unwrap_or_else(|| "fs".to_owned())
        }
    }
}

impl crate::fs::FileSys for PCFS {
    type FileError = io::Error;
    type File = GenioIo<fs::File>;

    fn file_exists(&mut self, name: &str) -> Result<bool, Self::FileError> {
        let name = format!("{}/{}", self.prefix, name);
        fs::metadata(name)
            .map(|_| true)
    }
    fn open_file(&mut self, name: &str, mode: Mode) -> Result<Self::File, Self::FileError> {
        let name = format!("{}/{}", self.prefix, name);
        let file = match mode {
            Mode::Append => fs::OpenOptions::new()
                .read(false)
                .create(true)
                .append(true)
                .write(true)
                .open(name)?,
            Mode::Truncate => fs::OpenOptions::new()
                .read(false)
                .create(true)
                .append(false)
                .write(true)
                .open(name)?,
            Mode::ReadOnly => fs::OpenOptions::new()
                .create(false)
                .read(true)
                .append(false)
                .write(false)
                .open(name)?
        };
        Ok(GenioIo::new(file))
    }
    fn delete_file(&mut self, name: &str) -> Result<(), Self::FileError> {
        let name = format!("{}/{}", self.prefix, name);
        fs::remove_file(name)
    }
    fn list_files(&mut self) -> Result<alloc::vec::Vec<String>, Self::FileError> {
        let mut names = Vec::new();
        for entry in fs::read_dir(&self.prefix)? {
            let entry = entry?;
            let path = entry.path();
            let name = path.file_name()
                .expect("Could not get file name")
                           .to_str()
                .expect("Could not convert filename to utf-8")
                           .to_owned();
            names.push(name);
        }
        Ok(names)
    }
}

