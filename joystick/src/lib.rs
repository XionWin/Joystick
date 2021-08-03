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

            let mut counter = 0;
            loop {
                file.run_effect(id);
                counter += 1;
                println!("counter: {}", counter);
                std::thread::sleep(core::time::Duration::from_secs(1));

                if counter % 3 == 0 {
                    let rr = file.remove_effect(id);
                    println!("rr: {:?}", rr);
                    let r = file.set_rumble_effect();

                    println!("r: {:?}", r);
                }
            }
        }
        Err(_) => println!("test error")
    }
}
