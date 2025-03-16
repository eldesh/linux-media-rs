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
    pub id: InterfaceId,
    pub r#type: MediaInterfaceType,
    pub devnode: MediaIntfDevnode,
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
