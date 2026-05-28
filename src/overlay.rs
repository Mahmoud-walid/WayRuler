// src/overlay.rs

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, DrawingArea, EventControllerMotion, EventControllerScroll, gdk};
use gtk4_layer_shell::{Layer, LayerShell};
use std::cell::RefCell;
use std::rc::Rc;
use std::path::PathBuf;
use crate::edge_detector::{EdgeEngine, Axis};

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Bounds,
    Crosshair,
    Horizontal,
    Vertical,
}

struct AppState {
    cursor_x: f64,
    cursor_y: f64,
    skip_index: usize,
    mode: Mode,
}

fn get_config_path() -> PathBuf {
    // save the mode to ~/.config/wayruler_mode.txt
    let mut path = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| String::from(".")));
    path.push(".config");
    path.push("wayruler_mode.txt");
    path
}

fn load_saved_mode() -> Mode {
    if let Ok(content) = std::fs::read_to_string(get_config_path()) {
        match content.trim() {
            "Bounds" => Mode::Bounds,
            "Crosshair" => Mode::Crosshair,
            "Horizontal" => Mode::Horizontal,
            "Vertical" => Mode::Vertical,
            _ => Mode::Horizontal,
        }
    } else {
        Mode::Horizontal
    }
}

fn save_mode(mode: Mode) {
    let name = match mode {
        Mode::Bounds => "Bounds",
        Mode::Crosshair => "Crosshair",
        Mode::Horizontal => "Horizontal",
        Mode::Vertical => "Vertical",
    };
    let _ = std::fs::write(get_config_path(), name);
}

pub fn build_ui(app: &Application, screenshot: image::RgbaImage) {
    let provider = gtk4::CssProvider::new();
    let css = "
        window { background-color: transparent; }
        .toolbar-box {
            background-color: #f3f3f3;
            border-radius: 8px;
            padding: 6px;
            margin-top: 20px;
            box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.3);
        }
        .toolbar-btn {
            border: none;
            background: transparent;
            font-size: 22px;
            border-radius: 6px;
            padding: 4px 14px;
            color: #333;
        }
        .toolbar-btn:checked {
            background-color: #2b747e;
            color: white;
        }
        .close-btn {
            border: none;
            background: transparent;
            font-size: 18px;
            color: #d00;
            padding: 4px 12px;
        }
        .close-btn:hover {
            background: #fee;
        }
    ";
    provider.load_from_data(css);
    gtk4::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let window = ApplicationWindow::builder()
        .application(app)
        .title("WayRuler Overlay")
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_exclusive_zone(-1);
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);
    window.set_anchor(gtk4_layer_shell::Edge::Top, true);
    window.set_anchor(gtk4_layer_shell::Edge::Bottom, true);
    window.set_anchor(gtk4_layer_shell::Edge::Left, true);
    window.set_anchor(gtk4_layer_shell::Edge::Right, true);

    let initial_mode = load_saved_mode();

    let state = Rc::new(RefCell::new(AppState {
        cursor_x: 0.0,
        cursor_y: 0.0,
        skip_index: 0,
        mode: initial_mode,
    }));

    let overlay_container = gtk4::Overlay::new();
    let drawing_area = DrawingArea::new();
    overlay_container.set_child(Some(&drawing_area));

    let toolbar = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);
    toolbar.add_css_class("toolbar-box");
    toolbar.set_halign(gtk4::Align::Center);
    toolbar.set_valign(gtk4::Align::Start);

    let btn_bounds = gtk4::ToggleButton::with_label("⛶");
    let btn_cross = gtk4::ToggleButton::with_label("┼");
    let btn_horiz = gtk4::ToggleButton::with_label("⟷");
    let btn_vert = gtk4::ToggleButton::with_label("↕");

    btn_cross.set_group(Some(&btn_bounds));
    btn_horiz.set_group(Some(&btn_bounds));
    btn_vert.set_group(Some(&btn_bounds));

    btn_bounds.add_css_class("toolbar-btn");
    btn_cross.add_css_class("toolbar-btn");
    btn_horiz.add_css_class("toolbar-btn");
    btn_vert.add_css_class("toolbar-btn");

    match initial_mode {
        Mode::Bounds => btn_bounds.set_active(true),
        Mode::Crosshair => btn_cross.set_active(true),
        Mode::Horizontal => btn_horiz.set_active(true),
        Mode::Vertical => btn_vert.set_active(true),
    }

    let state_btn = state.clone();
    let draw_btn = drawing_area.clone();
    let setup_btn = |btn: &gtk4::ToggleButton, mode: Mode| {
        let s = state_btn.clone();
        let d = draw_btn.clone();
        btn.connect_toggled(move |b| {
            if b.is_active() {
                s.borrow_mut().mode = mode;
                s.borrow_mut().skip_index = 0;
                save_mode(mode);
                d.queue_draw();
            }
        });
    };

    setup_btn(&btn_bounds, Mode::Bounds);
    setup_btn(&btn_cross, Mode::Crosshair);
    setup_btn(&btn_horiz, Mode::Horizontal);
    setup_btn(&btn_vert, Mode::Vertical);

    let btn_close = gtk4::Button::with_label("✕");
    btn_close.add_css_class("close-btn");
    let win_clone = window.clone();
    btn_close.connect_clicked(move |_| {
        win_clone.close();
    });

    toolbar.append(&btn_bounds);
    toolbar.append(&btn_cross);
    toolbar.append(&btn_horiz);
    toolbar.append(&btn_vert);

    let separator = gtk4::Separator::new(gtk4::Orientation::Vertical);
    separator.set_margin_top(5);
    separator.set_margin_bottom(5);
    toolbar.append(&separator);
    
    toolbar.append(&btn_close);

    overlay_container.add_overlay(&toolbar);
    window.set_child(Some(&overlay_container));

    let engine = Rc::new(EdgeEngine::new(screenshot));

    let motion = EventControllerMotion::new();
    let state_clone = state.clone();
    let area_clone = drawing_area.clone();
    motion.connect_motion(move |_, x, y| {
        let mut s = state_clone.borrow_mut();
        s.cursor_x = x;
        s.cursor_y = y;
        s.skip_index = 0;
        area_clone.queue_draw();
    });

    let scroll = EventControllerScroll::new(gtk4::EventControllerScrollFlags::VERTICAL);
    let state_scroll = state.clone();
    let area_scroll = drawing_area.clone();
    scroll.connect_scroll(move |_, _dx, dy| {
        let mut s = state_scroll.borrow_mut();
        if dy > 0.0 {
            s.skip_index += 1; 
        } else if dy < 0.0 && s.skip_index > 0 {
            s.skip_index -= 1;
        }
        area_scroll.queue_draw();
        glib::Propagation::Stop
    });

    drawing_area.add_controller(motion);
    drawing_area.add_controller(scroll);

    let engine_draw = engine.clone();
    let state_draw = state.clone();
    drawing_area.set_draw_func(move |_, cr, width, height| {
        let s = state_draw.borrow();
        let cx = s.cursor_x;
        let cy = s.cursor_y;

        cr.set_source_rgba(0.0, 0.0, 0.0, 0.2);
        cr.paint().unwrap();

        let (neg_x, pos_x) = engine_draw.find_edges(cx as u32, cy as u32, Axis::Horizontal);
        let (neg_y, pos_y) = engine_draw.find_edges(cx as u32, cy as u32, Axis::Vertical);

        let left_edge = neg_x.get(s.skip_index).copied().unwrap_or(0) as f64;
        let right_edge = pos_x.get(s.skip_index).copied().unwrap_or(width as u32) as f64;
        let top_edge = neg_y.get(s.skip_index).copied().unwrap_or(0) as f64;
        let bottom_edge = pos_y.get(s.skip_index).copied().unwrap_or(height as u32) as f64;

        cr.set_line_width(2.0);
        cr.set_source_rgba(0.0, 0.5, 1.0, 0.9);

        match s.mode {
            Mode::Bounds => {
                cr.rectangle(left_edge, top_edge, right_edge - left_edge, bottom_edge - top_edge);
            },
            Mode::Crosshair => {
                cr.move_to(left_edge, cy); cr.line_to(right_edge, cy);
                cr.move_to(cx, top_edge); cr.line_to(cx, bottom_edge);
            },
            Mode::Horizontal => {
                cr.move_to(left_edge, cy); cr.line_to(right_edge, cy);
            },
            Mode::Vertical => {
                cr.move_to(cx, top_edge); cr.line_to(cx, bottom_edge);
            }
        }
        cr.stroke().unwrap();

        let hud_w = 160.0;
        let hud_h = 40.0;
        cr.set_source_rgba(0.1, 0.1, 0.1, 0.85);
        cr.rectangle(cx + 15.0, cy + 15.0, hud_w, hud_h);
        cr.fill().unwrap();

        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.set_font_size(14.0);
        cr.move_to(cx + 25.0, cy + 32.0);

        let text = match s.mode {
            Mode::Bounds | Mode::Crosshair => format!("X: {:.0}px | Y: {:.0}px", right_edge - left_edge, bottom_edge - top_edge),
            Mode::Horizontal => format!("Width: {:.0}px", right_edge - left_edge),
            Mode::Vertical => format!("Height: {:.0}px", bottom_edge - top_edge),
        };
        cr.show_text(&text).unwrap();
    });

    window.present();
}