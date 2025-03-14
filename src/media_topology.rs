use std::fs::OpenOptions;
use std::os::fd::{AsRawFd, OwnedFd};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use linux_media_sys as media;

use crate::error::{self, Result};
use crate::ioctl;
use crate::media_device_info::MediaDeviceInfo;
use crate::media_entity::MediaEntity;
use crate::media_interface::MediaInterface;
use crate::media_link::MediaLink;
use crate::media_pad::MediaPad;

/// Wrapper of media_v2_topology.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MediaTopology {
    /// Device file from which topology information is read
    path: PathBuf,
    version: u64,
    entities: Vec<MediaEntity>,
    interfaces: Vec<MediaInterface>,
    pads: Vec<MediaPad>,
    links: Vec<MediaLink>,
}

fn zeros_vec<T>(num: u32) -> Vec<T>
where
    T: Clone,
{
    let mut xs = vec![];
    xs.resize(num as usize, unsafe { std::mem::zeroed() });
    xs
}

impl MediaTopology {
    /// Construct topology from the given device file such like: /dev/mediaX
    pub fn new<P>(info: &MediaDeviceInfo, path: P) -> Result<(OwnedFd, Self)>
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
        let mut topology: media::media_v2_topology = unsafe {
            let mut topology: media::media_v2_topology = std::mem::zeroed();
            ioctl!(owned_fd, media::MEDIA_IOC_G_TOPOLOGY, &mut topology)?;
            topology
        };
        let version = topology.topology_version;

        let entities: Vec<media::media_v2_entity> = zeros_vec(topology.num_entities);
        topology.ptr_entities = entities.as_ptr() as media::__u64;

        let interfaces: Vec<media::media_v2_interface> = zeros_vec(topology.num_interfaces);
        topology.ptr_interfaces = interfaces.as_ptr() as media::__u64;

        let links: Vec<media::media_v2_link> = zeros_vec(topology.num_links);
        topology.ptr_links = links.as_ptr() as media::__u64;

        let pads: Vec<media::media_v2_pad> = zeros_vec(topology.num_pads);
        topology.ptr_pads = pads.as_ptr() as media::__u64;

        unsafe {
            // Second ioctl call with allocated space to
            // populate the entities/interface/links/pads array.
            ioctl!(owned_fd, media::MEDIA_IOC_G_TOPOLOGY, &mut topology)?;
        };
        assert_eq!(version, { topology.topology_version });

        Ok((
            owned_fd,
            Self {
                path,
                version: topology.topology_version,
                entities: entities
                    .into_iter()
                    .map(|ent| MediaEntity::from_raw_entity(info.media_version, ent))
                    .collect(),
                interfaces: interfaces.into_iter().map(Into::into).collect(),
                pads: pads
                    .into_iter()
                    .map(|pad| MediaPad::from(info.media_version, pad))
                    .collect(),
                links: links.into_iter().map(Into::into).collect(),
            },
        ))
    }
}
