#[macro_export]
macro_rules! ioc {
    ($dir:expr, $ty:expr, $nr:expr, $sz:expr) => (
        (($dir as env::IoctlNumType & env::DIRMASK) << env::DIRSHIFT) |
        (($ty as env::IoctlNumType & env::TYPEMASK) << env::TYPESHIFT) |
        (($nr as env::IoctlNumType & env::NRMASK) << env::NRSHIFT) |
        (($sz as env::IoctlNumType & env::SIZEMASK) << env::SIZESHIFT))
}

#[macro_export]
macro_rules! get_buf_req {
    ($m:expr, $n:expr, $l: expr) => (
        ioc!(env::READ, $m, $n, $l)
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