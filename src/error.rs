use std::fmt;
use std::io;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io {
        source: io::Error,
        path: PathBuf,
    },
    Ioctl {
        code: libc::c_int,
        api: libc::c_ulong,
    },
    FileNotFound {
        path: PathBuf,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Display::fmt(&Error)")
    }
}

pub fn trap_io_error(err: io::Error, path: PathBuf) -> Error {
    use io::ErrorKind::*;
    match err.kind() {
        NotFound => Error::FileNotFound { path },
        _ => Error::Io { source: err, path },
    }
}
