use std::fs::OpenOptions;
use std::os::fd::{AsFd, AsRawFd, OwnedFd};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::ptr::null;

use crate::error::{self, Result};
use crate::ioctl;
use crate::Media;
use crate::MediaDeviceInfo;
use crate::MediaEntity;
use crate::MediaPad;
use crate::MediaTopology;

use linux_media_sys as media;

/// A type for constructing [`MediaTopology`] using builder pattern.
///
/// # Details
/// This type helps users to construct instances of [`MediaTopology`] with only needed fields.
/// This makes reducing memory allocation and size of constructed objects.
///
/// # Examples
/// In the following example, `topology` only have interface objects using [`MediaTopologyBuilder`].
/// ```
/// use linux_media::*;
/// # fn main () -> error::Result<()> {
/// if let Ok(media) = Media::from_path("/dev/media0") {
///     let topology = MediaTopologyBuilder::new()
///         .get_interface()
///         .from_fd(media.info(), media.device_fd())?;
///     assert!(matches!(topology.interfaces(), Some(_)));
///     assert_eq!(topology.entities(), None);
///     assert_eq!(topology.pads(), None);
///     assert_eq!(topology.links(), None);
/// }
/// # Ok(())
/// # }
/// ```
///
/// Calling full options of builder, constructed topology is equals to the instance constructed with [`MediaTopology::from_path`][crate::MediaTopology::from_path] or [`MediaTopology::from_fd`][crate::MediaTopology::from_fd].
///
/// ```
/// use linux_media::*;
/// # fn main () -> error::Result<()> {
/// if let Ok(media) = Media::from_path("/dev/media0") {
///     let topologyA = MediaTopologyBuilder::new()
///         .get_entity()
///         .get_interface()
///         .get_pad()
///         .get_link()
///         .from_fd(media.info(), media.device_fd())?;
///     assert!(matches!(topologyA.interfaces(), Some(_)));
///     assert!(matches!(topologyA.entities(), Some(_)));
///     assert!(matches!(topologyA.pads(), Some(_)));
///     assert!(matches!(topologyA.links(), Some(_)));
///     let topologyB = MediaTopology::from_fd(media.info(), media.device_fd())?;
///     assert_eq!(topologyA, topologyB);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct MediaTopologyBuilder {
    entities: bool,
    interfaces: bool,
    links: bool,
    pads: bool,
}

fn zeros_vec<T>(num: u32) -> Vec<T>
where
    T: Clone,
{
    let mut xs = vec![];
    xs.resize(num as usize, unsafe { std::mem::zeroed() });
    xs
}

impl MediaTopologyBuilder {
    pub fn new() -> Self {
        Self {
            entities: false,
            interfaces: false,
            links: false,
            pads: false,
        }
    }

    /// Enable inclusion of entities in the [`MediaTopology`].
    ///
    /// # Details
    /// Calling this method instructs the builder to include the entities as part of the [`MediaTopology`].
    pub fn get_entity(&mut self) -> &mut Self {
        self.entities = true;
        self
    }

    /// Enable inclusion of interfaces in the [`MediaTopology`].
    ///
    /// # Details
    /// Calling this method instructs the builder to include the interfaces as part of the [`MediaTopology`].
    pub fn get_interface(&mut self) -> &mut Self {
        self.interfaces = true;
        self
    }

    /// Enable inclusion of links in the [`MediaTopology`].
    ///
    /// # Details
    /// Calling this method instructs the builder to include the links as part of the [`MediaTopology`].
    pub fn get_link(&mut self) -> &mut Self {
        self.links = true;
        self
    }

    /// Enable inclusion of pads in the [`MediaTopology`].
    ///
    /// # Details
    /// Calling this method instructs the builder to include the pads as part of the [`MediaTopology`].
    pub fn get_pad(&mut self) -> &mut Self {
        self.pads = true;
        self
    }

    /// Construct an instance of [`MediaTopology`] includes items specified with builder methods.
    ///
    /// # Details
    /// Construct an instance of [`MediaTopology`] that
    /// includes only items specified [`get_entity`][Self::get_entity], [`get_interface`][Self::get_interface], [`get_link`][Self::get_link] or [`get_pad`][Self::get_pad].
    ///
    /// # Parameters
    ///
    /// * `info`: A reference to a [`MediaDeviceInfo`] containing the [`media_version`][crate::MediaDeviceInfo::media_version] used to build the topology.
    /// * `fd`: A file descriptor referring to the media device file from which `info` was obtained.
    ///
    /// # Returns
    /// A Result containing the constructed [`MediaTopology`] if successful, or an error otherwise.
    pub fn from_fd<F>(self, info: &MediaDeviceInfo, fd: F) -> Result<MediaTopology>
    where
        F: AsFd,
    {
        let mut topology: media::media_v2_topology = unsafe {
            let mut topology: media::media_v2_topology = std::mem::zeroed();
            ioctl!(fd.as_fd(), media::MEDIA_IOC_G_TOPOLOGY, &mut topology)?;
            topology
        };
        let version = topology.topology_version;

        let entities: Vec<media::media_v2_entity>;
        if self.entities {
            entities = zeros_vec(topology.num_entities);
            topology.ptr_entities = entities.as_ptr() as media::__u64;
        } else {
            entities = vec![];
            topology.ptr_entities = null::<media::media_v2_entity>() as media::__u64;
        }

        let interfaces: Vec<media::media_v2_interface>;
        if self.interfaces {
            interfaces = zeros_vec(topology.num_interfaces);
            topology.ptr_interfaces = interfaces.as_ptr() as media::__u64;
        } else {
            interfaces = vec![];
            topology.ptr_interfaces = null::<media::media_v2_interface>() as media::__u64;
        }

        let links: Vec<media::media_v2_link>;
        if self.links {
            links = zeros_vec(topology.num_links);
            topology.ptr_links = links.as_ptr() as media::__u64;
        } else {
            links = vec![];
            topology.ptr_links = null::<media::media_v2_link>() as media::__u64;
        }

        let pads: Vec<media::media_v2_pad>;
        if self.pads {
            pads = zeros_vec(topology.num_pads);
            topology.ptr_pads = pads.as_ptr() as media::__u64;
        } else {
            pads = vec![];
            topology.ptr_pads = null::<media::media_v2_pad>() as media::__u64;
        }

        unsafe {
            // Second ioctl call with allocated space to
            // populate the entities/interface/links/pads array.
            ioctl!(fd.as_fd(), media::MEDIA_IOC_G_TOPOLOGY, &mut topology)?;
        };
        assert_eq!(version, { topology.topology_version });

        Ok(MediaTopology::new(
            None,
            topology.topology_version,
            self.entities.then_some(
                entities
                    .into_iter()
                    .map(|ent| MediaEntity::from_raw_entity(info.media_version(), ent))
                    .collect(),
            ),
            self.interfaces
                .then_some(interfaces.into_iter().map(Into::into).collect()),
            self.pads.then_some(
                pads.into_iter()
                    .map(|pad| MediaPad::from(info.media_version(), pad))
                    .collect(),
            ),
            self.links
                .then_some(links.into_iter().map(Into::into).collect()),
        ))
    }

    /// Construct an instance of [`MediaTopology`] from device file.
    ///
    /// # Details
    /// Construct an instance of [`MediaTopology`] from device file that specified with path.
    ///
    /// # Parameters
    ///
    /// * `info`: A reference to a [`MediaDeviceInfo`] containing the [`media_version`][crate::MediaDeviceInfo::media_version] used to build the topology.
    /// * `path`: A reference to a device file containing the infomation of media topology.
    ///
    /// # Returns
    /// A Result with file descriptor to device file and constructed [`MediaTopology`] if successful, or an error otherwise.
    pub fn from_path<P>(self, info: &MediaDeviceInfo, path: P) -> Result<(OwnedFd, MediaTopology)>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_path_buf();
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_CLOEXEC)
            .open(&path)
            .map_err(|err| error::trap_io_error(err, path.clone()))?;
        let owned_fd = OwnedFd::from(file);
        let topo = self.from_fd(info, &owned_fd)?;
        Ok((owned_fd, topo))
    }

    /// Construct an instance of [`MediaTopology`] from [`Media`].
    ///
    /// # Details
    /// Construct an instance of [`MediaTopology`] from [`Media`] that
    /// includes only items specified [`get_entity`][Self::get_entity], [`get_interface`][Self::get_interface], [`get_link`][Self::get_link] or [`get_pad`][Self::get_pad].
    ///
    /// # Parameters
    ///
    /// * `media`: A reference to a [`Media`] containing the [`media_info`][crate::Media::info] and
    /// [`device_fd`][crate::Media::device_fd`].
    ///
    /// # Returns
    /// A Result containing the constructed [`MediaTopology`] if successful, or an error otherwise.
    pub fn from_media(&self, media: &Media) -> Result<MediaTopology> {
        self.from_fd(media.info(), media.device_fd())
    }
}
