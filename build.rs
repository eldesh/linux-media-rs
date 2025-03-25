use autocfg;

fn main() {
    let cfg = autocfg::new();
    cfg.emit_has_path("linux_media_sys::MEDIA_LNK_FL_ANCILLARY_LINK");
    cfg.emit_has_path("linux_media_sys::MEDIA_ENT_F_CONN_RF");
    cfg.emit_has_path("linux_media_sys::MEDIA_ENT_F_CONN_SVIDEO");
    cfg.emit_has_path("linux_media_sys::MEDIA_ENT_F_CONN_COMPOSITE");
}
