use linux_media as media;

fn main() {
    let info = media::MediaDeviceInfo::new("/dev/media0");
    match &info {
        Ok(info) => println!("info: {:?}", info),
        Err(err) => println!("err: {}", err),
    }
    println!("version: {}", info.as_ref().unwrap().media_version());
    assert!(media::MediaEntity::has_flags(info.unwrap().media_version()));
    let topology = media::MediaTopology::new("/dev/media0");
    match &topology {
        Ok(topology) => println!("topology: {:?}", topology),
        Err(err) => println!("err: {}", err),
    }
}
