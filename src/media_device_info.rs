use std::ffi::CStr;
use std::fmt;
use std::fs::OpenOptions;
use std::os::fd::{AsRawFd, OwnedFd};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use linux_media_sys as media;

use crate::error;
use crate::version::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
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
        let device_node = path.to_path_buf();
        let fd: OwnedFd = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_CLOEXEC)
            .open(path)
            .map_err(|err| error::trap_io_error(err, device_node.clone()))?
            .into();

        let info = unsafe {
            let mut info: media::media_device_info = std::mem::zeroed();
            let ret = libc::ioctl(fd.as_raw_fd(), media::MEDIA_IOC_DEVICE_INFO, &mut info);
            if ret != 0 {
                return Err(error::Error::Ioctl {
                    code: ret,
                    api: media::MEDIA_IOC_DEVICE_INFO,
                });
            }
            info
        };

        Ok((fd, info.into()))
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
        let driver = CStr::from_bytes_until_nul(&info.driver)
            .unwrap()
            .to_string_lossy()
            .to_string();
        let model = CStr::from_bytes_until_nul(&info.model)
            .unwrap()
            .to_string_lossy()
            .to_string();
        let serial = CStr::from_bytes_until_nul(&info.serial)
            .unwrap()
            .to_string_lossy()
            .to_string();
        let bus_info = CStr::from_bytes_until_nul(&info.bus_info)
            .unwrap()
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
