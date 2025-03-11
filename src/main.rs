use linux_media as media;

fn main() {
    let info = media::MediaDeviceInfo::new("/dev/media0");
    println!("info: {:?}", info);
}
