use std::fs;
use std::path::{Path, PathBuf};

use linux_media as media;
use regex::Regex;

use media::error::trap_io_error;

struct MediaDeviceIterator {
    /// The root directly from which media device searches originates.
    #[allow(unused)]
    path: PathBuf,
    /// A pattern matches driver name of enumerated media device interfaces.
    driver: Regex,
    /// iterator traverses the path and generates file paths of device files.
    iter: Box<dyn Iterator<Item = PathBuf> + 'static>,
}

/// Enumerate paths of media devices that file name contains the specified driver name.
impl Iterator for MediaDeviceIterator {
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if self.driver.is_match(&item.to_string_lossy()) {
                if let Some(file_name) = item.file_name() {
                    return Some(Path::new("/dev").join(file_name));
                }
            }
            dbg!(item.to_string_lossy());
        }
        None
    }
}

impl MediaDeviceIterator {
    pub fn new(driver: Regex) -> media::error::Result<Self> {
        let sysfs = Path::new("/sys/bus/media/devices");
        Self::with_sysfs(sysfs, driver)
    }

    pub fn with_sysfs<P>(sysfs: P, driver: Regex) -> media::error::Result<Self>
    where
        P: AsRef<Path>,
    {
        let sysfs = sysfs.as_ref().to_path_buf();
        let iter = Box::new(
            sysfs
                .as_path()
                .read_dir()
                .map_err(|e| trap_io_error(e, sysfs.clone()))?
                .filter_map(|e| e.ok())
                .filter(|dev| dev.path().is_symlink())
                .filter_map(|dev| fs::read_link(&dev.path()).ok()),
        );

        Ok(Self {
            path: sysfs,
            driver,
            iter,
        })
    }
}

fn media_devices(driver: Regex) -> media::error::Result<MediaDeviceIterator> {
    MediaDeviceIterator::new(driver)
}

fn read_link<P: AsRef<Path>>(path: P) -> media::error::Result<PathBuf> {
    let path = path.as_ref();
    Ok(fs::read_link(path).map_err(|e| trap_io_error(e, path.to_path_buf()))?)
}

fn read_to_string<P: AsRef<Path>>(path: P) -> media::error::Result<String> {
    let path = path.as_ref();
    Ok(fs::read_to_string(path).map_err(|e| trap_io_error(e, path.to_path_buf()))?)
}

fn main() -> media::error::Result<()> {
    let mut args = std::env::args();
    args.next(); // drop program name
    let driver = if let Some(path) = args.next() {
        std::borrow::Cow::Owned(path)
    } else {
        std::borrow::Cow::Borrowed("pisp_be")
    };
    println!("driver: {}", driver);

    for media_node in media_devices(Regex::new(&driver).unwrap())? {
        dbg!(media_node.display());
        let media = media::Media::from_path(&media_node).unwrap();
        let topology = media::MediaTopologyBuilder::new()
            .get_interface()
            .from_fd(media.info(), media.device_fd())?;

        for char_dev in topology
            .interfaces_slice()
            .iter()
            .filter(|intf| intf.r#type() == media::MediaInterfaceType::V4LVideo)
            .map(|intf| intf.devnode().into())
            .filter(|char_dev: &PathBuf| char_dev.is_symlink())
        {
            let dev_name = read_to_string(&dbg!(char_dev.join("name")))?;
            let char_dev_link = read_link(&char_dev)?;
            let dev_node = Path::new("/dev").join(dbg!(&char_dev_link).file_name().unwrap());
            println!("{}\t({})", dev_name, dev_node.display());
        }
    }
    Ok(())
}
