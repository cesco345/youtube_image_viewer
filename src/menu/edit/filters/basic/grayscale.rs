use image::{ImageBuffer, Rgba};
use crate::state::FilterError;
use super::super::ImageFilter; 

pub struct GrayscaleFilter {
    intensity: f32,
}

impl GrayscaleFilter {
    pub fn new(intensity: f32) -> Self {
        Self {
            intensity: intensity.clamp(0.0, 1.0),
        }
    }
}

impl ImageFilter for GrayscaleFilter {
    fn apply(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), FilterError> {
        for pixel in image.pixels_mut() {
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;

            // standard grayscale conversion weights
            let gray = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
            
            // blending magic between the original and gray based on intensity
            pixel[0] = ((1.0 - self.intensity) * r + self.intensity * gray as f32) as u8;
            pixel[1] = ((1.0 - self.intensity) * g + self.intensity * gray as f32) as u8;
            pixel[2] = ((1.0 - self.intensity) * b + self.intensity * gray as f32) as u8;
            
            // keep alpha channel unchanged, pixel[3] remains unchanged
        }
        Ok(())
    }
}