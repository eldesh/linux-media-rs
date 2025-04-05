use std::fs::OpenOptions;
use std::os::fd::{AsFd, BorrowedFd, OwnedFd};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use crate::error;
use crate::MediaDeviceInfo;
use crate::MediaTopology;
use crate::Request;
use crate::Version;

#[derive(Debug)]
pub struct Media {
    info: MediaDeviceInfo,
    path: PathBuf,
    fd: OwnedFd,
}

impl Media {
    pub fn from_path<P>(path: P) -> error::Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_path_buf();
        let fd: OwnedFd = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_CLOEXEC)
            .open(&path)
            .map_err(|err| error::trap_io_error(err, path.clone()))?
            .into();
        let info = MediaDeviceInfo::from_fd(fd.as_fd())?;
        Ok(Self { info, path, fd })
    }

    pub fn info(&self) -> &MediaDeviceInfo {
        &self.info
    }

    pub fn media_version(&self) -> Version {
        self.info.media_version()
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn device_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }

    pub fn new_request(&self) -> error::Result<Request<'_>> {
        Request::new(self.fd.as_fd())
    }

    pub fn new_topology(&self) -> error::Result<MediaTopology> {
        MediaTopology::from_fd(self.info(), self.device_fd())
    }
}
