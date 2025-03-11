use std::ffi::CStr;
use std::fmt;
use std::fs::OpenOptions;
use std::os::fd::{AsRawFd, OwnedFd};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use linux_media_sys as media;

use crate::error;
use crate::media_version::*;

pub struct MediaDeviceInfo {
    device_node: PathBuf,
    fd: OwnedFd,
    info: media::media_device_info,
}

impl fmt::Debug for MediaDeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MediaDeviceInfo {{ device_node: {}, fd: {:?}, info: {{ driver: \"{}\", model: \"{}\", serial: \"{}\", bus_info: \"{}\", media_version: {}, hw_revision: 0x{:02X}, driver_version: {} }} }}",
            self.device_node.display(),
            self.fd,
            self.driver(),
            self.model(),
            self.serial(),
            self.bus_info(),
            self.media_version(),
            self.hw_revision(),
            self.driver_version(),
        )
    }
}

impl MediaDeviceInfo {
    pub fn new<P>(path: P) -> error::Result<Self>
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

        Ok(Self {
            device_node,
            fd,
            info,
        })
    }

    pub fn driver(&self) -> &str {
        CStr::from_bytes_until_nul(&self.info.driver)
            .unwrap()
            .to_str()
            .unwrap()
    }

    pub fn model(&self) -> &str {
        CStr::from_bytes_until_nul(&self.info.model)
            .unwrap()
            .to_str()
            .unwrap()
    }

    pub fn serial(&self) -> &str {
        CStr::from_bytes_until_nul(&self.info.serial)
            .unwrap()
            .to_str()
            .unwrap()
    }

    pub fn bus_info(&self) -> &str {
        CStr::from_bytes_until_nul(&self.info.bus_info)
            .unwrap()
            .to_str()
            .unwrap()
    }

    pub fn media_version(&self) -> MediaVersion {
        self.info.media_version.into()
    }

    pub fn hw_revision(&self) -> u32 {
        self.info.hw_revision
    }

    pub fn driver_version(&self) -> MediaVersion {
        self.info.driver_version.into()
    }
}
