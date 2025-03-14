#[macro_export]
macro_rules! ioctl {
    ($fd:expr, $kind:expr, $arg:expr) => {{
        let ret = libc::ioctl($fd.as_raw_fd(), $kind, $arg);
        if ret != 0 {
            return Err(error::Error::Ioctl {
                code: ret,
                api: $kind,
            });
        }
        Ok(())
    }};
}
