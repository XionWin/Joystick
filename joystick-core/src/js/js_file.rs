use std::{fs::File, os::unix::prelude::{AsRawFd, OpenOptionsExt, RawFd}};
use super::linux::{Axis, Event, Key};

use super::js_utils;
use super::super::file::linux_file;

bitflags! {
    pub struct OpenMode: u8 {
        const NONE = 0b0000;
        const READ = 0b0001;
        const WRITE = 0b0010;
        const NONBLOCK = 0b1000;
    }
}

impl OpenMode {
    pub fn read() -> OpenMode {
        OpenMode::READ as OpenMode
    }
    pub fn write() -> OpenMode {
        OpenMode::WRITE as OpenMode
    }
    pub fn nonblock() -> OpenMode {
        OpenMode::NONBLOCK as OpenMode
    }
}

#[derive(Debug)]
pub struct JsFile {
    path: String,
    file: Option<File>
}

impl JsFile {
    pub fn new(path: &str) -> Self  {
        JsFile {
            path: String::from(path),
            file: Option::None
        }
    }

    fn fd(&self) -> RawFd {
        match &self.file {
            Some(f) => f.as_raw_fd(),
            None => -1
        }
    }

    pub fn is_connected(&self) -> bool {
        match self.file {
            Some(_) => true,
            None => false
        }
    }
    
    pub fn open(mut self, mode: OpenMode) -> Self {
        let mut file_options = std::fs::OpenOptions::new();

        file_options
            .read(mode & OpenMode::READ == OpenMode::READ)
            .write(mode & OpenMode::WRITE == OpenMode::WRITE);
        if mode & OpenMode::NONBLOCK == OpenMode::NONBLOCK {
            file_options.custom_flags(libc::O_NONBLOCK);
        }

        self.file = file_options.open(&self.path).ok();
        self
    }

    pub fn close(&mut self) {
        unsafe {
            libc::close(self.fd());
            self.file = Option::None
        }
    }

    pub fn switch(mut self, mode: OpenMode) {
        match &self.file {
            Some(_) => {
                self.close();
            }
            None => {}
        }
        self.open(mode);
    }

    pub fn read_driver_version(&self) -> Result<u32, &'static str> {
        js_utils::read_driver_version(self.fd())
    }
    
    pub fn read_axis_count(&self) -> Result<u8, &'static str> {
        js_utils::read_axis_count(self.fd())
    }
    
    pub fn read_button_count(&self) -> Result<u8, &'static str> {
        js_utils::read_button_count(self.fd())
    }

    pub fn read_axis_mapping(&self, size: usize) -> Result<Vec<Axis>, &'static str> {
        js_utils::read_axis_mapping(self.fd(), size)
    }

    pub fn read_button_mapping(&self, size: usize) -> Result<Vec<Key>, &'static str> {
        js_utils::read_button_mapping(self.fd(), size)
    }

    pub fn read_name(&self) -> Result<String, &'static str> {
        js_utils::read_name(self.fd())
    }

    pub fn read_event_with_block(&self) -> Event {
        linux_file::read_event_with_block(self.fd())
    }

    pub fn read_init_event_with_no_block(&self) -> Vec<Event> {
        linux_file::read_init_event_with_no_block(self.fd())
    }
}
