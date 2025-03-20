use std::ffi::CStr;
use std::os::fd::{AsRawFd, BorrowedFd};

use linux_media_sys as media;
use serde::{Deserialize, Serialize};

use crate::error;
use crate::ioctl;
use crate::{EntityId, MediaEntity, MediaEntityFlags, MediaEntityFunctions, Version};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct MediaEntityDesc {
    /// Entity ID, set by the application. When the ID is or’ed with MEDIA_ENT_ID_FLAG_NEXT, the driver clears the flag and returns the first entity with a larger ID. Do not expect that the ID will always be the same for each instance of the device. In other words, do not hardcode entity IDs in an application.
    pub id: EntityId,
    /// Entity name. This name must be unique within the media topology.
    pub name: String,
    /// Entity type.
    pub r#type: MediaEntityFunctions,
    /// Entity flags.
    pub flags: MediaEntityFlags,
    /// Number of pads
    pub pads: usize,
    /// Total number of outbound links.
    /// Inbound links are not counted in this field.
    pub links: usize,
}

impl MediaEntityDesc {
    pub fn from_fd<F>(fd: F, entity: EntityId) -> error::Result<Self>
    where
        F: AsRawFd,
    {
        unsafe {
            let mut desc: media::media_entity_desc = std::mem::zeroed();
            desc.id = entity.into();
            ioctl!(fd, media::MEDIA_IOC_ENUM_ENTITIES, &mut desc)?;
            Ok(desc.into())
        }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn r#type(&self) -> MediaEntityFunctions {
        self.r#type.clone()
    }

    pub fn flags(&self) -> MediaEntityFlags {
        self.flags
    }

    pub fn pads(&self) -> usize {
        self.pads
    }

    pub fn links(&self) -> usize {
        self.links
    }
}

impl From<media::media_entity_desc> for MediaEntityDesc {
    fn from(desc: media::media_entity_desc) -> Self {
        Self {
            id: desc.id.into(),
            name: unsafe {
                CStr::from_ptr(desc.name.as_ptr())
                    .to_string_lossy()
                    .to_string()
            },
            r#type: desc.type_.try_into().unwrap(),
            flags: desc.flags.try_into().unwrap(),
            pads: desc.pads.try_into().unwrap(),
            links: desc.links.try_into().unwrap(),
        }
    }
}

/// Iterates over all MediaEntities.
///
/// # Details
/// Iterates over all MediaEntities with an ID greater than or equal to the stored ID.
/// Enumerated items are in ascending order of ID.
#[derive(Debug)]
pub struct MediaEntityIter<'a> {
    fd: BorrowedFd<'a>,
    media_version: Version,
    id: EntityId,
    // next item descriptor
    desc: Option<MediaEntityDesc>,
}

impl<'a> MediaEntityIter<'a> {
    pub fn new(fd: BorrowedFd<'a>, media_version: Version, id: EntityId) -> Self {
        Self {
            fd,
            media_version,
            id,
            desc: Self::desc(fd, id),
        }
    }

    fn desc(fd: BorrowedFd<'_>, id: EntityId) -> Option<MediaEntityDesc> {
        unsafe {
            let mut desc: media::media_entity_desc = std::mem::zeroed();
            desc.id = Into::<u32>::into(id);
            if ioctl!(fd, media::MEDIA_IOC_ENUM_ENTITIES, &mut desc).is_ok() {
                Some(desc.into())
            } else {
                None
            }
        }
    }
}

impl<'a> Iterator for MediaEntityIter<'a> {
    type Item = MediaEntity;
    fn next(&mut self) -> Option<Self::Item> {
        match self.desc.clone() {
            Some(desc) => {
                let entity = MediaEntity::from_desc(self.media_version, desc);
                if let Some(desc) =
                    Self::desc(self.fd, self.id | media::MEDIA_ENT_ID_FLAG_NEXT.into())
                {
                    self.id = desc.id.into();
                    self.desc = Some(desc);
                } else {
                    self.desc = None;
                }
                Some(entity)
            }
            None => None,
        }
    }
}
