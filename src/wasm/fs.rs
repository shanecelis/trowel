use crate::{WriteMode, FS};
use web_sys::window;

pub struct WebFS {
    local_storage: web_sys::Storage,
}

impl WebFS {
    pub fn new() -> Self {
        Self {
            local_storage: window().unwrap().local_storage().unwrap().unwrap(),
        }
    }
}

impl FS for WebFS {
    fn file_exists(&mut self, name: &str) -> bool {
        self.local_storage.get_item(name).unwrap().is_some()
    }

    fn read_file(&mut self, name: &str) -> Option<(usize, Box<[u8]>)> {
        let data = self.local_storage.get_item(name).unwrap();

        if let Some(data) = data {
            Some((data.len(), data.into_bytes().into_boxed_slice()))
        } else {
            None
        }
    }

    fn delete_file(&mut self, name: &str) -> bool {
        let did_exist = self.file_exists(name);

        self.local_storage.remove_item(name).unwrap();

        did_exist
    }

    fn write_file(&mut self, name: &str, data: &[u8], mode: WriteMode) -> bool {
        let did_exist = self.file_exists(name);

        match mode {
            WriteMode::Append => {
                let existing_data = self.read_file(name).unwrap().1;
                let mut new_buf = vec![0u8; existing_data.len() + data.len()];
                new_buf[..existing_data.len()].copy_from_slice(&existing_data);
                new_buf[existing_data.len()..].copy_from_slice(data);
                self.local_storage
                    .set_item(name, &String::from_utf8(new_buf).unwrap())
                    .unwrap();
            }
            WriteMode::Truncate => {
                self.local_storage
                    .set_item(name, &String::from_utf8(data.to_vec()).unwrap())
                    .unwrap();
            }
        }

        did_exist
    }

    fn list_files(&mut self) -> Vec<String> {
        let mut files = vec![];

        for i in 0..self.local_storage.length().unwrap() {
            files.push(self.local_storage.key(i).unwrap().unwrap());
        }

        files
    }
}
