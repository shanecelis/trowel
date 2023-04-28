use alloc::{string::String, vec::Vec};

pub enum WriteMode {
    Append,
    Truncate,
}

pub trait FS {
    fn read_file(&mut self, name: &str) -> String;

    fn write_file(&mut self, name: &str, data: &[u8], mode: WriteMode) -> bool;

    fn delete_file(&mut self, name: &str) -> bool;

    fn list_files(&mut self) -> Vec<String>;

    fn file_exists(&mut self, name: &str) -> bool;
}
