#[macro_use]
extern crate bitflags;

pub(crate) mod def;
pub mod linux;
mod js_file;
pub mod utils;

use std::{os::{unix::prelude::RawFd}};

use linux::{env, ff};

pub use js_file::*;

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

macro_rules! ioc {
    ($dir:expr, $ty:expr, $nr:expr, $sz:expr) => {
        (($dir as env::IoctlNumType & env::DIRMASK) << env::DIRSHIFT)
            | (($ty as env::IoctlNumType & env::TYPEMASK) << env::TYPESHIFT)
            | (($nr as env::IoctlNumType & env::NRMASK) << env::NRSHIFT)
            | (($sz as env::IoctlNumType & env::SIZEMASK) << env::SIZESHIFT)
    };
}

const EV_FF: u16 = 0x15;
const FF_RUMBLE: u16 = 0x50;
const FF_PERIOD: u16 = 0x51;
const FF_CONSTA: u16 = 0x52;
const FF_SPRING: u16 = 0x53;
const FF_FRICTI: u16 = 0x54;
const FF_DAMPER: u16 = 0x55;
const FF_INERTI: u16 = 0x56;
const FF_RAMP: u16 = 0x57;


const EVIOCSFF: env::IoctlNumType = ioc!(
    env::consts::WRITE,
    b'E',
    0x80,
    core::mem::size_of::<ff::FfEffect>()
);
const EVIOCRMFF: env::IoctlNumType = ioc!(
    env::consts::WRITE,
    b'E',
    0x81,
    core::mem::size_of::<libc::c_int>()
);

const EVIOCGEFFECTS: env::IoctlNumType = ioc!(
    env::consts::READ,
    b'E',
    0x84,
    core::mem::size_of::<libc::c_int>()
);

pub fn test(fd: std::os::unix::prelude::RawFd) {
    // let effect_types = vec![FF_PERIOD, FF_CONSTA, FF_RUMBLE, FF_SPRING, FF_FRICTI,
    //     FF_DAMPER, FF_INERTI, FF_RAMP, FF_SQUARE, FF_TRIANG,
    //     FF_SINE, FF_SAW_UP, FF_SAW_DO, FF_CUSTOM, FF_GAIN, FF_AUTOCE];

    let effect_ids = vec![upload_periodic_effect(fd), upload_rumble_effect(fd)];
    
    loop {
        for effect_id in &effect_ids {
            run(fd, *effect_id);
        }
    }
}

pub fn upload_periodic_effect(fd: RawFd) -> i16 {
    let effect_type = FF_PERIOD;
    let mut effect = ff::FfEffect {
        type_: effect_type,
        id: -1,
        direction: 0,
        trigger: Default::default(),
        replay: Default::default(),
        effect:  ff::UEffect {
            periodic: Default::default()
        }
    };

    let result: i32;
    let effect_id: i16;
    unsafe {
        result = libc::ioctl(fd, EVIOCSFF, &mut effect);
        effect_id = effect.id;
        println!("upload id: {}, result: {:?}", effect_id, result);
    }

    if result == 0 {
        let min_duration = core::time::Duration::from_millis(20);
        let duration = min_duration.as_secs() * 1000 + u64::from(min_duration.subsec_millis());
        let duration = if duration > u64::from(u16::MAX) {
            u16::MAX
        } else {
            duration as u16
        };

        let mut effect = ff::FfEffect {
            type_: effect_type,
            id: effect_id,
            direction: 0,
            trigger: Default::default(),
            replay: ff::Replay {
                delay: 0,
                length: duration,
            },
            effect: ff::UEffect {
                periodic: ff::PeriodicEffect {
                    waveform: ff::WaveForm::FF_SINE,
                    period: 100,	/* 0.1 second */
                    magnitude: 0x7fff,	/* 0.5 * Maximum magnitude */
                    offset: 0,
                    phase: 0,
                
                    envelope: ff::Envelope {
                        attack_length: 1000,
                        attack_level: 0x00ff,
                        fade_length: 1000,
                        fade_level: 0x00ff,
                    },
                
                    custom_len: 0,
                    custom_data: unsafe {
                        core::mem::zeroed()
                    },
                }
            }
        };

        unsafe {
            let r = libc::ioctl(fd, EVIOCSFF, &mut effect);
            println!("{:?}", r);
        }
    }
    effect_id
}

pub fn upload_rumble_effect(fd: RawFd) -> i16 {
    let effect_type = FF_RUMBLE;
    let mut effect = ff::FfEffect {
        type_: effect_type,
        id: -1,
        direction: 0,
        trigger: Default::default(),
        replay: Default::default(),
        effect:  ff::UEffect {
            rumble: ff::RumbleEffect {
                strong_magnitude: 0,
                weak_magnitude: 0,
            }
        },
    };

    let result: i32;
    let effect_id: i16;
    unsafe {
        result = libc::ioctl(fd, EVIOCSFF, &mut effect);
        effect_id = effect.id;
        println!("upload id: {}, result: {:?}", effect_id, result);
    }

    if result == 0 {
        let min_duration = core::time::Duration::from_millis(20);
        let duration = min_duration.as_secs() * 1000 + u64::from(min_duration.subsec_millis());
        let duration = if duration > u64::from(u16::MAX) {
            u16::MAX
        } else {
            duration as u16
        };

        let mut effect = ff::FfEffect {
            type_: effect_type,
            id: effect_id,
            direction: 0,
            trigger: Default::default(),
            replay: ff::Replay {
                delay: 0,
                length: duration,
            },
            effect: ff::UEffect {
                rumble: ff::RumbleEffect {
                    strong_magnitude: 0x8000,
                    weak_magnitude: 0,
                }
            },
        };

        unsafe {
            let r = libc::ioctl(fd, EVIOCSFF, &mut effect);
            println!("{:?}", r);
        }
    }
    effect_id
}

pub fn run (fd: RawFd, effect_id: i16) {
    let time = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let ev = ff::InputEvent {
        type_: EV_FF,
        code: effect_id as u16,
        value: 10,
        time,
    };

    let size = core::mem::size_of::<ff::InputEvent>();
    let s = unsafe { std::slice::from_raw_parts(&ev as *const _ as *const u8, size) };

    unsafe {
        let r = libc::write(fd, (s as *const _) as *const libc::c_void, size);
        println!("{:?}", r);
    }
    std::thread::sleep(core::time::Duration::from_secs(2));
}



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

// const JSIOCGVERSION: env::IoctlNumType = ioc!(env::consts::READ, JOYSTICK_MAGIC, 0x01, mem::size_of::<libc::__u32>());
// const JSIOCGAXES: env::IoctlNumType = ioc!(env::consts::READ, JOYSTICK_MAGIC, 0x11, mem::size_of::<libc::__u8>());
// const JSIOCGBUTTONS: env::IoctlNumType = ioc!(env::consts::READ, JOYSTICK_MAGIC, 0x12, mem::size_of::<libc::__u8>());

// #[macro_export]
// macro_rules! get_buf_req {
//     ($m:expr, $n:expr, $l: expr) => (
//         ioc!(env::consts::READ, $m, $n, $l)
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
