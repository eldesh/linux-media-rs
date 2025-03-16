use std::marker::PhantomData;

use derive_more::{From, Into};
use linux_media_sys as media;

use crate::error;
use crate::media_entity::EntityId;
use crate::media_interface::InterfaceId;
use crate::media_pad::PadId;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, From, Into)]
pub struct LinkId(u32);

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
    pub struct MediaLinkFlags: u32 {
        /// The link is enabled and can be used to transfer media data. When two or more links target a sink pad, only one of them can be enabled at a time.
        const Enabled = media::MEDIA_LNK_FL_ENABLED;
        /// The link enabled state can’t be modified at runtime. An immutable link is always enabled.
        const Immutable = media::MEDIA_LNK_FL_IMMUTABLE;
        /// The link enabled state can be modified during streaming. This flag is set by drivers and is read-only for applications.
        const Dynamic = media::MEDIA_LNK_FL_DYNAMIC;
    }
}

impl TryFrom<u32> for MediaLinkFlags {
    type Error = error::Error;
    fn try_from(v: u32) -> error::Result<Self> {
        MediaLinkFlags::from_bits(v & !media::MEDIA_LNK_FL_LINK_TYPE)
            .ok_or_else(|| error::Error::LinkTypeParseError { from: v })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct PadIdOr<T>(u32, PhantomData<T>);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum LinkType {
    /// MEDIA_LNK_FL_DATA_LINK
    /// On pad to pad links: unique IDs for the source/sink pad.
    DataLink { source_id: PadId, sink_id: PadId },
    /// MEDIA_LNK_FL_INTERFACE_LINK
    /// On interface to entity links: unique IDs for the interface/entity.
    InterfaceLink {
        source_id: InterfaceId,
        sink_id: EntityId,
    },
    /// MEDIA_LNK_FL_ANCILLARY_LINK for links that represent a physical relationship between two entities. The link may or may not be immutable, so applications must not assume either case.
    AncillaryLink {
        source_id: PadIdOr<InterfaceId>,
        sink_id: PadIdOr<EntityId>,
    },
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MediaLink {
    /// Unique ID for the link. Do not expect that the ID will always be the same for each instance of the device. In other words, do not hardcode link IDs in an application.
    pub id: LinkId,
    pub r#type: LinkType,
    pub flags: MediaLinkFlags,
}

impl MediaLink {
    pub fn new(id: LinkId, r#type: LinkType, flags: MediaLinkFlags) -> Self {
        Self { id, r#type, flags }
    }

    pub fn id(&self) -> LinkId {
        self.id
    }
}

impl From<media::media_v2_link> for MediaLink {
    fn from(link: media::media_v2_link) -> Self {
        let r#type = match link.flags & media::MEDIA_LNK_FL_LINK_TYPE {
            media::MEDIA_LNK_FL_DATA_LINK => LinkType::DataLink {
                source_id: link.source_id.into(),
                sink_id: link.sink_id.into(),
            },
            media::MEDIA_LNK_FL_INTERFACE_LINK => LinkType::InterfaceLink {
                source_id: link.source_id.into(),
                sink_id: link.sink_id.into(),
            },
            media::MEDIA_LNK_FL_ANCILLARY_LINK => LinkType::AncillaryLink {
                source_id: PadIdOr(link.source_id, PhantomData),
                sink_id: PadIdOr(link.sink_id, PhantomData),
            },
            other => unreachable!("link type should not be there: {}", other),
        };
        Self {
            id: link.id.into(),
            r#type,
            flags: link.flags.try_into().unwrap(),
        }
    }
}
