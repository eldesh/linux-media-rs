use std::fs;
use std::path::{Path, PathBuf};

use linux_media as media;
use regex::Regex;

use media::error::trap_io_error;

struct MediaDeviceIterator {
    /// The root directly from which media device searches originates.
    #[allow(unused)]
    path: PathBuf,
    /// A pattern matches `model' name of enumerated media device interfaces.
    model: Regex,
    /// iterator traverses the [`path`] and generates file paths of device files.
    iter: Box<dyn Iterator<Item = std::fs::DirEntry> + 'static>,
}

/// Enumerate paths of media devices that file name contains the specified driver name.
impl Iterator for MediaDeviceIterator {
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        let dev = Path::new("/dev");
        while let Some(dir) = self.iter.next() {
            // filter by contents of `$sysfs / mediaN / model`
            if !fs::read_to_string(dir.path().join("model"))
                .map(|m| self.model.is_match(&m))
                .unwrap_or(false)
            {
                dbg!(dir.path());
                continue;
            }

            if let Ok(path) = fs::read_link(&dir.path()) {
                if let Some(file_name) = path.file_name() {
                    return Some(dev.join(file_name));
                }
            }
        }
        None
    }
}

impl MediaDeviceIterator {
    pub fn new(driver: Regex) -> media::error::Result<Self> {
        let sysfs = Path::new("/sys/bus/media/devices");
        Self::with_sysfs(sysfs, driver)
    }

    pub fn with_sysfs<P>(sysfs: P, model: Regex) -> media::error::Result<Self>
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
                .filter(|dir| dir.path().is_symlink()),
        );

        Ok(Self {
            path: sysfs,
            model,
            iter,
        })
    }
}

fn media_devices(model: Regex) -> media::error::Result<MediaDeviceIterator> {
    MediaDeviceIterator::new(model)
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
    let model = if let Some(path) = args.next() {
        std::borrow::Cow::Owned(path)
    } else {
        std::borrow::Cow::Borrowed("pispbe")
    };
    println!("model: {}", model);

    for media_node in media_devices(Regex::new(&model).unwrap())? {
        println!("media: {}", media_node.display());
        let media = media::Media::from_path(&media_node)?;
        let topology = media::MediaTopologyBuilder::new()
            .get_interface()
            .from_media(&media)?;

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
            println!("{}: {}", dev_name.trim_end(), dev_node.display());
        }
    }
    Ok(())
}
