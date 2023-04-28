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

    fn read_file(&mut self, name: &str) -> String {
        self.local_storage
            .get_item(name)
            .unwrap()
            .unwrap_or("".to_string())
    }

    fn delete_file(&mut self, name: &str) -> bool {
        let did_exist = self.file_exists(name);

        self.local_storage.remove_item(name).unwrap();

        did_exist
    }

    fn write_file(&mut self, name: &str, data: &[u8], mode: WriteMode) -> bool {
        match mode {
            WriteMode::Append => {
                let data = self.read_file(name) + &String::from_utf8_lossy(data);
                self.local_storage.set_item(name, &data).is_ok()
            }
            WriteMode::Truncate => self
                .local_storage
                .set_item(name, &String::from_utf8_lossy(data))
                .is_ok(),
        }
    }

    fn list_files(&mut self) -> Vec<String> {
        let mut files = vec![];

        for i in 0..self.local_storage.length().unwrap() {
            files.push(self.local_storage.key(i).unwrap().unwrap());
        }

        files
    }
}
