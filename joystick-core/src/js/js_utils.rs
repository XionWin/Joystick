use std::{ffi::CStr, os::unix::prelude::RawFd};
use ::core::{default::Default, mem};

use super::def::env;
use super::linux::{Key, Axis, Event};

const JOYSTICK_MAGIC: libc::c_uchar = b'j';

macro_rules! ioc {
    ($dir:expr, $ty:expr, $nr:expr, $sz:expr) => (
        (($dir as env::IoctlNumType & env::DIRMASK) << env::DIRSHIFT) |
        (($ty as env::IoctlNumType & env::TYPEMASK) << env::TYPESHIFT) |
        (($nr as env::IoctlNumType & env::NRMASK) << env::NRSHIFT) |
        (($sz as env::IoctlNumType & env::SIZEMASK) << env::SIZESHIFT))
}

const JSIOCGVERSION: env::IoctlNumType = ioc!(env::READ, JOYSTICK_MAGIC, 0x01, mem::size_of::<libc::__u32>());
const JSIOCGAXES: env::IoctlNumType = ioc!(env::READ, JOYSTICK_MAGIC, 0x11, mem::size_of::<libc::__u8>());
const JSIOCGBUTTONS: env::IoctlNumType = ioc!(env::READ, JOYSTICK_MAGIC, 0x12, mem::size_of::<libc::__u8>());

macro_rules! get_buf_req {
    ($m:expr, $n:expr, $l: expr) => (
        ioc!(env::READ, $m, $n, $l)
    )
}

const READ_NUM_ERR_MSG: &'static str = "read_number error";
macro_rules! read_number {
    ($fd:expr, $req:expr, $t: ty) => {
        {
            let mut value: $t = 0;
            unsafe {
                match libc::ioctl($fd, $req, &mut value) {
                    0 => Ok(value),
                    _ => Err(READ_NUM_ERR_MSG)
                }
            }
        }
    };
}

const READ_BUF_ERR_MSG: &'static str = "read_buf error";
macro_rules! read_buf {
    ($fd:expr, $req:expr, $buf: expr) => {
        {
            unsafe {
                match libc::ioctl($fd, $req, $buf) {
                    len if len > 0 => Ok(len),
                    _ => Err(READ_BUF_ERR_MSG)
                }
            }
        }
    };
}
pub fn read_driver_version(fd: RawFd) -> Result<u32, &'static str> {
    read_number!(fd, JSIOCGVERSION, u32)
}

pub fn read_axis_count(fd: RawFd) -> Result<u8, &'static str> {
    read_number!(fd, JSIOCGAXES, u8)
}

pub fn read_button_count(fd: RawFd) -> Result<u8, &'static str> {
    read_number!(fd, JSIOCGBUTTONS, u8)
}


pub fn read_axis_mapping(fd: RawFd, size: usize) -> Result<Vec<Axis>, &'static str> {
    let mut buf = vec![Axis::default(); size];
    match read_buf!(fd, get_buf_req!(JOYSTICK_MAGIC, 0x32, mem::size_of::<Axis>() * size), buf.as_mut_ptr()) {
        Ok(_) => {
            Ok(buf.to_owned())
        },
        Err(msg) => Err(msg)
    }
}

pub fn read_button_mapping(fd: RawFd, size: usize) -> Result<Vec<Key>, &'static str> {
    let mut buf = vec![Key::default(); size];
    match read_buf!(fd, get_buf_req!(JOYSTICK_MAGIC, 0x34, mem::size_of::<Key>() * size), buf.as_mut_ptr()) {
        Ok(_) => {
            Ok(buf.to_owned())
        },
        Err(msg) => Err(msg)
    }
}

pub fn read_name(fd: RawFd) -> Result<String, &'static str> {
    const BUF_SIZE: usize = 128usize;
    let mut buf: [libc::c_char; BUF_SIZE] = [0; BUF_SIZE];
    match read_buf!(fd, get_buf_req!(JOYSTICK_MAGIC, 0x13, mem::size_of::<libc::c_char>() * BUF_SIZE), buf.as_mut_ptr()) {
        Ok(_) => {
            Ok(String::from(
                unsafe {
                    CStr::from_ptr(buf.as_ptr()).to_str().unwrap()
                }
            ))
        },
        Err(msg) => Err(msg)
    }
}

pub fn read_init_event_with_no_block(fd: RawFd) -> Vec<Event> {
    let mut events = Vec::<Event>::new();
    let mut event = Event::default();
    unsafe {
        while libc::read(fd, (&mut event as *mut _) as *mut libc::c_void, mem::size_of::<Event>()) > 0 {
            events.push(event.clone());
        }
        libc::close(fd);
    }
    events
}

pub fn read_event_with_block(fd: RawFd) -> Event {
	let mut buf = Event::default();
    unsafe {
        libc::read(fd, (&mut buf as *mut _) as *mut libc::c_void, mem::size_of::<Event>());
    }
    buf
}
