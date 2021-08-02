// for linux arm

pub type IoctlNumType = ::libc::c_int;

#[allow(dead_code)]
pub mod consts {
    #[doc(hidden)]
    pub const NONE: u8 = 0;
    #[doc(hidden)]
    pub const READ: u8 = 2;
    #[doc(hidden)]
    pub const WRITE: u8 = 1;
    #[doc(hidden)]
    pub const SIZEBITS: u8 = 14;
    #[doc(hidden)]
    pub const DIRBITS: u8 = 2;
}

#[doc(hidden)]
pub const NRBITS: IoctlNumType = 8;
#[doc(hidden)]
pub const TYPEBITS: IoctlNumType = 8;

use super::env::consts::*;

#[doc(hidden)]
pub const NRSHIFT: IoctlNumType = 0;
#[doc(hidden)]
pub const TYPESHIFT: IoctlNumType = NRSHIFT + NRBITS as IoctlNumType;
#[doc(hidden)]
pub const SIZESHIFT: IoctlNumType = TYPESHIFT + TYPEBITS as IoctlNumType;
#[doc(hidden)]
pub const DIRSHIFT: IoctlNumType = SIZESHIFT + SIZEBITS as IoctlNumType;

#[doc(hidden)]
pub const NRMASK: IoctlNumType = (1 << NRBITS) - 1;
#[doc(hidden)]
pub const TYPEMASK: IoctlNumType = (1 << TYPEBITS) - 1;
#[doc(hidden)]
pub const SIZEMASK: IoctlNumType = (1 << SIZEBITS) - 1;
#[doc(hidden)]
pub const DIRMASK: IoctlNumType = (1 << DIRBITS) - 1;
