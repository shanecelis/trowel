use alloc::{boxed::Box, string::String, vec::Vec};

#[allow(dead_code)]
pub enum WriteMode {
    ReadOnly,
    Append,
    Truncate,
}


pub trait FileSys {
    type FileError;
    type File;

    fn file_exists(&mut self, name: &str) -> Result<bool, Self::FileError>;
    fn open_file(&mut self, name: &str, mode: WriteMode) -> Result<Self::File, Self::FileError>;
    fn delete_file(&mut self, name: &str) -> Result<(), Self::FileError>;
    fn list_files(&mut self) -> Result<alloc::vec::Vec<String>, Self::FileError>;
}

pub trait FS {
    /// Returns the size of the file and a slice of the file's contents.
    fn read_file(&mut self, name: &str) -> Option<(usize, Box<[u8]>)>;

    /// Returns true if the file was written successfully.
    fn write_file(&mut self, name: &str, data: &[u8], mode: WriteMode) -> bool;

    /// Returns true if the file was deleted successfully.
    fn delete_file(&mut self, name: &str) -> bool;

    /// Returns a list of all files in the filesystem.
    fn list_files(&mut self) -> Vec<String>;

    /// Returns true if the file exists.
    fn file_exists(&mut self, name: &str) -> bool;
}
