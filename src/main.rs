// src/main.rs

mod capture;
mod edge_detector;
mod overlay;
mod utils;

use gtk4::prelude::*;
use gtk4::Application;
use tokio::runtime::Runtime;

fn main() -> anyhow::Result<()> {
    // We run the background shortcut listener in a Toki runtime
    let rt = Runtime::new()?;

    println!("Starting WayRuler daemon...");

    rt.block_on(async {
        println!("Launching overlay directly...");

        let screenshot = capture::capture_plasma_screen()
            .await
            .expect("Failed to capture screen");

        let app = Application::builder()
            .application_id("com.wayruler.Overlay")
            .build();

        app.connect_activate(move |app| {
            overlay::build_ui(app, screenshot.clone());
        });

        app.run();

        Ok(())
    })
}
