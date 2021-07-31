

#[derive(Debug)]
pub enum EventType {
    Unknown = 0b0000,
    Button = 0b0001,
    Axis = 0b0010
}

#[derive(Debug)]
pub struct JsEvent {
    event_type: EventType,
    id: u8,
    alias: String,
    value: i16
}

impl JsEvent {
    pub fn new (event_type: EventType, id: u8, alias: &str, value: i16) -> Self {
        Self {
            event_type,
            id,
            alias: String::from(alias),
            value
        }
    }
}

impl Default for JsEvent {
    fn default() -> Self {
        Self {
            event_type: EventType::Unknown,
            id: 0,
            alias: String::from(""),
            value: 0
        }
    }
}