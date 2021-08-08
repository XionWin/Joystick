use std::{fs::File, mem::size_of, os::unix::prelude::{OpenOptionsExt, RawFd}};

use super::def::OpenMode;

pub fn open(path: &str, mode: OpenMode) -> Result<File, std::io::Error> {
    let mut file_options = std::fs::OpenOptions::new();

    file_options
        .read(mode & OpenMode::READ == OpenMode::READ)
        .write(mode & OpenMode::WRITE == OpenMode::WRITE);
    if mode & OpenMode::NONBLOCK == OpenMode::NONBLOCK {
        file_options.custom_flags(libc::O_NONBLOCK);
    }

    file_options.open(path)
}

pub fn read_with_block<T>(fd: RawFd) -> T
    where T: Default {
    let mut buf = T::default();
    unsafe {
        libc::read(fd, (&mut buf as *mut _) as *mut libc::c_void, size_of::<T>());
    }
    buf
}

pub fn read_with_no_block<T>(fd: RawFd) -> Vec<T>
where T: Default { 
    let mut items = Vec::<T>::new();
    unsafe {
        loop {
            let mut item = T::default();
            if libc::read(fd, (&mut item as *mut _) as *mut libc::c_void, size_of::<T>()) > 0 {
                items.push(item);
            }
            else {
                break;
            }
        }
        libc::close(fd);
    }
    items
}