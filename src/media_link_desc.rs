use linux_media_sys as media;
use serde::{Deserialize, Serialize};

use crate::MediaLinkFlags;
use crate::MediaPadDesc;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct MediaLinkDesc {
    source: MediaPadDesc,
    sink: MediaPadDesc,
    flags: MediaLinkFlags,
}

impl MediaLinkDesc {
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
}

impl From<media::media_link_desc> for MediaLinkDesc {
    fn from(desc: media::media_link_desc) -> Self {
        assert_eq!(
            desc.flags & media::MEDIA_LNK_FL_LINK_TYPE,
            media::MEDIA_LNK_FL_DATA_LINK
        );
        Self {
            source: desc.source.into(),
            sink: desc.sink.into(),
            flags: desc.flags.try_into().unwrap(),
        }
    }
}
