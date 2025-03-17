use derive_more::{Display, From, Into};
use linux_media_sys as media;
use serde::{Deserialize, Serialize};

use crate::error;
use crate::media_entity::EntityId;
use crate::version::Version;

#[derive(
    Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, From, Into, Display, Serialize, Deserialize,
)]
pub struct PadId(u32);

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum MediaPadFlags {
    /// Input pad, relative to the entity. Input pads sink data and are targets of links.
    Sink,
    /// Output pad, relative to the entity. Output pads source data and are origins of links.
    Source,
    /// The pad is an input pad and the pad is linked to any other pad, then at least one of those links must be enabled for the entity to be able to stream. There could be temporary reasons (e.g. device configuration dependent) for the pad to need enabled links even when this flag isn’t set; the absence of the flag doesn’t imply there is none.
    SinkMustConnect,
    /// The pad is an output pad and the pad is linked to any other pad, then at least one of those links must be enabled for the entity to be able to stream. There could be temporary reasons (e.g. device configuration dependent) for the pad to need enabled links even when this flag isn’t set; the absence of the flag doesn’t imply there is none.
    SourceMustConnect,
}

impl TryFrom<u32> for MediaPadFlags {
    type Error = error::Error;
    fn try_from(v: u32) -> error::Result<Self> {
        use MediaPadFlags::*;
        if v & media::MEDIA_PAD_FL_SINK != 0 {
            if v & media::MEDIA_PAD_FL_MUST_CONNECT != 0 {
                Ok(SinkMustConnect)
            } else {
                Ok(Sink)
            }
        } else if v & media::MEDIA_PAD_FL_SOURCE != 0 {
            if v & media::MEDIA_PAD_FL_MUST_CONNECT != 0 {
                Ok(SourceMustConnect)
            } else {
                Ok(Source)
            }
        } else {
            Err(error::Error::PadFlagsParseError { from: v })
        }
    }
}

impl From<MediaPadFlags> for u32 {
    fn from(flags: MediaPadFlags) -> u32 {
        use MediaPadFlags::*;
        match flags {
            Sink => media::MEDIA_PAD_FL_SINK,
            Source => media::MEDIA_PAD_FL_SOURCE,
            SinkMustConnect => media::MEDIA_PAD_FL_SINK | media::MEDIA_PAD_FL_MUST_CONNECT,
            SourceMustConnect => media::MEDIA_PAD_FL_SOURCE | media::MEDIA_PAD_FL_MUST_CONNECT,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct MediaPad {
    /// Unique ID for the pad. Do not expect that the ID will always be the same for each instance of the device. In other words, do not hardcode pad IDs in an application.
    pub id: PadId,
    /// Unique ID for the entity where this pad belongs.
    pub entity_id: EntityId,
    pub flags: MediaPadFlags,
    /// Pad index, starts at 0. Only valid if [has_index(media_version)][MediaPad::has_index] returns true.
    pub index: Option<usize>,
}

impl MediaPad {
    pub fn has_index(media_version: Version) -> bool {
        media::MEDIA_V2_PAD_HAS_INDEX(Into::<u32>::into(media_version).into())
    }

    pub fn from(version: Version, pad: media::media_v2_pad) -> Self {
        Self {
            id: pad.id.into(),
            entity_id: pad.entity_id.into(),
            flags: pad.flags.try_into().unwrap(),
            index: if Self::has_index(version) {
                Some(pad.index as usize)
            } else {
                None
            },
        }
    }
}
