use gtk4::prelude::*;
use gtk4::{Button, Picture, ToggleButton};
use std::path::PathBuf;

pub fn resolve_icon(name: &str) -> PathBuf {
    // dev mode (cargo run)
    let dev_path = PathBuf::from(format!("src/assets/{}", name));
    if dev_path.exists() {
        return dev_path;
    }

    // user install (AUR / local install)
    let system_path = PathBuf::from(format!("/usr/share/wayruler/icons/{}", name));
    if system_path.exists() {
        return system_path;
    }

    // fallback (safe)
    PathBuf::from(format!("/usr/share/pixmaps/{}", name))
}

// ///////////////////////////////

pub fn icon_button(name: &str) -> ToggleButton {
    let pic = Picture::for_filename(resolve_icon(name));
    pic.set_size_request(22, 22);

    let btn = ToggleButton::new();
    btn.set_child(Some(&pic));

    btn
}

pub fn icon_plain_button(name: &str) -> Button {
    let pic = Picture::for_filename(resolve_icon(name));
    pic.set_size_request(18, 18);

    let btn = Button::new();
    btn.set_child(Some(&pic));

    btn
}
