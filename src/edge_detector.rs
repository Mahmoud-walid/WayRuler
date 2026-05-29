// src/edge_detector.rs

use image::RgbaImage;

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
            threshold: 8,
        }
    }

    /// Finds all visual edges from a starting coordinate along a given axis
    pub fn find_edges(&self, start_x: u32, start_y: u32, axis: Axis) -> (Vec<u32>, Vec<u32>) {
        let (width, height) = self.screenshot.dimensions();
        let mut edges_negative = Vec::new();
        let mut edges_positive = Vec::new();

        let max_val = match axis {
            Axis::Horizontal => width,
            Axis::Vertical => height,
        };

        let mut last_color = self.get_rgb(start_x, start_y);

        // Scan negative direction
        let mut current = match axis {
            Axis::Horizontal => start_x,
            Axis::Vertical => start_y,
        };
        while current > 0 {
            current -= 1;
            let (x, y) = match axis {
                Axis::Horizontal => (current, start_y),
                Axis::Vertical => (start_x, current),
            };
            let color = self.get_rgb(x, y);
            if color_diff(color, last_color) > self.threshold {
                edges_negative.push(current);
                last_color = color;
            }
        }

        // Scan positive direction
        last_color = self.get_rgb(start_x, start_y);
        current = match axis {
            Axis::Horizontal => start_x,
            Axis::Vertical => start_y,
        };
        while current < max_val - 1 {
            current += 1;
            let (x, y) = match axis {
                Axis::Horizontal => (current, start_y),
                Axis::Vertical => (start_x, current),
            };
            let color = self.get_rgb(x, y);
            if color_diff(color, last_color) > self.threshold {
                edges_positive.push(current);
                last_color = color;
            }
        }

        (edges_negative, edges_positive)
    }

    fn get_rgb(&self, x: u32, y: u32) -> [u8; 3] {
        let pixel = self.screenshot.get_pixel(x, y).0; // .0 gives [u8; 4] array
        [pixel[0], pixel[1], pixel[2]]
    }
}

fn color_diff(c1: [u8; 3], c2: [u8; 3]) -> u8 {
    let r_diff = c1[0].abs_diff(c2[0]);
    let g_diff = c1[1].abs_diff(c2[1]);
    let b_diff = c1[2].abs_diff(c2[2]);

    r_diff.max(g_diff).max(b_diff)
}
