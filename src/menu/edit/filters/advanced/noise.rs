// src/menu/edit/filters/advanced/noise.rs
use image::{ImageBuffer, Rgba};
use rand::Rng;
use crate::state::FilterError;
use crate::menu::edit::filters::ImageFilter;
use crate::menu::edit::crop::crop_tool::CropSelection;

pub struct NoiseFilter {
    amount: f32,
    selection: Option<CropSelection>,
    feather_radius: u32,
    intensity: Option<f32>,
}

impl NoiseFilter {
    pub fn new(amount: f32) -> Self {
        Self { 
            amount,
            selection: None,
            feather_radius: 0,
            intensity: Some(1.0),
        }
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
}

impl ImageFilter for NoiseFilter {
    fn apply(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), FilterError> {
        let mut rng = rand::thread_rng();
        let width = image.width();
        let height = image.height();
        let mut output = image.clone();

        for y in 0..height {
            for x in 0..width {
                let factor = self.calculate_feather_factor(x as i32, y as i32);
                if factor == 0.0 || !self.is_inside_selection(x as i32, y as i32) {
                    continue;
                }

                if rng.gen::<f32>() < self.amount * factor {
                    let noise = rng.gen_range(-50..=50);
                    let original = image.get_pixel(x, y);
                    let mut noisy = *original;
                    
                    for c in 0..3 {
                        noisy[c] = ((original[c] as i16 + noise).clamp(0, 255)) as u8;
                    }

                    // Blend between original and noisy based on feather factor
                    let blend = |orig: u8, new: u8| -> u8 {
                        (new as f32 * factor + orig as f32 * (1.0 - factor)) as u8
                    };

                    output.put_pixel(x, y, Rgba([
                        blend(original[0], noisy[0]),
                        blend(original[1], noisy[1]),
                        blend(original[2], noisy[2]),
                        255,
                    ]));
                }
            }
        }

        *image = output;
        Ok(())
    }
}