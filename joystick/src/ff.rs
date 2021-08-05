

pub struct ForceFeedback {
    file: joystick_core::FfFile
}

impl ForceFeedback {
    pub fn new(path: &str) -> Self {
        Self {
            file: joystick_core::FfFile::new(path).open(joystick_core::OpenMode::READ | joystick_core::OpenMode::WRITE)
        }
    }

    pub fn set_rumble_effect(&self) -> Result<u16, &'static str> {
        match self.file.set_rumble_effect() {
            Ok(id) => {
                Ok(id)
            }
            Err(_) => Err("set_rumble_effect error")
        }
    }


    pub fn run_effect(&self, id: u16) -> bool {
        self.file.run_effect(id)
    }


}