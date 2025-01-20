// pixelate.rs
use image::{ImageBuffer, Rgba};
use crate::state::FilterError;
use crate::menu::edit::filters::ImageFilter;
use crate::menu::edit::crop::crop_tool::CropSelection;

pub struct PixelateFilter {
    block_size: u32,
    selection: Option<CropSelection>,
    feather_radius: u32,
    intensity: f32,
}

impl PixelateFilter {
    pub fn new(block_size: u32) -> Self {
        Self {
            block_size: block_size.max(2),
            selection: None,
            feather_radius: 0,
            intensity: 1.0,
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

    fn pixelate_region(
        &self,
        image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
        selection: &CropSelection
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut output = image.clone();
        let (sel_x, sel_y, sel_w, sel_h) = selection.get_image_dimensions();
        
        for by in (sel_y..sel_y + sel_h).step_by(self.block_size as usize) {
            for bx in (sel_x..sel_x + sel_w).step_by(self.block_size as usize) {
                let mut r_sum = 0u32;
                let mut g_sum = 0u32;
                let mut b_sum = 0u32;
                let mut a_sum = 0u32;
                let mut count = 0u32;

                // Calculate block boundaries
                let block_x_end = (bx + self.block_size as i32).min(sel_x + sel_w);
                let block_y_end = (by + self.block_size as i32).min(sel_y + sel_h);

                // Calculate average color for the block
                for y in by..block_y_end {
                    for x in bx..block_x_end {
                        if x >= 0 && y >= 0 && x < image.width() as i32 && y < image.height() as i32 {
                            let pixel = image.get_pixel(x as u32, y as u32);
                            r_sum += pixel[0] as u32;
                            g_sum += pixel[1] as u32;
                            b_sum += pixel[2] as u32;
                            a_sum += pixel[3] as u32;
                            count += 1;
                        }
                    }
                }

                if count > 0 {
                    let r_avg = (r_sum / count) as u8;
                    let g_avg = (g_sum / count) as u8;
                    let b_avg = (b_sum / count) as u8;
                    let a_avg = (a_sum / count) as u8;

                    // Fill the block with the average color, applying feathering
                    for y in by..block_y_end {
                        for x in bx..block_x_end {
                            if x >= 0 && y >= 0 && x < image.width() as i32 && y < image.height() as i32 {
                                let feather_factor = self.calculate_feather_factor(x, y, selection);
                                let blend_factor = feather_factor * self.intensity;
                                
                                let original = image.get_pixel(x as u32, y as u32);
                                let pixel = Rgba([
                                    Self::blend(original[0], r_avg, blend_factor),
                                    Self::blend(original[1], g_avg, blend_factor),
                                    Self::blend(original[2], b_avg, blend_factor),
                                    Self::blend(original[3], a_avg, blend_factor),
                                ]);
                                
                                output.put_pixel(x as u32, y as u32, pixel);
                            }
                        }
                    }
                }
            }
        }

        output
    }

    fn blend(original: u8, pixelated: u8, factor: f32) -> u8 {
        ((pixelated as f32 * factor + original as f32 * (1.0 - factor)) as u8)
    }
}

impl ImageFilter for PixelateFilter {
    fn apply(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), FilterError> {
        if let Some(ref selection) = self.selection {
            *image = self.pixelate_region(image, selection);
        } else {
            // Create a selection for the entire image
            let selection = CropSelection::new(
                image.width() as i32,
                image.height() as i32,
                image.width() as i32,
                image.height() as i32,
            );
            *image = self.pixelate_region(image, &selection);
        }
        Ok(())
    }
}