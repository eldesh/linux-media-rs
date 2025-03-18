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

    /// Initializes the request for recycling without re-allocating it.
    ///
    /// # Details
    /// Reinitializes the request, provided that it has not been queued yet or that it has already been queued and completed.
    /// After reinitialization, the request is ready to be queued again for subsequent operations.
    ///
    /// # Errors
    /// If the request is still queued and has not yet completed, this function returns [`error::Error::DeviceIsBusy`]. No other errors are possible.
    pub fn init(&mut self) -> error::Result<()> {
        unsafe { ioctl!(self.request_fd, media::MEDIA_REQUEST_IOC_REINIT) }
    }

    /// Enqueue the request
    ///
    /// # Errors
    /// - `RequestIsAlreadyQueued`   : The request was already queued or the application queued the first buffer directly, but later attempted to use a request. It is not permitted to mix the two APIs.
    /// - `RequestNotContainBuffers` : The request did not contain any buffers. All requests are required to have at least one buffer. This can also be returned if some required configuration is missing in the request.
    /// - `OutOfMemory`              : Out of memory when allocating internal data structures for this request.
    /// - `RequestHasInvalidData`    : The request has invalid data.
    /// - `HardwareBadState`         : The hardware is in a bad state. To recover, the application needs to stop streaming to reset the hardware state and then try to restart streaming.
    pub fn queue(&self) -> error::Result<()> {
        use error::Error::*;
        let api = media::MEDIA_REQUEST_IOC_QUEUE;
        unsafe {
            ioctl!(self.request_fd, api).map_err(|err| {
                let fd = self.request_fd.as_raw_fd();
                if let Ioctl { ref code, .. } = err {
                    match code.raw_os_error() {
                        Some(code @ libc::EBUSY) => RequestIsAlreadyQueued { fd, code, api },
                        Some(code @ libc::ENOENT) => RequestNotContainBuffers { fd, code, api },
                        Some(code @ libc::ENOMEM) => OutOfMemory { fd, code, api },
                        Some(code @ libc::EINVAL) => RequestHasInvalidData { fd, code, api },
                        Some(code @ libc::EIO) => HardwareBadState { fd, code, api },
                        _ => err,
                    }
                } else {
                    err
                }
            })
        }
    }
}
