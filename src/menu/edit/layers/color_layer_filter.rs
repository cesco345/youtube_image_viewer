// src/menu/edit/layers/color_layer_filter.rs
use image::{ImageBuffer, Rgba};
use crate::menu::edit::filters::ImageFilter;
use crate::state::FilterError;  // Import FilterError from state module
use crate::menu::edit::crop::crop_tool::CropSelection;

pub struct ColorLayerFilter {
    color: (u8, u8, u8),    // RGB color for the layer
    opacity: f32,
    selection: Option<CropSelection>,
    feather: i32,
}

impl ColorLayerFilter {
    pub fn new(color: (u8, u8, u8)) -> Self {
        Self {
            color,
            opacity: 0.5,
            selection: None,
            feather: 5,
        }
    }

    pub fn with_selection(mut self, selection: CropSelection) -> Self {
        self.selection = Some(selection);
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn with_feather(mut self, feather: i32) -> Self {
        self.feather = feather;
        self
    }

    fn calculate_feather_opacity(&self, x: i32, y: i32, bounds: (i32, i32, i32, i32)) -> f32 {
        let (sx, sy, sw, sh) = bounds;
        let feather = self.feather as f32;
        
        let dx = if x < sx + self.feather {
            (x - sx) as f32 / feather
        } else if x >= sx + sw - self.feather {
            (sx + sw - x) as f32 / feather
        } else {
            1.0
        };

        let dy = if y < sy + self.feather {
            (y - sy) as f32 / feather
        } else if y >= sy + sh - self.feather {
            (sy + sh - y) as f32 / feather
        } else {
            1.0
        };

        dx.min(dy).max(0.0)
    }
}

impl ImageFilter for ColorLayerFilter {
    fn apply(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), FilterError> {
        if let Some(selection) = &self.selection {
            let (sx, sy, sw, sh) = selection.get_image_dimensions();
            
            for y in sy..sy+sh {
                for x in sx..sx+sw {
                    if x >= 0 && y >= 0 && x < image.width() as i32 && y < image.height() as i32 {
                        let feather_opacity = self.calculate_feather_opacity(x, y, (sx, sy, sw, sh));
                        let final_opacity = self.opacity * feather_opacity;

                        let pixel = image.get_pixel_mut(x as u32, y as u32);
                        
                        // Blend the color with existing pixel
                        pixel[0] = ((1.0 - final_opacity) * pixel[0] as f32 + 
                                  final_opacity * self.color.0 as f32) as u8;
                        pixel[1] = ((1.0 - final_opacity) * pixel[1] as f32 + 
                                  final_opacity * self.color.1 as f32) as u8;
                        pixel[2] = ((1.0 - final_opacity) * pixel[2] as f32 + 
                                  final_opacity * self.color.2 as f32) as u8;
                    }
                }
            }
            Ok(())
        } else {
            Err(FilterError { message: "No selection area provided".to_string() })
        }
    }
}