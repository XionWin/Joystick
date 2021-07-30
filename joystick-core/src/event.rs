use std::mem::zeroed;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct Event {
	pub time: libc::__u32,
	pub value: libc::__s16,
	pub type_: libc::__u8,
	pub number: libc::__u8,
}

impl Default for Event {
	fn default() -> Self {
		unsafe {
            zeroed::<Event>()
        }
	}
}

#[allow(dead_code)]
pub enum EventType {
    EventButton = 0x01,    /* button pressed/released */
	EventAxis = 0x02,      /* joystick moved */
	EventInit = 0x80
}
