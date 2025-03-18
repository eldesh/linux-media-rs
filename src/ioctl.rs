/// A wrapper macro of ioctl.
/// If the calling ioctl returned -1, it returns [`crate::error::Error`] corresponding to the errno.
#[macro_export]
macro_rules! ioctl {
    ($fd:expr, $kind:expr) => {{
        let ret = libc::ioctl($fd.as_raw_fd(), $kind);
        if ret != 0 {
            Err(crate::error::Error::ioctl_error(
                $fd.as_raw_fd(),
                std::io::Error::last_os_error().raw_os_error().unwrap(),
                $kind,
            ))
        } else {
            Ok(())
        }
    }};
    ($fd:expr, $kind:expr, $arg:expr) => {{
        let ret = libc::ioctl($fd.as_raw_fd(), $kind, $arg);
        if ret != 0 {
            Err(crate::error::Error::ioctl_error(
                $fd.as_raw_fd(),
                std::io::Error::last_os_error().raw_os_error().unwrap(),
                $kind,
            ))
        } else {
            Ok(())
        }
    }};
}
