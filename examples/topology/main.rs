use std::borrow::Cow;
use std::path::{Path, PathBuf};

use linux_media as media;
use serde_json as json;

fn main() -> media::error::Result<()> {
    let mut args = std::env::args();
    args.next(); // drop program name
    let path = if let Some(path) = args.next() {
        Cow::Owned(PathBuf::from(path))
    } else {
        Cow::Borrowed(Path::new("/dev/media0"))
    };
    println!("path: {}", path.display());

    let media = media::Media::from_path(&path)?;
    let info = media.info();

    println!("info: {}", json::to_string_pretty(&info).unwrap());

    let topology = media::MediaTopology::from_fd(info, media.device_fd())?;
    println!("topology: {}", json::to_string_pretty(&topology).unwrap());

    let es = media::MediaEntityIter::new(
        media.device_fd(),
        media.media_version(),
        topology.entities_slice()[0].id(),
    );
    for e in es {
        println!("entity: {}", json::to_string_pretty(&e).unwrap());
    }

    match media::MediaLinksEnum::new(media.device_fd(), topology.entities_slice()[0].id()) {
        Ok(links) => {
            println!("link: {}", json::to_string_pretty(&links).unwrap());
        }
        Err(err) => {
            println!("err: {:?}", err);
        }
    }
    Ok(())
}
