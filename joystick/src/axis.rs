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

