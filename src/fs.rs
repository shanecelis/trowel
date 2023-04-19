use alloc::{boxed::Box, string::String, vec::Vec};

pub enum WriteMode {
    Append,
    Truncate,
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
