// src/menu/edit/filters/advanced/motion_blur.rs
use image::{ImageBuffer, Rgba};
use crate::state::FilterError;
use crate::menu::edit::filters::ImageFilter;
use crate::menu::edit::crop::crop_tool::CropSelection;

pub struct MotionBlurFilter {
    angle: f32,
    kernel_size: usize,
    selection: Option<CropSelection>,
    feather_radius: u32,
    intensity: Option<f32>,
}

impl MotionBlurFilter {
    pub fn new(angle: f32) -> Self {
        Self {
            angle,
            kernel_size: 9,
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

    fn create_motion_kernel(&self) -> Vec<f32> {
        let size = self.kernel_size;
        let mut kernel = vec![0.0; size * size];
        let center = size / 2;
        let radians = self.angle.to_radians();
        
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        // Create a line in the kernel based on the angle
        for i in -(center as i32)..(center as i32 + 1) {
            let x = (center as f32 + i as f32 * cos_theta).round() as i32;
            let y = (center as f32 + i as f32 * sin_theta).round() as i32;

            if x >= 0 && x < size as i32 && y >= 0 && y < size as i32 {
                kernel[y as usize * size + x as usize] = 1.0;
            }
        }

        // Normalize the kernel
        let sum: f32 = kernel.iter().sum();
        if sum > 0.0 {
            for value in kernel.iter_mut() {
                *value /= sum;
            }
        }

        kernel
    }
}

impl ImageFilter for MotionBlurFilter {
    fn apply(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), FilterError> {
        let kernel = self.create_motion_kernel();
        let width = image.width() as usize;
        let height = image.height() as usize;
        let mut output = image.clone();
        let half_kernel = self.kernel_size / 2;

        for y in half_kernel..height - half_kernel {
            for x in half_kernel..width - half_kernel {
                let factor = self.calculate_feather_factor(x as i32, y as i32);
                if factor == 0.0 || !self.is_inside_selection(x as i32, y as i32) {
                    continue;
                }

                let mut r_acc = 0.0;
                let mut g_acc = 0.0;
                let mut b_acc = 0.0;
                let mut a_acc = 0.0;

                for ky in 0..self.kernel_size {
                    for kx in 0..self.kernel_size {
                        let k = kernel[ky * self.kernel_size + kx];
                        let pixel = image.get_pixel(
                            (x + kx - half_kernel) as u32,
                            (y + ky - half_kernel) as u32,
                        );

                        r_acc += pixel[0] as f32 * k;
                        g_acc += pixel[1] as f32 * k;
                        b_acc += pixel[2] as f32 * k;
                        a_acc += pixel[3] as f32 * k;
                    }
                }

                // Get original pixel for blending
                let original = image.get_pixel(x as u32, y as u32);
                
                // Blend between original and motion blurred based on feather factor
                let blend = |orig: u8, new: f32| -> u8 {
                    (new * factor + orig as f32 * (1.0 - factor)).clamp(0.0, 255.0) as u8
                };

                output.put_pixel(
                    x as u32,
                    y as u32,
                    Rgba([
                        blend(original[0], r_acc),
                        blend(original[1], g_acc),
                        blend(original[2], b_acc),
                        blend(original[3], a_acc),
                    ])
                );
            }
        }

        // Handle border pixels by copying from original image
        for y in 0..height {
            for x in 0..width {
                if y < half_kernel || y >= height - half_kernel || 
                   x < half_kernel || x >= width - half_kernel {
                    output.put_pixel(x as u32, y as u32, *image.get_pixel(x as u32, y as u32));
                }
            }
        }

        *image = output;
        Ok(())
    }
}
                