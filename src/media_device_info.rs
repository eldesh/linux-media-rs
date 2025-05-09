use std::ffi::CStr;
use std::fmt;
use std::fs::OpenOptions;
use std::os::fd::{AsFd, AsRawFd, OwnedFd};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use linux_media_sys as media;
use serde::{Deserialize, Serialize};

use crate::error;
use crate::ioctl;
use crate::version::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct MediaDeviceInfo {
    pub driver: String,
    pub model: String,
    pub serial: String,
    pub bus_info: String,
    pub media_version: Version,
    pub hw_revision: u32,
    pub driver_version: Version,
}

impl fmt::Debug for MediaDeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MediaDeviceInfo {{ driver: \"{}\", model: \"{}\", serial: \"{}\", bus_info: \"{}\", media_version: {}, hw_revision: 0x{:02X}, driver_version: {} }}",
            self.driver,
            self.model,
            self.serial,
            self.bus_info,
            self.media_version,
            self.hw_revision,
            self.driver_version,
        )
    }
}

impl MediaDeviceInfo {
    pub fn from_path<P>(path: P) -> error::Result<(OwnedFd, Self)>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let fd: OwnedFd = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_CLOEXEC)
            .open(path)
            .map_err(|err| error::trap_io_error(err, path.to_path_buf()))?
            .into();
        let info = Self::from_fd(fd.as_fd())?;
        Ok((fd, info))
    }

    pub fn from_fd<F>(fd: F) -> error::Result<Self>
    where
        F: AsFd,
    {
        let info = unsafe {
            let mut info: media::media_device_info = std::mem::zeroed();
            ioctl!(fd.as_fd(), media::MEDIA_IOC_DEVICE_INFO, &mut info)?;
            info
        };
        Ok(info.into())
    }

    pub fn driver(&self) -> &str {
        &self.driver
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn serial(&self) -> &str {
        &self.serial
    }

    pub fn bus_info(&self) -> &str {
        &self.bus_info
    }

    pub fn media_version(&self) -> Version {
        self.media_version.clone()
    }

    pub fn hw_revision(&self) -> u32 {
        self.hw_revision
    }

    pub fn driver_version(&self) -> Version {
        self.driver_version.clone()
    }
}

impl From<media::media_device_info> for MediaDeviceInfo {
    fn from(info: media::media_device_info) -> Self {
        let driver = unsafe { CStr::from_ptr(info.driver.as_ptr()) }
            .to_string_lossy()
            .to_string();
        let model = unsafe { CStr::from_ptr(info.model.as_ptr()) }
            .to_string_lossy()
            .to_string();
        let serial = unsafe { CStr::from_ptr(info.serial.as_ptr()) }
            .to_string_lossy()
            .to_string();
        let bus_info = unsafe { CStr::from_ptr(info.bus_info.as_ptr()) }
            .to_string_lossy()
            .to_string();
        let media_version = info.media_version.into();
        let hw_revision = info.hw_revision;
        let driver_version = info.driver_version.into();
        Self {
            driver,
            model,
            serial,
            bus_info,
            media_version,
            hw_revision,
            driver_version,
        }
    }
}
