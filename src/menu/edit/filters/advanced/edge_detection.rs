// src/menu/edit/filters/advanced/edge_detection.rs
use image::{ImageBuffer, Rgba};
use crate::state::FilterError;
use super::super::ImageFilter;
use crate::menu::edit::crop::crop_tool::CropSelection;

#[derive(Clone, Copy)]
pub enum EdgeDetectionMethod {
    Sobel,
    Canny,
}

pub struct EdgeDetectionFilter {
    threshold: f32,
    method: EdgeDetectionMethod,
    low_threshold: f32,      // For Canny's hysteresis
    high_threshold: f32,     // For Canny's hysteresis
    selection: Option<CropSelection>,
    feather_radius: u32,
    intensity: Option<f32>,
}

impl EdgeDetectionFilter {
    pub fn new(threshold: f32, method: EdgeDetectionMethod) -> Self {
        let threshold = threshold.clamp(0.0, 1.0);
        Self {
            threshold,
            method,
            low_threshold: threshold * 0.5,    // Default low threshold for Canny
            high_threshold: threshold,          // Default high threshold for Canny
            selection: None,
            feather_radius: 0,
            intensity: Some(1.0),
        }
    }

    pub fn with_canny_thresholds(mut self, low: f32, high: f32) -> Self {
        self.low_threshold = low.clamp(0.0, 1.0);
        self.high_threshold = high.clamp(0.0, 1.0);
        self
    }

    pub fn with_selection(mut self, selection: CropSelection) -> Self {
        self.selection = Some(selection);
        self
    }

    pub fn with_feather(mut self, radius: u32) -> Self {
        self.feather_radius = radius;
        self
    }

    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.intensity = Some(intensity);
        self
    }

    fn calculate_feather_factor(&self, x: i32, y: i32) -> f32 {
        if self.feather_radius == 0 || self.selection.is_none() {
            return 1.0;
        }

        let selection = self.selection.as_ref().unwrap();
        let (sel_x, sel_y, sel_w, sel_h) = selection.get_image_dimensions();
        let sel_end_x = sel_x + sel_w;
        let sel_end_y = sel_y + sel_h;

        let dx = if x < sel_x {
            sel_x - x
        } else if x >= sel_end_x {
            x - (sel_end_x - 1)
        } else {
            0
        };

        let dy = if y < sel_y {
            sel_y - y
        } else if y >= sel_end_y {
            y - (sel_end_y - 1)
        } else {
            0
        };

        let distance = ((dx * dx + dy * dy) as f32).sqrt();
        if distance >= self.feather_radius as f32 {
            0.0
        } else {
            (1.0 - distance / self.feather_radius as f32).powf(0.75)
        }
    }

    fn is_inside_selection(&self, x: i32, y: i32) -> bool {
        if let Some(selection) = &self.selection {
            let (sel_x, sel_y, sel_w, sel_h) = selection.get_image_dimensions();
            x >= sel_x && x < sel_x + sel_w && y >= sel_y && y < sel_y + sel_h
        } else {
            true  // If no selection, apply to entire image
        }
    }

    fn sobel_operators() -> ([f32; 9], [f32; 9]) {
        let gx = [
            -1.0, 0.0, 1.0,
            -2.0, 0.0, 2.0,
            -1.0, 0.0, 1.0
        ];
        
        let gy = [
            -1.0, -2.0, -1.0,
            0.0, 0.0, 0.0,
            1.0, 2.0, 1.0
        ];
        
        (gx, gy)
    }

    fn gaussian_kernel(size: usize, sigma: f32) -> Vec<f32> {
        let mut kernel = vec![0.0; size * size];
        let center = (size / 2) as f32;
        let sigma_sq = sigma * sigma;
        let mut sum = 0.0;

        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - center;
                let dy = y as f32 - center;
                let exponent = -(dx * dx + dy * dy) / (2.0 * sigma_sq);
                let value = (exponent.exp()) / (2.0 * std::f32::consts::PI * sigma_sq);
                kernel[y * size + x] = value;
                sum += value;
            }
        }

        for value in kernel.iter_mut() {
            *value /= sum;
        }
        kernel
    }

    fn apply_sobel(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let (gx, gy) = Self::sobel_operators();
        let width = image.width() as usize;
        let height = image.height() as usize;
        let mut output = image.clone();

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let factor = self.calculate_feather_factor(x as i32, y as i32);
                if factor == 0.0 || !self.is_inside_selection(x as i32, y as i32) {
                    continue;
                }

                let mut px = 0.0;
                let mut py = 0.0;

                for ky in 0..3 {
                    for kx in 0..3 {
                        let pixel = image.get_pixel((x + kx - 1) as u32, (y + ky - 1) as u32);
                        let intensity = pixel[0] as f32 * 0.299 
                            + pixel[1] as f32 * 0.587 
                            + pixel[2] as f32 * 0.114;

                        px += intensity * gx[ky * 3 + kx];
                        py += intensity * gy[ky * 3 + kx];
                    }
                }

                let magnitude = (px * px + py * py).sqrt();
                let edge = if magnitude > self.threshold * 255.0 { 255 } else { 0 };

                // Blend between original and edge detected based on feather factor
                let original = image.get_pixel(x as u32, y as u32);
                let blend = |orig: u8, new: u8| -> u8 {
                    (new as f32 * factor + orig as f32 * (1.0 - factor)) as u8
                };

                output.put_pixel(x as u32, y as u32, Rgba([
                    blend(original[0], edge),
                    blend(original[1], edge),
                    blend(original[2], edge),
                    255,
                ]));
            }
        }

        output
    }

    fn apply_canny(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        // For now, use Sobel as a fallback for Canny
        // You can implement the full Canny edge detection algorithm here
        self.apply_sobel(image)
    }
}

impl ImageFilter for EdgeDetectionFilter {
    fn apply(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), FilterError> {
        let output = match self.method {
            EdgeDetectionMethod::Sobel => self.apply_sobel(image),
            EdgeDetectionMethod::Canny => self.apply_canny(image),
        };

        *image = output;
        Ok(())
    }
}