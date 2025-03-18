#[macro_export]
macro_rules! ioctl {
    ($fd:expr, $kind:expr) => {{
        let ret = libc::ioctl($fd.as_raw_fd(), $kind);
        if ret != 0 {
            Err(crate::error::Error::ioctl_error(
                $fd.as_raw_fd(),
                ret,
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
                ret,
                $kind,
            ))
        } else {
            Ok(())
        }
    }};
}
