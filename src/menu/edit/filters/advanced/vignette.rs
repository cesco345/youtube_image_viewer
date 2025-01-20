// src/menu/edit/filters/advanced/vignette.rs
use image::{ImageBuffer, Rgba};
use crate::state::FilterError;
use crate::menu::edit::filters::ImageFilter;
use crate::menu::edit::crop::crop_tool::CropSelection;

pub struct VignetteFilter {
    intensity: f32,
    feather: f32,     // this controls how soft the vignette edge is
    roundness: f32,   // this controls the shape (from circular to more rectangular)
    center_x: f32,    // this allows off-center vignette (0.5 is center)
    center_y: f32,    // this allows off-center vignette (0.5 is center)
    selection: Option<CropSelection>,
    feather_radius: u32,
}

impl VignetteFilter {
    pub fn new(intensity: f32) -> Self {
        Self {
            intensity: intensity.clamp(0.0, 1.0),
            feather: 0.5,        // default softness value
            roundness: 0.5,      // default shape (between circular and rectangular)
            center_x: 0.5,       // center horizontally
            center_y: 0.5,       // center vertically
            selection: None,
            feather_radius: 0,
        }
    }

    pub fn with_selection(mut self, selection: CropSelection) -> Self {
        self.selection = Some(selection);
        self
    }

    pub fn with_feather_radius(mut self, radius: u32) -> Self {
        self.feather_radius = radius;
        self
    }

    fn calculate_selection_factor(&self, x: i32, y: i32) -> f32 {
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

    fn calculate_vignette_factor(&self, x: f32, y: f32, width: f32, height: f32) -> f32 {
        // Regular vignette calculation
        let center_x = width * self.center_x;
        let center_y = height * self.center_y;

        let dx = (x - center_x) / (width * 0.5);
        let dy = (y - center_y) / (height * 0.5);

        let d_squared = {
            let dx_weighted = dx.abs().powf(2.0 * self.roundness);
            let dy_weighted = dy.abs().powf(2.0 * self.roundness);
            (dx_weighted + dy_weighted).powf(1.0 / self.roundness)
        };

        let falloff = 1.0 / (1.0 + (d_squared / self.feather).exp());
        let base_factor = falloff.powf(1.0 + self.intensity * 2.0);
        
        let contrast_boost = if self.intensity > 0.7 {
            let boost_amount = ((self.intensity - 0.7) / 0.3) * 0.2;
            base_factor.powf(1.0 + boost_amount)
        } else {
            base_factor
        };

        contrast_boost.clamp(0.0, 1.0)
    }

    pub fn with_feather(mut self, feather: f32) -> Self {
        self.feather = feather.clamp(0.1, 2.0);
        self
    }

    pub fn with_roundness(mut self, roundness: f32) -> Self {
        self.roundness = roundness.clamp(0.1, 2.0);
        self
    }

    pub fn with_center(mut self, x: f32, y: f32) -> Self {
        self.center_x = x.clamp(0.0, 1.0);
        self.center_y = y.clamp(0.0, 1.0);
        self
    }
}

impl ImageFilter for VignetteFilter {
    fn apply(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), FilterError> {
        let width = image.width() as f32;
        let height = image.height() as f32;
        let mut output = image.clone();

        for y in 0..image.height() {
            for x in 0..image.width() {
                let selection_factor = self.calculate_selection_factor(x as i32, y as i32);
                if selection_factor == 0.0 || !self.is_inside_selection(x as i32, y as i32) {
                    continue;
                }

                let vignette_factor = self.calculate_vignette_factor(x as f32, y as f32, width, height);
                let factor = vignette_factor * selection_factor;
                
                let original = image.get_pixel(x, y);
                let mut new_pixel = *original;
                
                // Apply the effect with improved gamma correction
                for c in 0..3 {
                    let linear = (original[c] as f32 / 255.0).powf(2.2);
                    let vignetted = linear * (factor * 0.8 + 0.2);
                    
                    let contrast_enhanced = if self.intensity > 0.5 {
                        let boost = (self.intensity - 0.5) * 0.4;
                        vignetted.powf(1.0 + boost)
                    } else {
                        vignetted
                    };
                    
                    // Blend between original and vignetted based on selection factor
                    let final_value = contrast_enhanced.powf(1.0/2.2) * 255.0;
                    new_pixel[c] = (final_value * selection_factor + original[c] as f32 * (1.0 - selection_factor))
                        .clamp(0.0, 255.0) as u8;
                }
                
                output.put_pixel(x, y, new_pixel);
            }
        }

        *image = output;
        Ok(())
    }
}