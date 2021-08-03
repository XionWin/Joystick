extern crate joystick;

fn main() {
    // let mut gamepad = joystick::Gamepad::new("/dev/input/js0");
    // println!("{:?}", &gamepad);

    // let read_event = |event: joystick::JsEvent| {
    //     println!("{:?}", &event);
    // };

    // joystick::begin_read!(read_event, &mut gamepad);


    joystick::test("/dev/input/event2");

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