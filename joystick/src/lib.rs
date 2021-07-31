extern crate bitflags;
extern crate joystick_core;

mod control;
mod axis;
mod button;
mod gamepad;
mod utils;
mod js_event;

pub use gamepad::*;
pub use js_event::*;

pub fn test(fd: std::os::unix::prelude::RawFd) {
    joystick_core::test(fd);
}
