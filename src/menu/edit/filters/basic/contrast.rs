use image::{ImageBuffer, Rgba};
use crate::state::FilterError;
use super::super::ImageFilter;

pub struct ContrastFilter {
    intensity: f32,
}

impl ContrastFilter {
    pub fn new(intensity: f32) -> Self {
        Self {
            intensity: intensity.clamp(0.0, 2.0),
        }
    }
}

impl ImageFilter for ContrastFilter {
    fn apply(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), FilterError> {
        let factor = (259.0 * (self.intensity * 255.0 + 255.0)) / (255.0 * (259.0 - self.intensity * 255.0));
        
        for pixel in image.pixels_mut() {
            for i in 0..3 {  // apply to RGB channels only
                let value = pixel[i] as f32;
                let new_value = factor * (value - 128.0) + 128.0;
                pixel[i] = new_value.clamp(0.0, 255.0) as u8;
            }
            // keep alpha channel unchanged
        }
        Ok(())
    }
}