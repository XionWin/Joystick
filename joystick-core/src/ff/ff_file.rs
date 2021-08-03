use std::os::unix::prelude::RawFd;

use crate::file::linux_file::{OpenMode, LinuxFile};


#[derive(Debug)]
pub struct FfFile {
    file: LinuxFile
}

impl FfFile {
    pub fn new(path: &str) -> Self  {
        FfFile {
            file: LinuxFile::new(path)
        }
    }

    pub fn get_fd(&self) -> RawFd {
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

    // pub fn read_driver_version(&self) -> Result<u32, &'static str> {
    //     js_utils::read_driver_version(self.fd())
    // }
    
    // pub fn read_axis_count(&self) -> Result<u8, &'static str> {
    //     js_utils::read_axis_count(self.fd())
    // }
    
    // pub fn read_button_count(&self) -> Result<u8, &'static str> {
    //     js_utils::read_button_count(self.fd())
    // }

    // pub fn read_axis_mapping(&self, size: usize) -> Result<Vec<Axis>, &'static str> {
    //     js_utils::read_axis_mapping(self.fd(), size)
    // }

    // pub fn read_button_mapping(&self, size: usize) -> Result<Vec<Key>, &'static str> {
    //     js_utils::read_button_mapping(self.fd(), size)
    // }

    // pub fn read_name(&self) -> Result<String, &'static str> {
    //     js_utils::read_name(self.fd())
    // }

    // pub fn read_event_with_block(&self) -> Event {
    //     js_utils::read_event_with_block(self.fd())
    // }

    // pub fn read_init_event_with_no_block(&self) -> Vec<Event> {
    //     js_utils::read_init_event_with_no_block(self.fd())
    // }
}