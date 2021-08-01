#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct Trigger {
    pub button: u16,
    pub interval: u16,
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct Replay {
    pub length: u16,
    pub delay: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct FfEffect {
    pub type_: u16,
    pub id: i16,
    pub direction: u16,
    pub trigger: Trigger,
    pub replay: Replay,
    // FIXME this is actually a union
    pub effect: UEffect
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union UEffect {
    pub periodic: PeriodicEffect,
    pub rumble: RumbleEffect
}


#[derive(Copy, Clone)]
#[repr(C)]
pub struct RumbleEffect {
    pub strong_magnitude: u32,
    pub weak_magnitude: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Envelope {
	pub attack_length: u16,
	pub attack_level: u16,
	pub fade_length: u16,
	pub fade_level: u16,
}

impl Default for Envelope {
    fn default() -> Self {
        Self {
            attack_length: 0,
            attack_level: 0,
            fade_length: 0,
            fade_level: 0
        }
    }
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
pub struct PeriodicEffect {
	pub waveform: WaveForm,
	pub period: u16,
	pub magnitude: i16,
	pub offset: i16,
	pub phase: u16,

	pub envelope: Envelope,

	pub custom_len: u32,
	pub __user: *const u16,
}

impl Default for PeriodicEffect {
    fn default() -> Self {
        Self {
            waveform: WaveForm::FF_SINE,
            period: 0,
            magnitude: 0,
            offset: 0,
            phase: 0,
            envelope: Default::default(),
            custom_len: 0,
            __user: unsafe {
                core::mem::zeroed()
            }
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct InputEvent {
    pub time: libc::timeval,
    pub type_: u16,
    pub code: u16,
    pub value: i32,
}

impl ::std::default::Default for InputEvent {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}