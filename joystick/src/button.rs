use joystick_core as jsc;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Button {
    id: u8,
    alias: String,
    value: i16
}

impl Clone for Button {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            alias: self.alias.clone(),
            value: self.value
        }
    }
}

impl super::control::Control for Button {
    fn set_value(&mut self, value: i16) {
        self.value = value
    }
}

impl Button {
    pub fn parse(id: u8, key: &jsc::key::Key) -> Self {
        Self {
            id,
            alias: key.to_string(),
            value: 0
        }
    }
}


