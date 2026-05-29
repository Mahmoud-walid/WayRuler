use gtk4::prelude::*;
use gtk4::{Button, Picture, ToggleButton};

pub fn icon_button(path: &str) -> ToggleButton {
    let pic = Picture::for_filename(path);
    pic.set_size_request(22, 22);

    let btn = ToggleButton::new();
    btn.set_child(Some(&pic));

    btn
}

pub fn icon_plain_button(path: &str) -> Button {
    let pic = Picture::for_filename(path);
    pic.set_size_request(18, 18);

    let btn = Button::new();
    btn.set_child(Some(&pic));

    btn
}
