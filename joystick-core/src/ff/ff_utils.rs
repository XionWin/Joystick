use std::os::unix::prelude::RawFd;

use crate::{EffectType, FfEffect, InputEvent, LinuxEventType, Replay, UEffect, file::def::env, ioc, write_number};

const EVIOCSFF: env::IoctlNumType = ioc!(
    env::WRITE,
    b'E',
    0x80,
    core::mem::size_of::<FfEffect>()
);

pub fn set_effect(fd: RawFd, effect_type: EffectType, effect: UEffect) -> Result<u16, &'static str> {
    let mut ff_effect = FfEffect {
        effect_type,
        id: -1,
        direction: 0,
        trigger: Default::default(),
        replay: Default::default(),
        effect
    };

    unsafe {
        match libc::ioctl(fd, EVIOCSFF, &mut ff_effect) {
            0 => {
                let min_duration = core::time::Duration::from_millis(20);
                let duration = min_duration.as_secs() * 1000 + u64::from(min_duration.subsec_millis());
                let duration = if duration > u64::from(u16::MAX) {
                    u16::MAX
                } else {
                    duration as u16
                };

                let mut ff_effect = FfEffect {
                    effect_type,
                    id: ff_effect.id,
                    direction: 0,
                    trigger: Default::default(),
                    replay: Replay {
                        delay: 0,
                        length: duration,
                    },
                    effect
                };

                match libc::ioctl(fd, EVIOCSFF, &mut ff_effect) {
                    0 => {
                        Ok(ff_effect.id as u16)
                    }
                    _ => Err("Register effect error")
                }
            }
            _ => {
                Err("Register effect type error")
            }
        }
    }

}

pub fn run_effect(fd: RawFd, id: u16) -> bool {
    let time = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let ev = InputEvent {
        event_type: LinuxEventType::ForceFeedback,
        id,
        value: 10,
        time,
    };

    let size = core::mem::size_of::<InputEvent>();
    let s = unsafe { std::slice::from_raw_parts(&ev as *const _ as *const u8, size) };

    unsafe {
        libc::write(fd, (s as *const _) as *const libc::c_void, size) == 0
    }
}

const EVIOCRMFF: env::IoctlNumType = ioc!(
    env::WRITE,
    b'E',
    0x81,
    core::mem::size_of::<libc::c_int>()
);

pub fn remove_effect(fd: RawFd, id: u16) -> bool {
    write_number!(fd, EVIOCRMFF, id as libc::c_int)
}