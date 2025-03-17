use linux_media_sys as media;
use serde::{Deserialize, Serialize};

use crate::EntityId;
use crate::MediaPadFlags;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct MediaPadDesc {
    entity: EntityId,
    index: usize,
    flags: MediaPadFlags,
}

impl MediaPadDesc {
    pub fn new(entity: EntityId, index: usize, flags: MediaPadFlags) -> Self {
        Self {
            entity,
            index,
            flags,
        }
    }

    /// ID of the entity this pad belongs to.
    pub fn id(&self) -> EntityId {
        self.entity
    }

    /// Pad index
    pub fn index(&self) -> usize {
        self.index
    }

    /// Pad flags
    pub fn flags(&self) -> MediaPadFlags {
        self.flags
    }
}

impl From<media::media_pad_desc> for MediaPadDesc {
    fn from(desc: media::media_pad_desc) -> Self {
        Self {
            entity: desc.entity.into(),
            index: desc.index.into(),
            flags: desc.flags.try_into().unwrap(),
        }
    }
}

impl From<MediaPadDesc> for media::media_pad_desc {
    fn from(desc: MediaPadDesc) -> media::media_pad_desc {
        let mut raw: media::media_pad_desc = unsafe { std::mem::zeroed() };
        raw.entity = desc.entity.into();
        raw.index = desc.index as u16;
        raw.flags = desc.flags.into();
        raw
    }
}
