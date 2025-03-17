use std::os::fd::{AsFd, AsRawFd};

use linux_media_sys as media;
use serde::{Deserialize, Serialize};

use crate::error;
use crate::ioctl;
use crate::MediaLinkFlags;
use crate::MediaPadDesc;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct MediaLinkDesc {
    source: MediaPadDesc,
    sink: MediaPadDesc,
    flags: MediaLinkFlags,
}

impl MediaLinkDesc {
    pub fn new(source: MediaPadDesc, sink: MediaPadDesc, flags: MediaLinkFlags) -> Self {
        Self {
            source,
            sink,
            flags,
        }
    }

    /// Pad at the origin of this link.
    pub fn source(&self) -> &MediaPadDesc {
        &self.source
    }

    /// Pad at the target of this link.
    pub fn sink(&self) -> &MediaPadDesc {
        &self.sink
    }

    /// Link flags
    pub fn flags(&self) -> MediaLinkFlags {
        self.flags
    }

    pub fn setup<F>(&mut self, fd: F, flags: MediaLinkFlags) -> error::Result<()>
    where
        F: AsFd,
    {
        unsafe {
            let mut desc: linux_media_sys::media_link_desc = self.clone().into();
            desc.flags = flags.bits();
            ioctl!(fd.as_fd(), media::MEDIA_IOC_SETUP_LINK, &mut desc)?;
            *self = desc.into();
            Ok(())
        }
    }
}

impl From<media::media_link_desc> for MediaLinkDesc {
    fn from(desc: media::media_link_desc) -> Self {
        assert!({
            let link_type = desc.flags & media::MEDIA_LNK_FL_LINK_TYPE;
            (link_type == media::MEDIA_LNK_FL_DATA_LINK) ||
            (link_type == media::MEDIA_LNK_FL_ANCILLARY_LINK)
          },
          "The link type of MediaLinkDesc must be either DATA_LINK or ANCILLARY_LINK, but got flags: {:#x}",
          desc.flags
        );
        Self {
            source: desc.source.into(),
            sink: desc.sink.into(),
            flags: desc.flags.try_into().unwrap(),
        }
    }
}

impl From<MediaLinkDesc> for media::media_link_desc {
    fn from(desc: MediaLinkDesc) -> media::media_link_desc {
        let mut raw: linux_media_sys::media_link_desc = unsafe { std::mem::zeroed() };
        raw.source = desc.source.into();
        raw.sink = desc.sink.into();
        raw.flags = desc.flags.bits();
        raw
    }
}
