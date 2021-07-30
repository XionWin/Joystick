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


