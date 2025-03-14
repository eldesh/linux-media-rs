use linux_media as media;

fn main() {
    let path = "/dev/media0";
    let info = media::MediaDeviceInfo::from_path(path);
    match &info {
        Ok((fd, info)) => println!("info: ({:?},{:?})", fd, info),
        Err(err) => println!("err: {}", err),
    }
    let (_info_fd, info) = info.unwrap();
    let topology = media::MediaTopology::new(&info, path);
    match &topology {
        Ok((fd, topology)) => println!("topology: ({:?},{:?})", fd, topology),
        Err(err) => println!("err: {}", err),
    }
}
