use std::path::PathBuf;

use linux_media_sys as media;
use serde::{Deserialize, Serialize};

/// A wrapper type of [`linux_media_sys::media_v2_intf_devnode`]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct MediaIntfDevnode {
    pub major: u32,
    pub minor: u32,
}

impl From<media::media_v2_intf_devnode> for MediaIntfDevnode {
    fn from(devnode: media::media_v2_intf_devnode) -> Self {
        MediaIntfDevnode {
            major: devnode.major,
            minor: devnode.minor,
        }
    }
}

impl From<MediaIntfDevnode> for PathBuf {
    fn from(devnode: MediaIntfDevnode) -> Self {
        PathBuf::from(format!("/sys/dev/char/{}:{}", devnode.major, devnode.minor))
    }
}
