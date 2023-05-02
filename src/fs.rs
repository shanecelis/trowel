use alloc::{string::String, vec::Vec};
use genio::{Read, Write};

#[derive(PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum Mode {
    ReadOnly,
    Append,
    Truncate,
}

pub trait FileSys {
    type FileError;
    type File: Read + Write;

    fn file_exists(&mut self, name: &str) -> Result<bool, Self::FileError>;
    fn open_file(&mut self, name: &str, mode: Mode) -> Result<Self::File, Self::FileError>;
    fn delete_file(&mut self, name: &str) -> Result<(), Self::FileError>;
    fn list_files(&mut self) -> Result<Vec<String>, Self::FileError>;
}
