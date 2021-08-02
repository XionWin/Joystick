use std::{mem::size_of, os::unix::prelude::RawFd};

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


pub fn read_init_event_with_no_block<T>(fd: RawFd) -> Vec<T>
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

pub fn read_event_with_block<T>(fd: RawFd) -> T
    where T: Default {
	let mut buf = T::default();
    unsafe {
        libc::read(fd, (&mut buf as *mut _) as *mut libc::c_void, size_of::<T>());
    }
    buf
}