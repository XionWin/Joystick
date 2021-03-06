use crate::JsEvent;

use super::{axis::Axis, button::Button, utils};
use joystick_core as jsc;
use jsc::{FfFile, JsFile, OpenMode};
use crate::TForceFeedback;

#[derive(Debug)]
pub struct Gamepad {
    path: String,
    open_mode: OpenMode,
    file: Option<JsFile>,

    version: u32,
    name: String,

    buttons: Vec<Button>,
    axes: Vec<Axis>,

    ff_file: Option<FfFile>
}

impl Default for Gamepad {
    fn default() -> Self {
        Gamepad {
            path: String::from(""),
            open_mode: OpenMode::NONE,
            file: Option::None,
            version: 0,
            name: String::from(""),
            buttons: Vec::<Button>::new(),
            axes: Vec::<Axis>::new(),
            ff_file: Option::None
        }
    }
}

impl Gamepad {
    pub fn new(path: &str) -> Self {
        let mut gamepad = Gamepad::default();
        gamepad.init(path);
        gamepad
    }

    pub fn disconnect(&mut self) {
        match &mut self.file {
            Some(f) => {
                f.close();
            }
            None => {}
        }
    }

    pub fn connect(&mut self) {
        match &self.file {
            Some(js_file) => {
                if !js_file.is_connected() {
                    self.open_mode = self.open_mode ^ OpenMode::NONBLOCK;
                    self.file = Some(JsFile::new(&self.path).open(self.open_mode));
                }
            }
            None => {
            }
        }
    }

    fn init(&mut self, path: &str) {
        let open_mode = OpenMode::read() | OpenMode::write() | OpenMode::nonblock();
        let file = JsFile::new(path).open(open_mode);
        let axis_count = file.read_axis_count().unwrap();
        let button_count = file.read_button_count().unwrap();

        let axes: Vec<Axis> = file
            .read_axis_mapping(axis_count as usize)
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, item)| Axis::parse(i as u8, item))
            .collect();

        let buttons: Vec<Button> = file
            .read_button_mapping(button_count as usize)
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, item)| Button::parse(i as u8, item))
            .collect();

        self.path = String::from(path);
        self.open_mode = open_mode;
        self.version = file.read_driver_version().unwrap();
        self.name = file.read_name().unwrap();
        self.axes = axes;
        self.buttons = buttons;

        utils::update_with_init_events(self, file.read_init_event_with_no_block());

        self.file = Some(file);
        self.disconnect();
    }

    pub fn update(&mut self) -> JsEvent {
        utils::update_with_events(self, self.file.as_ref().unwrap().read_event_with_block())
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_axes(&mut self) -> &mut Vec<Axis> {
        &mut self.axes
    }

    pub fn get_buttons(&mut self) -> &mut Vec<Button> {
        &mut self.buttons
    }
}


impl TForceFeedback for Gamepad {
    type Item = Gamepad;

    fn register_force_feedback(&mut self, path: &str) {
        self.ff_file = Some (
            joystick_core::FfFile::new(path).open(joystick_core::OpenMode::READ | joystick_core::OpenMode::WRITE)
        );
    }

    fn set_rumble_effect(&self) -> Result<u16, &'static str> {
        match &self.ff_file {
            Some(file) => {
                match file.set_rumble_effect() {
                    Ok(id) => {
                        Ok(id)
                    }
                    Err(_) => Err("set_rumble_effect error")
                }
            },
            None => Err("force feedback file is null")
        }
    }

    fn run_effect(&self, id: u16) -> bool {
        match &self.ff_file {
            Some(file) => {
                file.run_effect(id)
            },
            None => false
        }
    }

    fn remove_effect(&self, id: u16) -> bool {
        match &self.ff_file {
            Some(file) => {
                file.remove_effect(id)
            },
            None => false
        }
    }


}


#[macro_export]
macro_rules! begin_read {
    ($(#[$attr:meta])* $name:ident, $gamepad:expr) => {
        loop {
            $gamepad.connect();
            let js_event = $gamepad.update();
            // println!("{:?}", $gamepad);
            $name(
                $gamepad,
                js_event
            );
        }
    };
}

