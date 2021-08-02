use std::{ffi::CStr, os::unix::prelude::RawFd};
use ::core::{default::Default, mem};


use super::super::file::def::env;
use super::linux::{Key, Axis, Event};

use crate::{ioc, read_number, get_buf_req, read_buf};

const JOYSTICK_MAGIC: libc::c_uchar = b'j';
const JSIOCGVERSION: env::IoctlNumType = ioc!(env::READ, JOYSTICK_MAGIC, 0x01, mem::size_of::<libc::__u32>());
const JSIOCGAXES: env::IoctlNumType = ioc!(env::READ, JOYSTICK_MAGIC, 0x11, mem::size_of::<libc::__u8>());
const JSIOCGBUTTONS: env::IoctlNumType = ioc!(env::READ, JOYSTICK_MAGIC, 0x12, mem::size_of::<libc::__u8>());

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

