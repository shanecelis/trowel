use crate::{WriteMode, fs::FileSys};
use web_sys::window;
use wasm_bindgen::JsValue;
use genio::{Read, Write};
use alloc::rc::Rc;

pub struct WebFS {
    local_storage: Rc<web_sys::Storage>,
}

impl WebFS {
    pub fn new() -> Result<Self, JsValue> {
        Ok(Self {
            local_storage: Rc::new(
                window().ok_or_else(|| JsValue::from_str("Cannot get window"))?
                        .local_storage()?
                        .ok_or_else(|| JsValue::from_str("Cannot get storage"))?),
        })
    }
}

pub struct WebFile {
    name: String,
    mode: WriteMode,
    data: Option<Vec<u8>>,
    start: Option<usize>,
    local_storage: Rc<web_sys::Storage>
}

impl WebFile {
    fn new(name: String, mode: WriteMode, local_storage: Rc<web_sys::Storage>) -> Self {
        Self {
            name,
            mode,
            data: None,
            start: None,
            local_storage
        }
    }
}


impl Read for WebFile {
    type ReadError = JsValue;
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::ReadError> {
        if self.data.is_none() {
            let bytes = self.local_storage.get_item(&self.name)?
                                          .unwrap_or_else(|| String::new())
                                          .into_bytes();
            self.data = Some(bytes);
        }
        let slice: &[u8] = self.data.as_ref().expect("Could not get data");
        let i: usize = *self.start.get_or_insert(0);
        let len = buf.len().min(slice.len() - i);
        buf[0..len].copy_from_slice(&slice[i..i + len]);
        self.start.replace(i + len);
        Ok(len)
    }
}

impl Write for WebFile {
    type WriteError = JsValue;
    type FlushError = JsValue;
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::WriteError> {
        if self.mode == WriteMode::ReadOnly {
            return Err(JsValue::from_str("Cannot write to read only file"));
        }
        if self.data.is_none() {
            let string = match self.mode {
                WriteMode::Truncate => String::new(),
                WriteMode::Append => self.local_storage
                                         .get_item(&self.name)?
                                         .unwrap_or_else(|| String::new()),
                WriteMode::ReadOnly => String::new()
            };
            let bytes = string.into_bytes();
            self.data = Some(bytes);
        }

        let slice: &mut Vec<u8> = self.data.as_mut().expect("Could not get data");
        slice.extend_from_slice(buf);
        let len = buf.len();
        Ok(len)
    }

    fn flush(&mut self) -> Result<(), Self::FlushError> {
        // There doesn't appear to be any flush possible.
        if let Some(v) = self.data.as_ref() {
            let str = String::from_utf8(v.clone())
                .map_err(|_| JsValue::from_str("File contents not valid utf-8 string"))?;
            self.local_storage
                .set_item(&self.name, &str)?
        }
        Ok(())
    }

    fn size_hint(&mut self, _bytes: usize) {
    }

    fn uses_size_hint(&self) -> bool {
        false
    }
}

impl Drop for WebFile {
    fn drop(&mut self) {
        self.flush().expect("Unable to flush file");
    }
}

impl FileSys for WebFS {
    type FileError = JsValue;
    type File = WebFile;

    fn file_exists(&mut self, name: &str) -> Result<bool, Self::FileError> {
        Ok(self.local_storage.get_item(name)?.is_some())
    }
    fn open_file(&mut self, name: &str, mode: WriteMode) -> Result<Self::File, Self::FileError> {
        if mode == WriteMode::ReadOnly && ! self.file_exists(name)? {
            Err(JsValue::from_str("No such file"))
        } else {
            Ok(WebFile::new(name.to_owned(), mode, self.local_storage.clone()))
        }
    }
    fn delete_file(&mut self, name: &str) -> Result<(), Self::FileError> {
        self.local_storage.remove_item(name)?;
        Ok(())
    }
    fn list_files(&mut self) -> Result<alloc::vec::Vec<String>, Self::FileError> {

        let mut files = vec![];

        for i in 0..self.local_storage.length()? {
            files.push(self.local_storage.key(i)?.ok_or_else(|| JsValue::from_str("Cannot get file/key name"))?);
        }

        Ok(files)
    }
}

