use crate::{WriteMode};
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

// pub struct PcFile {
//     path: String
// }

impl crate::fs::FileSys for PCFS {
    type FileError = io::Error;
    type File = GenioIo<fs::File>;

    fn file_exists(&mut self, name: &str) -> Result<bool, Self::FileError> {
        let name = format!("{}/{}", self.prefix, name);
        fs::metadata(name)
            .map(|_| true)
    }
    fn open_file(&mut self, name: &str, mode: WriteMode) -> Result<Self::File, Self::FileError> {
        let name = format!("{}/{}", self.prefix, name);
        let file = match mode {
            WriteMode::Append => fs::OpenOptions::new()
                .read(false)
                .create(true)
                .append(true)
                .write(true)
                .open(name)?,
            WriteMode::Truncate => fs::OpenOptions::new()
                .read(false)
                .create(true)
                .append(false)
                .write(true)
                .open(name)?,
            WriteMode::ReadOnly => fs::OpenOptions::new()
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

// impl FS for PCFS {
//     fn file_exists(&mut self, name: &str) -> bool {
//         let name = format!("{}/{}", self.prefix, name);
//         fs::metadata(name).is_ok()
//     }

//     fn read_file(&mut self, name: &str) -> Option<(usize, Box<[u8]>)> {
//         let name = format!("{}/{}", self.prefix, name);
//         let data = fs::read(name).ok()?;
//         Some((data.len(), data.into_boxed_slice()))
//     }

//     fn delete_file(&mut self, name: &str) -> bool {
//         let name = format!("{}/{}", self.prefix, name);
//         fs::remove_file(name).is_ok()
//     }

//     fn write_file(&mut self, name: &str, data: &[u8], mode: WriteMode) -> bool {
//         let name = format!("{}/{}", self.prefix, name);
//         match mode {
//             WriteMode::ReadOnly => false,
//             WriteMode::Append => fs::OpenOptions::new()
//                 .create(true)
//                 .append(true)
//                 .open(name)
//                 .expect("Unable to open file")
//                 .write_all(data)
//                 .is_ok(),
//             WriteMode::Truncate => fs::write(name, data).is_ok(),
//         }
//     }

//     fn list_files(&mut self) -> Vec<String> {
//         let mut names = Vec::new();
//         for entry in fs::read_dir(&self.prefix).unwrap() {
//             let entry = entry.unwrap();
//             let path = entry.path();
//             let name = path.file_name().unwrap().to_str().unwrap().to_string();
//             names.push(name);
//         }
//         names
//     }
// }
