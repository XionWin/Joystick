pub use crate::io::def;

#[macro_export]
macro_rules! ioc {
    ($dir:expr, $ty:expr, $nr:expr, $sz:expr) => (
        (($dir as def::IoctlNumType & def::DIRMASK) << def::DIRSHIFT) |
        (($ty as def::IoctlNumType & def::TYPEMASK) << def::TYPESHIFT) |
        (($nr as def::IoctlNumType & def::NRMASK) << def::NRSHIFT) |
        (($sz as def::IoctlNumType & def::SIZEMASK) << def::SIZESHIFT))
}

#[macro_export]
macro_rules! get_buf_req {
    ($m:expr, $n:expr, $l: expr) => (
        ioc!(def::READ, $m, $n, $l)
    )
}

#[macro_export]
macro_rules! read_number {
    ($fd:expr, $req:expr, $t: ty) => {
        {
            let mut value: $t = 0;
            unsafe {
                match libc::ioctl($fd, $req, &mut value) {
                    0 => Ok(value),
                    _ => Err("read_number error")
                }
            }
        }
    };
}

#[macro_export]
macro_rules! write_number {
    ($fd:expr, $req:expr, $v: expr) => {
        {
            unsafe {
                match libc::ioctl($fd, $req, $v) {
                    0 => true,
                    _ => false
                }
            }
        }
    };
}

#[macro_export]
macro_rules! read_buf {
    ($fd:expr, $req:expr, $buf: expr) => {
        {
            unsafe {
                match libc::ioctl($fd, $req, $buf) {
                    len if len > 0 => Ok(len),
                    _ => Err("read_buf error")
                }
            }
        }
    };
}

