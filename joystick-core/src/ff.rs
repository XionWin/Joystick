

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
    pub effect: effect_union
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union effect_union {
    pub periodic: ff_periodic_effect,
    pub rumble: ff_rumble_effect
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct ff_rumble_effect {
    pub strong_magnitude: u32,
    pub weak_magnitude: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ff_envelope {
	pub attack_length: u16,
	pub attack_level: u16,
	pub fade_length: u16,
	pub fade_level: u16,
}

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
pub enum WaveForm {
    FF_SQUARE = 0x58,
    FF_TRIANG = 0x59,
    FF_SINE = 0x5a,
    FF_SAW_UP = 0x5b,
    FF_SAW_DO = 0x5c,
    FF_CUSTOM = 0x5d,
    FF_GAIN = 0x60,
    FF_AUTOCE = 0x61,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ff_periodic_effect {
	pub waveform: WaveForm,
	pub period: u16,
	pub magnitude: i16,
	pub offset: i16,
	pub phase: u16,

	pub envelope: ff_envelope,

	pub custom_len: u32,
	pub __user: *const u16,
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