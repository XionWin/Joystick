extern crate joystick;

use joystick::TForceFeedback;

fn main() {
    let mut gamepad = joystick::Gamepad::new("/dev/input/js0");

    gamepad.register_force_feedback("/dev/input/event2");
    println!("{:?}", &gamepad);

    let mut ff_id = Option::<u16>::None;
    let mut read_event = |gamepad: &mut joystick::Gamepad, event: joystick::JsEvent| {
        println!("{:?}", &event);

        if event.get_event_type() == joystick::EventType::Button && event.get_alias() == "ButtonWest" && event.get_value() == 1 {
            match ff_id {
                Some(id) => println!("Error: id has value, value is {}", id),
                None => ff_id = Some(gamepad.set_rumble_effect().unwrap())
            }
        }


        if event.get_event_type() == joystick::EventType::Button && event.get_alias() == "ButtonEast" && event.get_value() == 1 {
            match ff_id {
                Some(id) => {
                    gamepad.remove_effect(id);
                    ff_id = Option::None;
                },
                None => println!("Error: id has no value")
            }
        }


        if event.get_event_type() == joystick::EventType::Button && event.get_alias() == "ButtonSouth" && event.get_value() == 1 {
            match ff_id {
                Some(id) => {
                    gamepad.run_effect(id);
                },
                None => println!("Error: id has no value")
            }
        }
    };

    joystick::begin_read!(read_event, &mut gamepad);


    // let file = OpenOptions::new()
    //     .read(true)
    //     .write(true)
    //     // .custom_flags(libc::O_NONBLOCK)
    //     .open("/dev/input/js0").expect("Error");

    // let fd = file.as_raw_fd();

    // let v = joystick::utils::read_driver_version(fd).unwrap();
    // println!("{:#X}", v);
    // println!("{:?}", pad.get_name());

    // test!(joystick::read_driver_version, fd, "{:#X}");
    // test!(joystick::read_axis_count, fd, "{}");
    // test!(joystick::read_button_count, fd, "{}");
    // test!(joystick::read_name, fd, "{:?}");
    // test!(joystick::read_axis_mapping, fd, joystick::read_axis_count(fd).unwrap() as usize, "{:?}");
    // test!(joystick::read_button_mapping, fd, joystick::read_button_count(fd).unwrap() as usize, "{:?}");
    
    // joystick::begin_read_event!(read_event, fd);
}

// fn read_event(event: joystick::JsEvent) {
//     println!("{:?}", &event);
// }

#[macro_export]
macro_rules! test {
    ($p: path, $fd: expr, $fmt: expr) => {
        match $p($fd) {
            Ok(v) => {
                print!("{}: ", stringify!($p));
                println!($fmt, v);
            }
            Err(msg) => println!("{}", msg)
        }
    };
    ($p: path, $fd: expr, $pr:expr, $fmt: expr) => {
        match $p($fd, $pr) {
            Ok(v) => {
                print!("{}: ", stringify!($p));
                println!($fmt, v);
            }
            Err(msg) => println!("{}", msg)
        }
    };
}