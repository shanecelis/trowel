use crate::{WriteMode, FS};
use std::{fs, io::Write};

pub struct PCFS {}

impl PCFS {
    pub fn new() -> Self {
        Self {}
    }
}

impl FS for PCFS {
    fn file_exists(&mut self, name: &str) -> bool {
        let name = format!("fs/{}", name);
        fs::metadata(name).is_ok()
    }

    fn read_file(&mut self, name: &str) -> String {
        let name = format!("fs/{}", name);
        fs::read_to_string(name).unwrap()
    }

    fn delete_file(&mut self, name: &str) -> bool {
        let name = format!("fs/{}", name);
        fs::remove_file(name).is_ok()
    }

    fn write_file(&mut self, name: &str, data: &[u8], mode: WriteMode) -> bool {
        let name = format!("fs/{}", name);
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
        for entry in fs::read_dir("fs").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            names.push(name);
        }
        names
    }
}
