use std::{os::unix::prelude::{AsRawFd, RawFd}};

use super::def::OpenMode;

#[derive(Debug)]
pub struct File {
    path: String,
    file: Option<std::fs::File>
}

impl File {
    pub fn new(path: &str) -> Self  {
        Self {
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
    
    pub fn open(mut self, mode: OpenMode) -> Self {
        self.file = super::utils::open(&self.path, mode).ok();
        self
    }

    pub fn close(&mut self) {
        unsafe {
            libc::close(self.get_fd());
            self.file = Option::None;
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


    pub fn read_with_no_block<T>(&self) -> Vec<T>
    where T: Default { 
        super::utils::read_with_no_block(self.get_fd())
    }

    pub fn read_with_block<T>(&self) -> T
    where T: Default {
        super::utils::read_with_block(self.get_fd())
    }
}