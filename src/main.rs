use linux_media as media;

fn main() {
    let info = media::MediaDeviceInfo::from_path("/dev/media0");
    match &info {
        Ok((fd, info)) => println!("info: ({:?},{:?})", fd, info),
        Err(err) => println!("err: {}", err),
    }
    assert!(media::MediaEntity::has_flags(
        info.as_ref().unwrap().1.media_version()
    ));
    let topology = media::MediaTopology::new("/dev/media0");
    match &topology {
        Ok((fd, topology)) => println!("topology: ({:?},{:?})", fd, topology),
        Err(err) => println!("err: {}", err),
    }
}
