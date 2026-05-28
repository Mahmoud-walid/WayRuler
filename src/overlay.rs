// src/overlay.rs

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, DrawingArea, EventControllerMotion, EventControllerScroll, gdk};
use gtk4_layer_shell::{Layer, LayerShell};
use std::cell::RefCell;
use std::rc::Rc;
use crate::edge_detector::{EdgeEngine, Axis};

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Crosshair,
    Horizontal,
    Vertical,
    Freeform,
}

struct AppState {
    cursor_x: f64,
    cursor_y: f64,
    skip_index: usize,
    mode: Mode,
}

pub fn build_ui(app: &Application, screenshot: image::RgbaImage) {
    let provider = gtk4::CssProvider::new();
    provider.load_from_data("window { background-color: transparent; }");
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
    window.set_keyboard_mode(
        gtk4_layer_shell::KeyboardMode::Exclusive
    );

    window.set_anchor(gtk4_layer_shell::Edge::Top, true);
    window.set_anchor(gtk4_layer_shell::Edge::Bottom, true);
    window.set_anchor(gtk4_layer_shell::Edge::Left, true);
    window.set_anchor(gtk4_layer_shell::Edge::Right, true);

    let state = Rc::new(RefCell::new(AppState {
        cursor_x: 0.0,
        cursor_y: 0.0,
        skip_index: 0,
        mode: Mode::Crosshair,
    }));

    let engine = Rc::new(EdgeEngine::new(screenshot));
    let drawing_area = DrawingArea::new();

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

    // Custom Cairo drawing for high-performance HUD & Lines
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

        // Safe indexing with Scroll-to-skip state
        let left_edge = neg_x.get(s.skip_index).copied().unwrap_or(0) as f64;
        let right_edge = pos_x.get(s.skip_index).copied().unwrap_or(width as u32) as f64;
        let top_edge = neg_y.get(s.skip_index).copied().unwrap_or(0) as f64;
        let bottom_edge = pos_y.get(s.skip_index).copied().unwrap_or(height as u32) as f64;

        cr.set_line_width(2.0);
        cr.set_source_rgba(0.0, 0.5, 1.0, 0.9);

        if s.mode == Mode::Crosshair {
            cr.move_to(left_edge, cy);
            cr.line_to(right_edge, cy);

            cr.move_to(cx, top_edge);
            cr.line_to(cx, bottom_edge);

            cr.stroke().unwrap();
        }

        let hud_w = 160.0;
        let hud_h = 40.0;
        cr.set_source_rgba(0.1, 0.1, 0.1, 0.85);
        // Position HUD to the bottom right of cursor, preventing off-screen clipping
        cr.rectangle(cx + 15.0, cy + 15.0, hud_w, hud_h);
        cr.fill().unwrap();

        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.set_font_size(14.0);
        cr.move_to(cx + 25.0, cy + 32.0);

        let text = format!("X: {:.0}px | Y: {:.0}px", right_edge - left_edge, bottom_edge - top_edge);
        cr.show_text(&text).unwrap();
    });

    window.set_child(Some(&drawing_area));
    window.present();
}