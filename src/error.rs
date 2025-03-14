use std::fmt;
use std::io;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// general io error
    Io { source: io::Error, path: PathBuf },
    /// ioctl error
    Ioctl {
        code: libc::c_int,
        api: libc::c_ulong,
    },
    /// file not found
    FileNotFound { path: PathBuf, source: io::Error },
    /// parse error as [`crate::MediaInterfaceType`]
    InterfaceTypeParseError { from: u32 },
    /// parse error as [`crate::MediaEntityFunctions`]
    EntityFunctionsParseError { from: u32 },
    /// parse error as [`crate::MediaEntityFlags`]
    EntityFlagsParseError { from: u32 },
    /// parse error as [`crate::MediaPadFlags`]
    PadFlagsParseError { from: u32 },
    /// parse error as [`crate::MediaLinkType`]
    LinkTypeParseError { from: u32 },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            Io { path, .. } => write!(f, "io error: {}", path.display()),
            Ioctl { code, api } => write!(f, "ioctl failed with: {} for 0x{:02X}", code, api),
            FileNotFound { path, .. } => write!(f, "file not found: {}", path.display()),
            InterfaceTypeParseError { from, .. } => {
                write!(f, "interface type parse error: {}", from)
            }
            EntityFunctionsParseError { from, .. } => {
                write!(f, "entity functions parse error: {}", from)
            }
            EntityFlagsParseError { from, .. } => {
                write!(f, "entity flags parse error: {}", from)
            }
            PadFlagsParseError { from, .. } => {
                write!(f, "pad flags parse error: {}", from)
            }
            LinkTypeParseError { from, .. } => {
                write!(f, "link type parse error: {}", from)
            }
        }
    }
}

pub fn trap_io_error(err: io::Error, path: PathBuf) -> Error {
    use io::ErrorKind::*;
    match err.kind() {
        NotFound => Error::FileNotFound { path, source: err },
        _ => Error::Io { source: err, path },
    }
}
