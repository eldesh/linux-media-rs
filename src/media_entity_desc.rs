use std::os::fd::{AsRawFd, BorrowedFd};

use linux_media_sys as media;

use crate::error;
use crate::ioctl;
use crate::{EntityId, MediaEntity, MediaEntityFlags, MediaEntityFunctions, Version};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MediaEntityDesc {
    /// Entity ID, set by the application. When the ID is or’ed with MEDIA_ENT_ID_FLAG_NEXT, the driver clears the flag and returns the first entity with a larger ID. Do not expect that the ID will always be the same for each instance of the device. In other words, do not hardcode entity IDs in an application.
    id: EntityId,
    /// Entity name. This name must be unique within the media topology.
    name: String,
    /// Entity type.
    r#type: MediaEntityFunctions,
    /// Entity flags.
    flags: MediaEntityFlags,
    /// Number of pads
    pads: usize,
    /// Total number of outbound links.
    /// Inbound links are not counted in this field.
    links: u16,
}

#[derive(Debug)]
pub struct MediaEntityIter<'a> {
    fd: BorrowedFd<'a>,
    media_version: Version,
    id: EntityId,
}

impl<'a> MediaEntityIter<'a> {
    pub fn new(fd: BorrowedFd<'a>, media_version: Version, id: EntityId) -> Self {
        Self {
            fd,
            media_version,
            id,
        }
    }
}

/// Iterates over all MediaEntities with an ID greater than the stored ID.
impl<'a> Iterator for MediaEntityIter<'a> {
    type Item = MediaEntity;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut desc: media::media_entity_desc = std::mem::zeroed();
            desc.id = Into::<u32>::into(self.id) | media::MEDIA_ENT_ID_FLAG_NEXT;
            if ioctl!(self.fd, media::MEDIA_IOC_ENUM_ENTITIES, &mut desc).is_ok() {
                self.id = desc.id.into();
                Some(MediaEntity::from_raw_desc(self.media_version, desc))
            } else {
                None
            }
        }
    }
}
