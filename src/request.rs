use std::os::fd::{AsRawFd, BorrowedFd, FromRawFd, OwnedFd};

use linux_media_sys as media;

use crate::error;
use crate::ioctl;

/// A request associated with a media device.
///
/// # Details
/// This is a wrapper for the media control API's request, which is tied to a specific media device.
#[derive(Debug)]
pub struct Request<'a> {
    /// The file descriptor of the media device from which the request was allocated.
    media_fd: BorrowedFd<'a>,
    /// The file descriptor corresponding to the request allocated on the media device (referenced by media_fd).
    request_fd: OwnedFd,
}

impl<'a> Request<'a> {
    pub fn new(media_fd: BorrowedFd<'a>) -> error::Result<Self> {
        unsafe {
            let mut request_fd: libc::c_int = -1;
            ioctl!(media_fd, media::MEDIA_IOC_REQUEST_ALLOC, &mut request_fd)?;
            Ok(Self {
                media_fd,
                request_fd: OwnedFd::from_raw_fd(request_fd),
            })
        }
    }

    /// Allocate a new request on the same media device
    pub fn new_request(&self) -> error::Result<Self> {
        Self::new(self.media_fd)
    }

    /// Initialize the request
    ///
    /// # Details
    /// Initialize the request if it either has not been queued yet, or if it was queued and completed. Otherwise it will set errno to EBUSY. No other error codes can be returned.
    pub fn init(&mut self) -> error::Result<()> {
        unsafe { ioctl!(self.request_fd, media::MEDIA_REQUEST_IOC_REINIT) }
    }
}

