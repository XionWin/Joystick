use std::{fs::File, os::unix::prelude::{AsRawFd, RawFd}};


#[derive(Debug)]
pub struct FfFile {
    path: String,
    file: Option<File>
}

impl FfFile {
    pub fn new(path: &str) -> Self  {
        FfFile {
            path: String::from(path),
            file: Option::None
        }
    }

    pub fn get_fd(&self) -> RawFd {
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
    
    // pub fn open(mut self, mode: OpenMode) -> Self {
    //     let mut file_options = std::fs::OpenOptions::new();

    //     file_options
    //         .read(mode & OpenMode::READ == OpenMode::READ)
    //         .write(mode & OpenMode::WRITE == OpenMode::WRITE);
    //     if mode & OpenMode::NONBLOCK == OpenMode::NONBLOCK {
    //         file_options.custom_flags(libc::O_NONBLOCK);
    //     }

    //     self.file = file_options.open(&self.path).ok();
    //     self
    // }

    // pub fn close(&mut self) {
    //     unsafe {
    //         libc::close(self.fd());
    //         self.file = Option::None
    //     }
    // }

    // pub fn switch(mut self, mode: OpenMode) {
    //     match &self.file {
    //         Some(_) => {
    //             self.close();
    //         }
    //         None => {}
    //     }
    //     self.open(mode);
    // }

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