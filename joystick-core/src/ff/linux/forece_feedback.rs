#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
pub enum EffectType {
    Rumble = 0x50,
    Periodic = 0x51,
    Constant = 0x52,
    Spring = 0x53,
    Fiction = 0x54,
    Damper = 0x55,
    Inertia = 0x56,
    Ramp = 0x57,
}

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
    pub effect_type: EffectType,
    pub id: i16,
    pub direction: u16,
    pub trigger: Trigger,
    pub replay: Replay,
    pub effect: UEffect
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union UEffect {
    pub periodic: PeriodicEffect,
    pub rumble: RumbleEffect
    // there will be some other effects.
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u16)]
pub enum WaveForm {
    Square = 0x58,
    Triangle = 0x59,
    Sine = 0x5a,
    SawUp = 0x5b,
    SawDown = 0x5c,
    Custom = 0x5d
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
	pub custom_data: *const u16,
}

impl Default for PeriodicEffect {
    fn default() -> Self {
        Self {
            waveform: WaveForm::Sine,
            period: 0,
            magnitude: 0,
            offset: 0,
            phase: 0,
            envelope: Default::default(),
            custom_len: 0,
            custom_data: unsafe {
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