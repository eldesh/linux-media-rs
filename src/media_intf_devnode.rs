use linux_media_sys as media;
use serde::{Deserialize, Serialize};

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
