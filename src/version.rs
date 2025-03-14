use std::fmt;

/// Version information wrapper formatted with `KERNEL_VERSION` macro.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl From<u32> for Version {
    /// Convert to u32.
    ///
    /// 17..24th bits, 9..16th bits, 0..8th bits represents major, minor and patch version respectively.
    fn from(ver: u32) -> Self {
        let major = ((ver & (0xFF << 16)) >> 16) as u8;
        let minor = ((ver & (0xFF << 8)) >> 8) as u8;
        let patch = ((ver & (0xFF << 0)) >> 0) as u8;
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl Into<u32> for Version {
    fn into(self: Version) -> u32 {
        let Self {
            major,
            minor,
            patch,
        } = self;
        ((major as u32) << 16) | ((minor as u32) << 8) | (patch as u32)
    }
}

impl fmt::Display for Version {
    /// Version is formatted to "{major}.{minor}.{patch}".
    /// Where the each component is formatted as decimal number.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}
