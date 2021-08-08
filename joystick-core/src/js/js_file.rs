use std::{os::unix::prelude::RawFd};

use super::linux::{Axis, Event, Key};

use super::js_utils;
use nix::io::{def::OpenMode, File};

#[derive(Debug)]
pub struct JsFile {
    file: File
}

impl JsFile {
    pub fn new(path: &str) -> Self  {
        JsFile {
            file: File::new(path)
        }
    }

    fn get_fd(&self) -> RawFd {
        self.file.get_fd()
    }

    pub fn is_connected(&self) -> bool {
        self.file.is_connected()
    }
    
    pub fn open(mut self, mode: OpenMode) -> Self {
        self.file = self.file.open(mode);
        self
    }

    pub fn close(&mut self) {
        self.file.close();
    }

    pub fn switch(self, mode: OpenMode) {
        self.file.switch(mode);
    }

    pub fn read_driver_version(&self) -> Result<u32, &'static str> {
        js_utils::read_driver_version(self.get_fd())
    }
    
    pub fn read_axis_count(&self) -> Result<u8, &'static str> {
        js_utils::read_axis_count(self.get_fd())
    }
    
    pub fn read_button_count(&self) -> Result<u8, &'static str> {
        js_utils::read_button_count(self.get_fd())
    }

    pub fn read_axis_mapping(&self, size: usize) -> Result<Vec<Axis>, &'static str> {
        js_utils::read_axis_mapping(self.get_fd(), size)
    }

    pub fn read_button_mapping(&self, size: usize) -> Result<Vec<Key>, &'static str> {
        js_utils::read_button_mapping(self.get_fd(), size)
    }

    pub fn read_name(&self) -> Result<String, &'static str> {
        js_utils::read_name(self.get_fd())
    }

    pub fn read_event_with_block(&self) -> Event {
        self.file.read_with_block()
    }

    pub fn read_init_event_with_no_block(&self) -> Vec<Event> {
        self.file.read_with_no_block()
    }
}
