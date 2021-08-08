bitflags! {
    pub struct OpenMode: u8 {
        const NONE = 0b0000;
        const READ = 0b0001;
        const WRITE = 0b0010;
        const NONBLOCK = 0b1000;
    }
}

impl OpenMode {
    pub fn read() -> OpenMode {
        OpenMode::READ as OpenMode
    }
    pub fn write() -> OpenMode {
        OpenMode::WRITE as OpenMode
    }
    pub fn nonblock() -> OpenMode {
        OpenMode::NONBLOCK as OpenMode
    }
}