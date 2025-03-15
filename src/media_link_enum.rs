use std::os::fd::AsRawFd;

use crate::error;
use crate::ioctl;
use crate::{EntityId, MediaEntityDesc, MediaLinkDesc, MediaPadDesc};

use linux_media_sys as media;

/// Enumerates MediaPads and/or MediaLinks associated to an Entity specified with id.
#[derive(Debug)]
pub struct MediaLinksEnum {
    entity: EntityId,
    pads: Vec<MediaPadDesc>,
    links: Vec<MediaLinkDesc>,
}

fn zeros_vec<T>(num: usize) -> Vec<T>
where
    T: Clone,
{
    let mut xs = vec![];
    xs.resize(num, unsafe { std::mem::zeroed() });
    xs
}

impl MediaLinksEnum {
    pub fn new<F>(fd: F, entity: EntityId) -> error::Result<Self>
    where
        F: AsRawFd,
    {
        let desc = MediaEntityDesc::from_fd(fd.as_raw_fd(), entity)?;
        let mut enum_links: media::media_links_enum = unsafe { std::mem::zeroed() };
        enum_links.entity = entity.into();
        unsafe {
            let mut pads: Vec<media::media_pad_desc> = zeros_vec(desc.pads);
            enum_links.pads = pads.as_mut_ptr();

            let mut links: Vec<media::media_link_desc> = zeros_vec(desc.links);
            enum_links.links = links.as_mut_ptr();

            ioctl!(fd, media::MEDIA_IOC_ENUM_LINKS, &mut enum_links)?;
            Ok(Self {
                entity,
                pads: pads.into_iter().map(Into::into).collect(),
                links: links.into_iter().map(Into::into).collect(),
            })
        }
    }

    pub fn entity(&self) -> EntityId {
        self.entity
    }

    pub fn pads(&self) -> &[MediaPadDesc] {
        self.pads.as_ref()
    }

    pub fn links(&self) -> &[MediaLinkDesc] {
        self.links.as_ref()
    }
}
