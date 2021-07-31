

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct ff_trigger {
    pub button: u16,
    pub interval: u16,
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct ff_replay {
    pub length: u16,
    pub delay: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ff_effect {
    pub type_: u16,
    pub id: i16,
    pub direction: u16,
    pub trigger: ff_trigger,
    pub replay: ff_replay,
    // FIXME this is actually a union
    #[cfg(target_pointer_width = "64")]
    pub u: [u64; 4],
    #[cfg(target_pointer_width = "32")]
    pub u: [u32; 7],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ff_rumble_effect {
    pub strong_magnitude: u16,
    pub weak_magnitude: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_event {
    pub time: libc::timeval,
    pub type_: u16,
    pub code: u16,
    pub value: i32,
}

impl ::std::default::Default for input_event {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}