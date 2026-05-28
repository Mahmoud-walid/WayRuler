// src/capture.rs

use anyhow::{Context, Result};
use image::RgbaImage;
use std::process::Command;

pub async fn capture_plasma_screen() -> Result<RgbaImage> {
    let path = "/tmp/wayruler_capture.png";

    // KDE Plasma 6 blocks unauthorized D-Bus screenshot calls.
    // Instead, we silently execute 'spectacle' which has native authorization.
    let status = Command::new("spectacle")
        .args(["-b", "-n", "-f", "-o", path])
        .status()
        .context("Failed to run 'spectacle'. Is it installed?")?;

    if !status.success() {
        anyhow::bail!("Spectacle failed to capture screen.");
    }

    // Decode the saved screenshot image
    let img = image::io::Reader::open(path)?
        .with_guessed_format()?
        .decode()?;

    // Clean up the temp file
    let _ = std::fs::remove_file(path);

    Ok(img.to_rgba8())
}