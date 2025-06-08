extern crate alloc;


use crate::uefi::*;

use alloc::string::String;



#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    EfiError(EfiStatus),
    Failed(&'static str),
    FailedString(String),
    FileNameTooLong,
    GraphicsError,
    PciBusDeviceFuncationOutOfRange,
    ReadFileSizeMismatch { expected: usize, actual: usize },
    ApicRegIndexOutOfRange,
    CalcOutOfRange,
    PageNotFound,
    PciBarInvalid,
    PciEcmOutOfRange,
    TryFromIntError,
    LockFailed,
    NoliError // NoliError(NoliError),
}
impl From<EfiStatus> for Error {
    fn from(e: EfiStatus) -> Self {
        Error::EfiError(e)
    }
}
impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::FailedString(s)
    }
}
impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Error::Failed(s)
    }
}

pub type Result<T> = core::result::Result<T,Error>;
