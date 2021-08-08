extern crate nix;


mod js;
mod ff;


pub use nix::io::def::OpenMode;
pub use js::{linux::*, js_file::*};
pub use ff::{linux::*, ff_file::*};



// #define EVIOCGMTSLOTS(len)	_IOC(_IOC_READ, 'E', 0x0a, len)

// #define EVIOCGKEY(len)		_IOC(_IOC_READ, 'E', 0x18, len)		/* get global key state */
// #define EVIOCGLED(len)		_IOC(_IOC_READ, 'E', 0x19, len)		/* get all LEDs */
// #define EVIOCGSND(len)		_IOC(_IOC_READ, 'E', 0x1a, len)		/* get all sounds status */
// #define EVIOCGSW(len)		_IOC(_IOC_READ, 'E', 0x1b, len)		/* get all switch states */
// #define EVIOCGBIT(ev,len)	_IOC(_IOC_READ, 'E', 0x20 + (ev), len)	/* get event bits */
// #define EVIOCGABS(abs)		_IOR('E', 0x40 + (abs), struct input_absinfo)	/* get abs value/limits */
// #define EVIOCSABS(abs)		_IOW('E', 0xc0 + (abs), struct input_absinfo)	/* set abs value/limits */
// #define EVIOCSFF		_IOW('E', 0x80, struct ff_effect)	/* send a force effect to a force feedback device */
// #define EVIOCRMFF		_IOW('E', 0x81, int)			/* Erase a force effect */
// #define EVIOCGEFFECTS		_IOR('E', 0x84, int)			/* Report number of effects playable at the same time */
// #define EVIOCGRAB		_IOW('E', 0x90, int)			/* Grab/Release device */
// #define EVIOCREVOKE		_IOW('E', 0x91, int)			/* Revoke device access */



// const EVIOCSFF: env::IoctlNumType = ioc!(
//     env::WRITE,
//     b'E',
//     0x80,
//     core::mem::size_of::<FfEffect>()
// );
// const EVIOCRMFF: env::IoctlNumType = ioc!(
//     env::WRITE,
//     b'E',
//     0x81,
//     core::mem::size_of::<libc::c_int>()
// );

// const EVIOCGEFFECTS: env::IoctlNumType = ioc!(
//     env::READ,
//     b'E',
//     0x84,
//     core::mem::size_of::<libc::c_int>()
// );


// use std::{ffi::CStr, os::unix::prelude::RawFd};
// use ::core::{default::Default, mem};

// pub(crate) mod def;
// pub(crate) mod env;
// pub(crate) mod utils;
// pub mod axis;
// pub mod key;
// pub mod event;
// pub mod js_file;

// use crate::axis::Axis;
// use crate::key::Key;
// use crate::event::Event;

// const JOYSTICK_MAGIC: libc::c_uchar = b'j';

// #[macro_export]
// macro_rules! ioc {
//     ($dir:expr, $ty:expr, $nr:expr, $sz:expr) => (
//         (($dir as env::IoctlNumType & env::DIRMASK) << env::DIRSHIFT) |
//         (($ty as env::IoctlNumType & env::TYPEMASK) << env::TYPESHIFT) |
//         (($nr as env::IoctlNumType & env::NRMASK) << env::NRSHIFT) |
//         (($sz as env::IoctlNumType & env::SIZEMASK) << env::SIZESHIFT))
// }

// const JSIOCGVERSION: env::IoctlNumType = ioc!(env::READ, JOYSTICK_MAGIC, 0x01, mem::size_of::<libc::__u32>());
// const JSIOCGAXES: env::IoctlNumType = ioc!(env::READ, JOYSTICK_MAGIC, 0x11, mem::size_of::<libc::__u8>());
// const JSIOCGBUTTONS: env::IoctlNumType = ioc!(env::READ, JOYSTICK_MAGIC, 0x12, mem::size_of::<libc::__u8>());

// #[macro_export]
// macro_rules! get_buf_req {
//     ($m:expr, $n:expr, $l: expr) => (
//         ioc!(env::READ, $m, $n, $l)
//     )
// }

// const READ_NUM_ERR_MSG: &'static str = "read_number error";
// #[macro_export]
// macro_rules! read_number {
//     ($fd:expr, $req:expr, $t: ty) => {
//         {
//             let mut value: $t = 0;
//             unsafe {
//                 match libc::ioctl($fd, $req, &mut value) {
//                     0 => Ok(value),
//                     _ => Err(READ_NUM_ERR_MSG)
//                 }
//             }
//         }
//     };
// }

// const READ_BUF_ERR_MSG: &'static str = "read_buf error";
// #[macro_export]
// macro_rules! read_buf {
//     ($fd:expr, $req:expr, $buf: expr) => {
//         {
//             unsafe {
//                 match libc::ioctl($fd, $req, $buf) {
//                     len if len > 0 => Ok(len),
//                     _ => Err(READ_BUF_ERR_MSG)
//                 }
//             }
//         }
//     };
// }
// pub fn read_driver_version(fd: RawFd) -> Result<u32, &'static str> {
//     read_number!(fd, JSIOCGVERSION, u32)
// }

// pub fn read_axis_count(fd: RawFd) -> Result<u8, &'static str> {
//     read_number!(fd, JSIOCGAXES, u8)
// }

// pub fn read_button_count(fd: RawFd) -> Result<u8, &'static str> {
//     read_number!(fd, JSIOCGBUTTONS, u8)
// }

// pub fn read_axis_mapping(fd: RawFd, size: usize) -> Result<Vec<Axis>, &'static str> {
//     let mut buf = vec![Axis::default(); size];
//     match read_buf!(fd, get_buf_req!(JOYSTICK_MAGIC, 0x32, mem::size_of::<Axis>() * size), buf.as_mut_ptr()) {
//         Ok(_) => {
//             Ok(buf.to_owned())
//         },
//         Err(msg) => Err(msg)
//     }
// }

// pub fn read_button_mapping(fd: RawFd, size: usize) -> Result<Vec<Key>, &'static str> {
//     let mut buf = vec![Key::default(); size];
//     match read_buf!(fd, get_buf_req!(JOYSTICK_MAGIC, 0x34, mem::size_of::<Key>() * size), buf.as_mut_ptr()) {
//         Ok(_) => {
//             Ok(buf.to_owned())
//         },
//         Err(msg) => Err(msg)
//     }
// }

// pub fn read_name(fd: RawFd) -> Result<String, &'static str> {
//     const BUF_SIZE: usize = 128usize;
//     let mut buf: [libc::c_char; BUF_SIZE] = [0; BUF_SIZE];
//     match read_buf!(fd, get_buf_req!(JOYSTICK_MAGIC, 0x13, mem::size_of::<libc::c_char>() * BUF_SIZE), buf.as_mut_ptr()) {
//         Ok(_) => {
//             Ok(String::from(
//                 unsafe {
//                     CStr::from_ptr(buf.as_ptr()).to_str().unwrap()
//                 }
//             ))
//         },
//         Err(msg) => Err(msg)
//     }
// }

// pub fn test() {
//     use std::os::unix::prelude::{AsRawFd, OpenOptionsExt};

//     let file = std::fs::OpenOptions::new()
//         .read(true)
//         .write(true)
//         .custom_flags(libc::O_NONBLOCK)
//         .open("/dev/input/js0").expect("Error");

//     let fd = file.as_raw_fd();

//     let mut buf = Event::default();

//     while unsafe {
//         libc::read(fd, (&mut buf as *mut _) as *mut libc::c_void, mem::size_of::<Event>()) > 0
//     } {
//         println!("{:?}", buf);
//     }
//     unsafe {
//         libc::close(fd);
//     }

//     let file = std::fs::OpenOptions::new()
//         .read(true)
//         .write(true)
//         // .custom_flags(libc::O_NONBLOCK)
//         .open("/dev/input/js0").expect("Error");

//     let fd = file.as_raw_fd();
//     while unsafe {
//         libc::read(fd, (&mut buf as *mut _) as *mut libc::c_void, mem::size_of::<Event>()) > 0
//     } {
//         println!("{:?}", buf);
//     }
//     unsafe {
//         libc::close(fd);
//     }

// }

// #[macro_export]
// macro_rules! begin_read_event {
//     ($(#[$attr:meta])* $name:ident, $fd:expr) => {
//         loop {
//             $name(
//                 joystick::read_event_with_block($fd)
//             );
//         }
//     };
// }

// pub fn read_init_event_with_no_block<P>(path: P) -> Vec<Event>
// where P: AsRef<std::path::Path> {
//     use std::os::unix::prelude::{AsRawFd, OpenOptionsExt};

//     let file = std::fs::OpenOptions::new()
//         .read(true)
//         .write(true)
//         .custom_flags(libc::O_NONBLOCK)
//         .open(path).expect("Error");

//     let fd = file.as_raw_fd();
//     let mut events = Vec::<Event>::new();
//     let mut event = Event::default();

//     unsafe {
//         while libc::read(fd, (&mut event as *mut _) as *mut libc::c_void, mem::size_of::<Event>()) > 0 {
//             events.push(event.clone());
//         }
//         libc::close(fd);
//     }
//     events
// }

// pub fn read_event_with_block(fd: RawFd) -> Event {
// 	let mut buf = Event::default();
//     unsafe {
//         libc::read(fd, (&mut buf as *mut _) as *mut libc::c_void, mem::size_of::<Event>());
//     }
//     buf
// }
