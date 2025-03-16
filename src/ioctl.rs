#[macro_export]
macro_rules! ioctl {
    ($fd:expr, $kind:expr, $arg:expr) => {{
        let ret = libc::ioctl($fd.as_raw_fd(), $kind, $arg);
        if ret != 0 {
            Err(error::Error::Ioctl {
                code: std::io::Error::from_raw_os_error(ret),
                api: $kind,
            })
        } else {
            Ok(())
        }
    }};
}
