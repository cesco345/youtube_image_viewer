use image::{ImageBuffer, Rgba};
use crate::state::FilterError;
use super::super::ImageFilter;

pub struct ThresholdFilter {
    threshold: f32,
}

impl ThresholdFilter {
    pub fn new(threshold: f32) -> Self {
        Self {
            threshold: threshold.clamp(0.0, 1.0),
        }
    }
}

impl ImageFilter for ThresholdFilter {
    fn apply(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), FilterError> {
        let threshold_value = (self.threshold * 255.0) as u8;

        for pixel in image.pixels_mut() {
            // need to convert to grayscale first
            let gray = (0.299 * pixel[0] as f32 + 
                       0.587 * pixel[1] as f32 + 
                       0.114 * pixel[2] as f32) as u8;
            
            let new_value = if gray > threshold_value { 255 } else { 0 };
            
            for i in 0..3 {  // apply only to color RGB channels
                pixel[i] = new_value;
            }
            // keep alpha channel unchanged
        }
        Ok(())
    }
}