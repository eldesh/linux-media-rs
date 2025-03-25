use derive_more::{Display, From, Into};
use linux_media_sys as media;
use serde::{Deserialize, Serialize};

use crate::media_interface_type::MediaInterfaceType;
use crate::media_intf_devnode::MediaIntfDevnode;

#[derive(
    Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, From, Into, Display, Serialize, Deserialize,
)]
pub struct InterfaceId(u32);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct MediaInterface {
    id: InterfaceId,
    r#type: MediaInterfaceType,
    devnode: MediaIntfDevnode,
}

impl MediaInterface {
    pub fn new(id: InterfaceId, r#type: MediaInterfaceType, devnode: MediaIntfDevnode) -> Self {
        Self {
            id,
            r#type,
            devnode,
        }
    }

    pub fn id(&self) -> InterfaceId {
        self.id
    }

    pub fn r#type(&self) -> MediaInterfaceType {
        self.r#type
    }

    pub fn devnode(&self) -> MediaIntfDevnode {
        self.devnode
    }

    /// Get the path to the charactor device constructed with:
    /// `/sys/dev/char/{devnode.major}:{devnode.minor}`
    pub fn path(&self) -> PathBuf {
        PathBuf::from(format!(
            "/sys/dev/char/{}:{}",
            self.devnode.major, self.devnode.minor
        ))
    }
}

impl From<media::media_v2_interface> for MediaInterface {
    fn from(intf: media::media_v2_interface) -> Self {
        Self {
            id: intf.id.into(),
            r#type: intf.intf_type.try_into().unwrap(),
            devnode: unsafe { intf.__bindgen_anon_1.devnode.into() },
        }
    }
}
