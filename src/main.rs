use std::borrow::Cow;
use std::os::fd::AsFd;

use linux_media as media;
use serde_json as json;

fn main() {
    let mut args = std::env::args();
    args.next(); // drop program name
    let path = if let Some(path) = args.next() {
        Cow::Owned(path)
    } else {
        Cow::Borrowed("/dev/media0")
    };
    println!("path: {}", path.as_ref());

    let info = media::MediaDeviceInfo::from_path(&path.as_ref());
    match &info {
        Ok((fd, info)) => {
            println!("info: {:?}", fd);
            println!("{}", json::to_string_pretty(&info).unwrap());
        }
        Err(err) => println!("err: {}", err),
    }
    let (info_fd, info) = info.unwrap();
    let topology = media::MediaTopology::new(&info, &path.as_ref());
    match &topology {
        Ok((fd, topology)) => {
            println!("topology: {:?}", fd);
            println!("{}", json::to_string_pretty(&topology).unwrap());
        }
        Err(err) => println!("err: {}", err),
    }
    let (topo_fd, topology) = topology.unwrap();

    let es = media::MediaEntityIter::new(
        info_fd.as_fd(),
        info.media_version,
        topology.entities()[0].id(),
    );
    for e in es {
        println!("entity: {}", json::to_string_pretty(&e).unwrap());
    }

    match media::MediaLinksEnum::new(topo_fd, topology.entities()[0].id()) {
        Ok(links) => {
            println!("link: {}", json::to_string_pretty(&links).unwrap());
        }
        Err(err) => {
            println!("err: {:?}", err);
        }
    }
}
