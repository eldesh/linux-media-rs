use std::fs::OpenOptions;
use std::os::fd::{AsFd, OwnedFd};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{self, Result};
use crate::media_device_info::MediaDeviceInfo;
use crate::media_entity::MediaEntity;
use crate::media_interface::MediaInterface;
use crate::media_link::MediaLink;
use crate::media_pad::MediaPad;
use crate::media_topology_builder::MediaTopologyBuilder;

/// Rust representation of the [`media_v2_topology`][linux_media_sys::media_v2_topology] type.
///
/// # Details
/// Captures a media device’s topology as defined by the Linux media controller API,
/// including its version, optional device file path (if built from a path), and collections of entities, interfaces, pads, and links.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct MediaTopology {
    /// If the instance was built with a file path given, the device file path from which topology information was read, otherwise None if it was built from a file descriptor.
    path: Option<PathBuf>,
    version: u64,
    entities: Option<Vec<MediaEntity>>,
    interfaces: Option<Vec<MediaInterface>>,
    pads: Option<Vec<MediaPad>>,
    links: Option<Vec<MediaLink>>,
}

impl MediaTopology {
    /// Construct a [`MediaTopology`].
    /// This function is provided solely for use by [`MediaTopologyBuilder`].
    pub(crate) fn new(
        path: Option<PathBuf>,
        version: u64,
        entities: Option<Vec<MediaEntity>>,
        interfaces: Option<Vec<MediaInterface>>,
        pads: Option<Vec<MediaPad>>,
        links: Option<Vec<MediaLink>>,
    ) -> Self {
        Self {
            path,
            version,
            entities,
            interfaces,
            pads,
            links,
        }
    }

    /// Constructs a MediaTopology from the given device file such like: /dev/mediaX
    ///
    /// # Details
    /// Constructs a MediaTopology from the media device file.
    ///
    /// * `info`: The device info including [`media_version`][crate::MediaDeviceInfo#structfield.media_version].
    /// * `path`: The path to the device file from which topology information is read.
    ///
    /// # Returns
    /// A Result containing the constructed MediaTopology if successful, or an error otherwise.
    pub fn from_path<P>(info: &MediaDeviceInfo, path: P) -> Result<(OwnedFd, Self)>
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
        let mut topo = Self::from_fd(info, owned_fd.as_fd())?;
        topo.path = Some(path);
        Ok((owned_fd, topo))
    }

    /// Constructs a MediaTopology from a file descriptor.
    ///
    /// # Details
    /// Constructs a MediaTopology from a file descriptor referencing a device file (e.g., /dev/mediaX).
    ///
    /// * `info`: A reference to a [`MediaDeviceInfo`] containing the [`media_version`][crate::MediaDeviceInfo::media_version] used to build the topology.
    /// * `fd`: A file descriptor referring to the media device file from which `info` was obtained.
    ///
    /// # Returns
    /// A Result containing the constructed [`MediaTopology`] if successful, or an error otherwise.
    pub fn from_fd<F>(info: &MediaDeviceInfo, fd: F) -> Result<Self>
    where
        F: AsFd,
    {
        MediaTopologyBuilder::new()
            .get_entity()
            .get_interface()
            .get_pad()
            .get_link()
            .from_fd(info, fd)
    }

    pub fn entities_slice(&self) -> &[MediaEntity] {
        self.entities.as_deref().unwrap_or(&[])
    }

    pub fn interfaces_slice(&self) -> &[MediaInterface] {
        self.interfaces.as_deref().unwrap_or(&[])
    }

    pub fn pads_slice(&self) -> &[MediaPad] {
        self.pads.as_deref().unwrap_or(&[])
    }

    pub fn links_slice(&self) -> &[MediaLink] {
        self.links.as_deref().unwrap_or(&[])
    }

    pub fn entities(&self) -> Option<&[MediaEntity]> {
        self.entities.as_deref()
    }

    pub fn interfaces(&self) -> Option<&[MediaInterface]> {
        self.interfaces.as_deref()
    }

    pub fn pads(&self) -> Option<&[MediaPad]> {
        self.pads.as_deref()
    }

    pub fn links(&self) -> Option<&[MediaLink]> {
        self.links.as_deref()
    }
}
