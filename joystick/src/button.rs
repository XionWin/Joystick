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
    fn get_id(&self) -> u8 {
        self.id
    }
    fn get_alias(&self) -> &str {
        &self.alias
    }
    fn get_value(&self) -> i16 {
        self.value
    }
}

impl Button {
    pub fn parse(id: u8, key: &jsc::js::linux::Key) -> Self {
        Self {
            id,
            alias: key.to_string(),
            value: 0
        }
    }
}


