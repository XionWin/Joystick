#[allow(dead_code)]
pub mod abs {
    pub const ABS_X: u16 = 0x00;
    pub const ABS_Y: u16 = 0x01;
    pub const ABS_Z: u16 = 0x02;
    pub const ABS_RX: u16 = 0x03;
    pub const ABS_RY: u16 = 0x04;
    pub const ABS_RZ: u16 = 0x05;
    pub const ABS_THROTTLE: u16 = 0x06;
    pub const ABS_RUDDER: u16 = 0x07;
    pub const ABS_WHEEL: u16 = 0x08;
    pub const ABS_GAS: u16 = 0x09;
    pub const ABS_BRAKE: u16 = 0x0a;
    pub const ABS_HAT0X: u16 = 0x10;
    pub const ABS_HAT0Y: u16 = 0x11;
    pub const ABS_HAT1X: u16 = 0x12;
    pub const ABS_HAT1Y: u16 = 0x13;
    pub const ABS_HAT2X: u16 = 0x14;
    pub const ABS_HAT2Y: u16 = 0x15;
    pub const ABS_HAT3X: u16 = 0x16;
    pub const ABS_HAT3Y: u16 = 0x17;
    pub const ABS_PRESSURE: u16 = 0x18;
    pub const ABS_DISTANCE: u16 = 0x19;
    pub const ABS_TILT_X: u16 = 0x1a;
    pub const ABS_TILT_Y: u16 = 0x1b;
    pub const ABS_TOOL_WIDTH: u16 = 0x1c;

    pub const ABS_VOLUME: u16 = 0x20;

    pub const ABS_MISC: u16 = 0x28;

    pub const ABS_RESERVED: u16 = 0x2e;

    pub const ABS_MT_SLOT: u16 = 0x2f;
    pub const ABS_MT_TOUCH_MAJOR: u16 = 0x30;
    pub const ABS_MT_TOUCH_MINOR: u16 = 0x31;
    pub const ABS_MT_WIDTH_MAJOR: u16 = 0x32;
    pub const ABS_MT_WIDTH_MINOR: u16 = 0x33;
    pub const ABS_MT_ORIENTATION: u16 = 0x34;
    pub const ABS_MT_POSITION_X: u16 = 0x35;
    pub const ABS_MT_POSITION_Y: u16 = 0x36;
    pub const ABS_MT_TOOL_TYPE: u16 = 0x37;
    pub const ABS_MT_BLOB_ID: u16 = 0x38;
    pub const ABS_MT_TRACKING_ID: u16 = 0x39;
    pub const ABS_MT_PRESSURE: u16 = 0x3a;
    pub const ABS_MT_DISTANCE: u16 = 0x3b;
    pub const ABS_MT_TOOL_X: u16 = 0x3c;
    pub const ABS_MT_TOOL_Y: u16 = 0x3d;

    pub const ABS_MAX: u16 = 0x3f;
    pub const ABS_CNT: u16 = ABS_MAX + 1;

}