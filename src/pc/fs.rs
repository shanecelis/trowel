use crate::{WriteMode, FS};
use std::{fs, io::Write, env};

pub struct PCFS {
    prefix: String,

}

impl PCFS {
    pub fn new() -> Self {
        Self {
            prefix: env::var_os("TROWEL_FS")
                .map(|s| s.into_string().expect("Invalid utf-8 string in TROWEL_FS"))
                .unwrap_or_else(|| "fs".to_owned())
        }
    }
}

impl FS for PCFS {
    fn file_exists(&mut self, name: &str) -> bool {
        let name = format!("{}/{}", self.prefix, name);
        fs::metadata(name).is_ok()
    }

    fn read_file(&mut self, name: &str) -> Option<(usize, Box<[u8]>)> {
        let name = format!("{}/{}", self.prefix, name);
        let data = fs::read(name).ok()?;
        Some((data.len(), data.into_boxed_slice()))
    }

    fn delete_file(&mut self, name: &str) -> bool {
        let name = format!("{}/{}", self.prefix, name);
        fs::remove_file(name).is_ok()
    }

    fn write_file(&mut self, name: &str, data: &[u8], mode: WriteMode) -> bool {
        let name = format!("{}/{}", self.prefix, name);
        match mode {
            WriteMode::Append => fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(name)
                .unwrap()
                .write_all(data)
                .is_ok(),
            WriteMode::Truncate => fs::write(name, data).is_ok(),
        }
    }

    fn list_files(&mut self) -> Vec<String> {
        let mut names = Vec::new();
        for entry in fs::read_dir(&self.prefix).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            names.push(name);
        }
        names
    }
}
