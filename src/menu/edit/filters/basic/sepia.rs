use image::{ImageBuffer, Rgba};
use crate::state::FilterError;
use super::super::ImageFilter; 

pub struct SepiaFilter {
    intensity: f32,
}

impl SepiaFilter {
    pub fn new(intensity: f32) -> Self {
        Self {
            intensity: intensity.clamp(0.0, 1.0),
        }
    }
}

impl ImageFilter for SepiaFilter {
    fn apply(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), FilterError> {
        for pixel in image.pixels_mut() {
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;

            // this is the sepia tone conversion
            let sepia_r = (0.393 * r + 0.769 * g + 0.189 * b).min(255.0);
            let sepia_g = (0.349 * r + 0.686 * g + 0.168 * b).min(255.0);
            let sepia_b = (0.272 * r + 0.534 * g + 0.131 * b).min(255.0);
            
            // blend between the original and sepia based on intensity
            pixel[0] = ((1.0 - self.intensity) * r + self.intensity * sepia_r) as u8;
            pixel[1] = ((1.0 - self.intensity) * g + self.intensity * sepia_g) as u8;
            pixel[2] = ((1.0 - self.intensity) * b + self.intensity * sepia_b) as u8;
            // keep alpha channel unchanged pixel[3] remains unchanged
        }
        Ok(())
    }
}