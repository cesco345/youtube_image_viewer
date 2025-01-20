// src/menu/edit/filters/advanced/posterize.rs
use image::{ImageBuffer, Rgba};
use crate::state::FilterError;
use crate::menu::edit::filters::ImageFilter;
use crate::menu::edit::crop::crop_tool::CropSelection;

pub struct PosterizeFilter {
    levels: u8,
    selection: Option<CropSelection>,
    feather_radius: u32,
    intensity: Option<f32>,
}

impl PosterizeFilter {
    pub fn new(levels: u8) -> Self {
        Self {
            levels: levels.clamp(2, 8),  // Ensure levels is between 2 and 8
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

    fn posterize_value(&self, value: u8) -> u8 {
        // Calculate the size of each level band
        let band_size = 256.0 / self.levels as f32;
        
        // Determine which band the value falls into
        let band = (value as f32 / band_size).floor();
        
        // Calculate the representative value for this band
        // Use the middle of the band for better visual results
        let new_value = ((band * band_size) + (band_size / 2.0)) as u8;
        
        new_value.clamp(0, 255)
    }
}

impl ImageFilter for PosterizeFilter {
    fn apply(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), FilterError> {
        // Create a lookup table for better performance
        let lut: Vec<u8> = (0..=255)
            .map(|v| self.posterize_value(v))
            .collect();

        let mut output = image.clone();

        for y in 0..image.height() {
            for x in 0..image.width() {
                let factor = self.calculate_feather_factor(x as i32, y as i32);
                if factor == 0.0 || !self.is_inside_selection(x as i32, y as i32) {
                    continue;
                }

                let original = image.get_pixel(x, y);
                let mut posterized = [0u8; 4];

                // Apply posterization
                for c in 0..3 {
                    posterized[c] = lut[original[c] as usize];
                }
                posterized[3] = original[3]; // Keep alpha unchanged

                // Blend between original and posterized based on feather factor
                let blend = |orig: u8, new: u8| -> u8 {
                    (new as f32 * factor + orig as f32 * (1.0 - factor)) as u8
                };

                output.put_pixel(x, y, Rgba([
                    blend(original[0], posterized[0]),
                    blend(original[1], posterized[1]),
                    blend(original[2], posterized[2]),
                    original[3],
                ]));
            }
        }

        *image = output;
        Ok(())
    }
}