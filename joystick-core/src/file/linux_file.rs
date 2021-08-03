use std::{fs::File, mem::size_of, os::unix::prelude::{AsRawFd, OpenOptionsExt, RawFd}};

#[macro_export]
macro_rules! ioc {
    ($dir:expr, $ty:expr, $nr:expr, $sz:expr) => (
        (($dir as env::IoctlNumType & env::DIRMASK) << env::DIRSHIFT) |
        (($ty as env::IoctlNumType & env::TYPEMASK) << env::TYPESHIFT) |
        (($nr as env::IoctlNumType & env::NRMASK) << env::NRSHIFT) |
        (($sz as env::IoctlNumType & env::SIZEMASK) << env::SIZESHIFT))
}

#[macro_export]
macro_rules! get_buf_req {
    ($m:expr, $n:expr, $l: expr) => (
        ioc!(env::READ, $m, $n, $l)
    )
}

#[macro_export]
macro_rules! read_number {
    ($fd:expr, $req:expr, $t: ty) => {
        {
            let mut value: $t = 0;
            unsafe {
                match libc::ioctl($fd, $req, &mut value) {
                    0 => Ok(value),
                    _ => Err("read_number error")
                }
            }
        }
    };
}

#[macro_export]
macro_rules! write_number {
    ($fd:expr, $req:expr, $v: expr) => {
        {
            unsafe {
                match libc::ioctl($fd, $req, $v) {
                    0 => true,
                    _ => false
                }
            }
        }
    };
}

#[macro_export]
macro_rules! read_buf {
    ($fd:expr, $req:expr, $buf: expr) => {
        {
            unsafe {
                match libc::ioctl($fd, $req, $buf) {
                    len if len > 0 => Ok(len),
                    _ => Err("read_buf error")
                }
            }
        }
    };
}

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
pub struct LinuxFile {
    path: String,
    file: Option<File>
}

impl LinuxFile {
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


    pub fn read_init_event_with_no_block<T>(&self) -> Vec<T>
    where T: Default { 
        let mut items = Vec::<T>::new();
        unsafe {
            loop {
                let mut item = T::default();
                if libc::read(self.get_fd(), (&mut item as *mut _) as *mut libc::c_void, size_of::<T>()) > 0 {
                    items.push(item);
                }
                else {
                    break;
                }
            }
            libc::close(self.get_fd());
        }
        items
    }

    pub fn read_event_with_block<T>(&self) -> T
        where T: Default {
        let mut buf = T::default();
        unsafe {
            libc::read(self.get_fd(), (&mut buf as *mut _) as *mut libc::c_void, size_of::<T>());
        }
        buf
    }
    
}



