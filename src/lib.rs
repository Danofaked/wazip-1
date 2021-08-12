use js_sys::{Date as JsDate, Error as JsError};
use std::{
    convert::TryInto,
    io::{Cursor, Write},
};
use wasm_bindgen::prelude::*;
use zip::write::FileOptions as ZipFileOptions;

type ZipWriterImpl = zip::ZipWriter<Cursor<Vec<u8>>>;

#[wasm_bindgen]
pub struct ZipWriter {
    inner: ZipWriterImpl,
}

#[wasm_bindgen]
extern "C" {
    pub type FileOptions;

    #[wasm_bindgen(method, getter)]
    pub fn last_modified(this: &FileOptions) -> Option<JsDate>;
}

#[wasm_bindgen]
impl ZipWriter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        ZipWriter {
            inner: ZipWriterImpl::new(Default::default()),
        }
    }

    /// Creates a new file in the archive.
    ///
    /// Subsequent calls to `write` will write to this file.
    pub fn start_file(&mut self, name: &str, options: Option<FileOptions>) -> Result<(), JsValue> {
        let options = if let Some(options) = options {
            let mut zip_options = ZipFileOptions::default();

            if let Some(date) = options.last_modified() {
                let year: u16 = date.get_full_year().try_into().expect_throw("Invalid year");
                let month: u8 = (date.get_month() + 1)
                    .try_into()
                    .expect_throw("Invalid month");
                let day: u8 = date.get_date().try_into().expect_throw("Invalid day");
                let hours: u8 = date.get_hours().try_into().expect_throw("Invalid hours");
                let minutes: u8 = date
                    .get_minutes()
                    .try_into()
                    .expect_throw("Invalid minutes");
                let seconds: u8 = date
                    .get_seconds()
                    .try_into()
                    .expect_throw("Invalid seconds");

                zip_options = zip_options.last_modified_time(
                    zip::DateTime::from_date_and_time(year, month, day, hours, minutes, seconds)
                        .expect_throw("Invalid date"),
                );
            }

            zip_options
        } else {
            ZipFileOptions::default()
        };

        self.inner
            .start_file(name, options)
            .map_err(|e| JsError::new(&e.to_string()).into())
    }

    /// Writes some data to the current file.
    pub fn write(&mut self, data: &[u8]) -> Result<(), JsValue> {
        self.inner
            .write_all(data)
            .map_err(|e| JsError::new(&e.to_string()).into())
    }

    /// Sets the ZIP file comment.
    pub fn set_comment(&mut self, comment: String) {
        self.inner.set_comment(comment)
    }

    /// Closes the ZipWriter and returns the generated binary data (the zip file) as a `Uint8Array`.
    pub fn finish(mut self) -> Result<Vec<u8>, JsValue> {
        self.inner
            .finish()
            .map(Cursor::into_inner)
            .map_err(|e| JsError::new(&e.to_string()).into())
    }
}
