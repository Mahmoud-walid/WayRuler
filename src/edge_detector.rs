// src/edge_detector.rs

use image::{Pixel, RgbaImage};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

pub struct EdgeEngine {
    screenshot: RgbaImage,
    threshold: u8,
}

impl EdgeEngine {
    pub fn new(screenshot: RgbaImage) -> Self {
        Self {
            screenshot,
            threshold: 25, // Contrast threshold for edge detection
        }
    }

    /// Finds all visual edges from a starting coordinate along a given axis
    pub fn find_edges(&self, start_x: u32, start_y: u32, axis: Axis) -> (Vec<u32>, Vec<u32>) {
        let (width, height) = self.screenshot.dimensions();
        let mut edges_negative = Vec::new();
        let mut edges_positive = Vec::new();

        let max_val = match axis { Axis::Horizontal => width, Axis::Vertical => height };
        let mut last_luma = self.get_luma(start_x, start_y);

        // Scan negative direction
        let mut current = match axis { Axis::Horizontal => start_x, Axis::Vertical => start_y };
        while current > 0 {
            current -= 1;
            let (x, y) = match axis {
                Axis::Horizontal => (current, start_y),
                Axis::Vertical => (start_x, current),
            };
            let luma = self.get_luma(x, y);
            if current_diff(luma, last_luma) > self.threshold {
                edges_negative.push(current);
                last_luma = luma;
            }
        }

        // Scan positive direction
        last_luma = self.get_luma(start_x, start_y);
        current = match axis { Axis::Horizontal => start_x, Axis::Vertical => start_y };
        while current < max_val - 1 {
            current += 1;
            let (x, y) = match axis {
                Axis::Horizontal => (current, start_y),
                Axis::Vertical => (start_x, current),
            };
            let luma = self.get_luma(x, y);
            if current_diff(luma, last_luma) > self.threshold {
                edges_positive.push(current);
                last_luma = luma;
            }
        }

        (edges_negative, edges_positive)
    }

    fn get_luma(&self, x: u32, y: u32) -> u8 {
        let pixel = self.screenshot.get_pixel(x, y).channels();
        ((pixel[0] as u32 * 299 + pixel[1] as u32 * 587 + pixel[2] as u32 * 114) / 1000) as u8
    }
}

fn current_diff(a: u8, b: u8) -> u8 {
    if a > b { a - b } else { b - a }
}