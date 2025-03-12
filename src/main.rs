use linux_media as media;

fn main() {
    let info = media::MediaDeviceInfo::new("/dev/media0");
    match info {
        Ok(info) => println!("info: {:?}", info),
        Err(err) => println!("err: {}", err),
    }
}
