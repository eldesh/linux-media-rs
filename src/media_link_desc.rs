use linux_media_sys as media;

use crate::MediaLinkProperty;
use crate::MediaPadDesc;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MediaLinkDesc {
    /// Pad at the origin of this link.
    source: MediaPadDesc,
    /// Pad at the target of this link.
    sink: MediaPadDesc,
    property: MediaLinkProperty,
}

impl MediaLinkDesc {
    pub fn source(&self) -> &MediaPadDesc {
        &self.source
    }

    pub fn sink(&self) -> &MediaPadDesc {
        &self.sink
    }

    pub fn property(&self) -> MediaLinkProperty {
        self.property
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
            property: desc.flags.try_into().unwrap(),
        }
    }
}
