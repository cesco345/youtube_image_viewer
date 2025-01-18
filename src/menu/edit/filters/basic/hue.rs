use image::{ImageBuffer, Rgba};
use crate::state::FilterError;
use super::super::ImageFilter;

#[derive(Clone)]
pub struct HueFilter {
    angle: f32,
}

impl HueFilter {
    pub fn new(angle: f32) -> Self {
        Self {
            angle: angle % 360.0,
        }
    }

    fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
        let max = r.max(g.max(b));
        let min = r.min(g.min(b));
        let delta = max - min;

        let mut h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };

        if h < 0.0 {
            h += 360.0;
        }

        let s = if max == 0.0 { 0.0 } else { delta / max };
        let v = max;

        (h, s, v)
    }

    fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = match (h as i32) {
            h if h < 60 => (c, x, 0.0),
            h if h < 120 => (x, c, 0.0),
            h if h < 180 => (0.0, c, x),
            h if h < 240 => (0.0, x, c),
            h if h < 300 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        (r + m, g + m, b + m)
    }
}

impl ImageFilter for HueFilter {
    fn apply(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), FilterError> {
        for pixel in image.pixels_mut() {
            let (r, g, b) = (
                pixel[0] as f32 / 255.0,
                pixel[1] as f32 / 255.0,
                pixel[2] as f32 / 255.0
            );

            let (mut h, s, v) = Self::rgb_to_hsv(r, g, b);
            
            // rotate hue
            h = (h + self.angle) % 360.0;
            
            let (r, g, b) = Self::hsv_to_rgb(h, s, v);
            
            pixel[0] = (r * 255.0).clamp(0.0, 255.0) as u8;
            pixel[1] = (g * 255.0).clamp(0.0, 255.0) as u8;
            pixel[2] = (b * 255.0).clamp(0.0, 255.0) as u8;
            
            // keep alpha channel unchanged, pixel[3] remains unchanged
        }
        Ok(())
    }
}