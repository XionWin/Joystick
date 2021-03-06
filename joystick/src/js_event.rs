

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EventType {
    Unknown = 0b0000,
    Button = 0b0001,
    Axis = 0b0010,
	ButtonInit = 0x01 | 0x80,
	AxisInit = 0x02 | 0x80
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

    pub fn get_event_type(&self) -> EventType {
        self.event_type
    }
    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_alias(&self) -> String {
        self.alias.clone()
    }
    pub fn get_value(&self) -> i16 {
        self.value
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