// src/menu/edit/layers/color_filter.rs
use image::{ImageBuffer, Rgba};
use crate::state::FilterError;
use crate::menu::edit::filters::ImageFilter;
use crate::menu::edit::crop::crop_tool::CropSelection;

pub struct ColorFilter {
    color: (u8, u8, u8),
    selection: Option<CropSelection>,
    feather_radius: i32,
    intensity: f32,
}

impl ColorFilter {
    pub fn new(color: (u8, u8, u8)) -> Self {
        Self {
            color,
            selection: None,
            feather_radius: 5,
            intensity: 0.5,
        }
    }

    pub fn with_selection(mut self, selection: CropSelection) -> Self {
        self.selection = Some(selection);
        self
    }

    pub fn with_feather(mut self, radius: i32) -> Self {
        self.feather_radius = radius;
        self
    }

    pub fn with_intensity(mut self, intensity: f32) -> Self {
        self.intensity = intensity.clamp(0.0, 1.0);
        self
    }

    fn calculate_feather_factor(&self, x: i32, y: i32, selection: &CropSelection) -> f32 {
        if self.feather_radius == 0 {
            return 1.0;
        }

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

    fn blend(original: u8, color: u8, factor: f32) -> u8 {
        ((color as f32 * factor + original as f32 * (1.0 - factor)) as u8)
    }
}

impl ImageFilter for ColorFilter {
    fn apply(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), FilterError> {
        if let Some(ref selection) = self.selection {
            let (sel_x, sel_y, sel_w, sel_h) = selection.get_image_dimensions();
            
            for y in sel_y..sel_y + sel_h {
                for x in sel_x..sel_x + sel_w {
                    if x >= 0 && y >= 0 && x < image.width() as i32 && y < image.height() as i32 {
                        let feather_factor = self.calculate_feather_factor(x, y, selection);
                        let blend_factor = feather_factor * self.intensity;
                        
                        let pixel = image.get_pixel_mut(x as u32, y as u32);
                        
                        pixel[0] = Self::blend(pixel[0], self.color.0, blend_factor);
                        pixel[1] = Self::blend(pixel[1], self.color.1, blend_factor);
                        pixel[2] = Self::blend(pixel[2], self.color.2, blend_factor);
                    }
                }
            }
            Ok(())
        } else {
            Err(FilterError { 
                message: "No selection area provided".to_string() 
            })
        }
    }
}
impl ColorFilter {
    // this method exists to support opacity
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.intensity = opacity.clamp(0.0, 1.0);
        self
    }
}