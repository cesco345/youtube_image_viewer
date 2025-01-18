use image::{ImageBuffer, Rgba};
use crate::state::FilterError;
use super::super::ImageFilter;

pub struct SaturationFilter {
    intensity: f32,
}

impl SaturationFilter {
    pub fn new(intensity: f32) -> Self {
        Self {
            intensity: intensity.clamp(0.0, 2.0),
        }
    }
}

impl ImageFilter for SaturationFilter {
    fn apply(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), FilterError> {
        for pixel in image.pixels_mut() {
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;

            // convert to HSL-like space
            let max = r.max(g.max(b));
            let min = r.min(g.min(b));
            let delta = max - min;
            let l = (max + min) / 2.0;

            if delta > 0.0 {
                for i in 0..3 {
                    let value = pixel[i] as f32;
                    let new_value = l + (value - l) * self.intensity;
                    pixel[i] = new_value.clamp(0.0, 255.0) as u8;
                }
            }
            // when delta == 0, the pixel is already grayscale, no change is needed
        }
        Ok(())
    }
}