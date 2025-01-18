use image::{ImageBuffer, Rgba};
use crate::state::FilterError;
use super::super::ImageFilter;

pub struct BrightnessFilter {
    intensity: f32,
}

impl BrightnessFilter {
    pub fn new(intensity: f32) -> Self {
        Self {
            // need to allow negative values for darkening effect
            intensity: intensity.clamp(-1.0, 1.0),
        }
    }
}

impl ImageFilter for BrightnessFilter {
    fn apply(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), FilterError> {
        for pixel in image.pixels_mut() {
            for i in 0..3 {  // you only apply to RGB channels only
                let value = pixel[i] as f32;
                if self.intensity > 0.0 {
                    // when brightening an image
                    pixel[i] = (value + (255.0 - value) * self.intensity) as u8;
                } else {
                    // when darkening an image
                    pixel[i] = (value * (1.0 + self.intensity)) as u8;
                }
            }
            // for transparency - keep alpha channel unchanged
        }
        Ok(())
    }
}