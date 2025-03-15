use linux_media_sys as media;

use crate::EntityId;
use crate::MediaPadFlags;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MediaPadDesc {
    /// ID of the entity this pad belongs to.
    entity: EntityId,
    /// Pad index
    index: usize,
    /// Pad flags
    flags: MediaPadFlags,
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
