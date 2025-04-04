use std::fmt;
use std::io;
use std::os::fd::{AsRawFd, RawFd};
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Generic io error
    Io { source: io::Error, path: PathBuf },
    /// File not found
    FileNotFound { path: PathBuf, source: io::Error },
    /// Generic ioctl error
    /// `code` is constructed from [`std::io::Error::from_raw_os_error`].
    Ioctl {
        fd: RawFd,
        code: io::Error,
        api: libc::c_ulong,
    },
    /// The ioctl is not supported by the file descriptor.
    NotSupportedIoctl {
        fd: RawFd,
        code: libc::c_int,
        api: libc::c_ulong,
    },
    /// The ioctl can’t be handled because the device is busy. This is typically return while device is streaming, and an ioctl tried to change something that would affect the stream, or would require the usage of a hardware resource that was already allocated. The ioctl must not be retried without performing another action to fix the problem first (typically: stop the stream before retrying).
    DeviceIsBusy {
        fd: RawFd,
        code: libc::c_int,
        api: libc::c_ulong,
    },
    /// The request was already queued or the application queued the first buffer directly, but later attempted to use a request.
    RequestIsAlreadyQueued {
        fd: RawFd,
        code: libc::c_int,
        api: libc::c_ulong,
    },
    /// The request did not contain any buffers. All requests are required to have at least one buffer. This can also be returned if some required configuration is missing in the request.
    RequestNotContainBuffers {
        fd: RawFd,
        code: libc::c_int,
        api: libc::c_ulong,
    },
    /// Out of memory when allocating internal data structures for a request.
    OutOfMemory {
        fd: RawFd,
        code: libc::c_int,
        api: libc::c_ulong,
    },
    /// Request has invalid data
    RequestHasInvalidData {
        fd: RawFd,
        code: libc::c_int,
        api: libc::c_ulong,
    },
    /// The hardware is in a bad state. To recover, the application needs to stop streaming to reset the hardware state and then try to restart streaming.
    HardwareBadState {
        fd: RawFd,
        code: libc::c_int,
        api: libc::c_ulong,
    },
    /// parse error as [`crate::MediaInterfaceType`]
    InterfaceTypeParseError { from: u32 },
    /// parse error as [`crate::MediaEntityFunctions`]
    EntityFunctionsParseError { from: u32 },
    /// parse error as [`crate::MediaEntityFlags`]
    EntityFlagsParseError { from: u32 },
    /// parse error as [`crate::MediaPadFlags`]
    PadFlagsParseError { from: u32 },
    /// parse error as [`crate::MediaLinkFlags`]
    LinkFlagsParseError { from: u32 },
}

impl Error {
    /// Constructs an Error from an ioctl failure
    ///
    /// # Arguments
    /// - `fd`  : The file descriptor on which the ioctl error occurred.
    /// - `code`: The return code from the ioctl call.
    /// - `api` : The kind of operation that resulted in the error.
    ///
    /// # References
    /// <https://www.kernel.org/doc/html/v6.9/userspace-api/media/gen-errors.html>
    pub fn ioctl_error<F>(fd: F, code: libc::c_int, api: libc::c_ulong) -> Error
    where
        F: AsRawFd,
    {
        use Error::*;
        let fd = fd.as_raw_fd();
        match code {
            libc::EBUSY => DeviceIsBusy { fd, code, api },
            libc::ENOTTY => NotSupportedIoctl { fd, code, api },
            _ => Ioctl {
                fd,
                code: io::Error::from_raw_os_error(code),
                api,
            },
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            Io { path, .. } => write!(f, "io error: {}", path.display()),
            FileNotFound { path, .. } => write!(f, "file not found: {}", path.display()),
            Ioctl { fd, code, api } => {
                write!(f, "generic ioctl error {}: 0x{:02X}: {}", fd, api, code)
            }
            NotSupportedIoctl { fd, code, api } => write!(
                f,
                "the ioctl is not supported by the file descriptor {}: 0x{:02X}: {}",
                fd, api, code
            ),
            DeviceIsBusy { fd, code, api } => {
                write!(f, "the device is busy {}: 0x{:02X}: {}", fd, api, code)
            }
            RequestIsAlreadyQueued { fd, code, api } => {
                write!(
                    f,
                    "the request is already queued {}: 0x{:02X}: {}",
                    fd, api, code
                )
            }
            RequestNotContainBuffers { fd, code, api } => {
                write!(
                    f,
                    "the request did not contain any buffers {}: 0x{:02X}: {}",
                    fd, api, code
                )
            }
            OutOfMemory { fd, code, api } => {
                write!(f, "Out of memory when allocating internal data structures for this request. {}: 0x{:02X}: {}", fd, api, code)
            }
            RequestHasInvalidData { fd, code, api } => {
                write!(
                    f,
                    "The request has invalid data. {}: 0x{:02X}: {}",
                    fd, api, code
                )
            }
            HardwareBadState { fd, code, api } => {
                write!(f, "The hardware is in a bad state. To recover, the application needs to stop streaming to reset the hardware state and then try to restart streaming. {}: 0x{:02X}: {}", fd, api, code)
            }
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
            LinkFlagsParseError { from, .. } => {
                write!(f, "link flags parse error: {}", from)
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

#[cfg(test)]
pub mod test {
    use super::*;

    // https://www.kernel.org/doc/html/v6.9/userspace-api/media/gen-errors.html
    #[test]
    fn enotty_is_not_supported() {
        use Error::*;
        let err = NotSupportedIoctl {
            fd: 0,
            code: libc::ENOTTY,
            api: 0,
        };
        assert!(matches!(
            err,
            NotSupportedIoctl {
                code: libc::ENOTTY,
                ..
            }
        ));
    }
}
