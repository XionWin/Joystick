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

pub fn test(path: &str) {
    let file = joystick_core::FfFile::new(path);

    let file = file.open(joystick_core::OpenMode::READ | joystick_core::OpenMode::WRITE);
    
    match file.set_rumble_effect() {
        Ok(id) => {
            loop {
                file.run_effect(id);
                std::thread::sleep(core::time::Duration::from_secs(2));
            }
        }
        Err(_) => println!("test error")
    }
}
