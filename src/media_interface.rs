use derive_more::{Display, From, Into};
use linux_media_sys as media;

use crate::media_interface_type::MediaInterfaceType;
use crate::media_intf_devnode::MediaIntfDevnode;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, From, Into, Display)]
pub struct InterfaceId(u32);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MediaInterface {
    id: InterfaceId,
    r#type: MediaInterfaceType,
    devnode: MediaIntfDevnode,
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
