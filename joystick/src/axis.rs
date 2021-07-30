use joystick_core as jsc;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Axis {
    id: u8,
    alias: String,
    value: i16
}

impl Clone for Axis {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            alias: self.alias.clone(),
            value: self.value
        }
    }
}

impl Axis {
    pub fn parse(id: u8, axis: &jsc::axis::Axis) -> Self {
        Self {
            id,
            alias: axis.to_string(),
            value: 0
        }
    }
}

