use super::{axis::Axis, button::Button};
use joystick_core as jsc;
use jsc::{JsFile, OpenMode};

#[derive(Debug)]
pub struct Gamepad {
    path: String,
    open_mode: OpenMode,
    file: Option<JsFile>,

    version: u32,
    name: String,

    axes: Vec<Axis>,
    buttons: Vec<Button>,
}

impl Default for Gamepad {
    fn default() -> Self {
        Gamepad {
            path: String::from(""),
            open_mode: OpenMode::NONE,
            file: Option::None,
            version: 0,
            name: String::from(""),
            axes: Vec::<Axis>::new(),
            buttons: Vec::<Button>::new(),
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
        match self.file {
            Some(_) => {}
            None => {}
        }
    }

    fn init(&mut self, path: &str) {
        let open_mode = OpenMode::read() | OpenMode::write() | OpenMode::nonblock();
        let file = JsFile::new(path).open(open_mode);

        self.path = String::from(path);
        self.open_mode = open_mode;
        self.version = file.read_driver_version().unwrap();
        self.name = file.read_name().unwrap();
        self.axes = Vec::<Axis>::new();
        self.buttons = Vec::<Button>::new();
        self.file = Some(file);

        self.disconnect();
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_axes(&self) -> &Vec<Axis> {
        &(self.axes)
    }

    pub fn get_buttons(&self) -> &Vec<Button> {
        &(self.buttons)
    }
}
