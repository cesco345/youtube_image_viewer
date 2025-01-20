// src/menu/edit/filters/advanced/convolution.rs
use image::{ImageBuffer, Rgba};
use crate::state::FilterError;
use crate::menu::edit::filters::ImageFilter;
use crate::menu::edit::crop::crop_tool::CropSelection;

#[derive(Clone)]
pub enum FilterType {
    BoxBlur,
    GaussianBlur,
    Sharpen,
}
#[derive(Clone)]
 pub enum ConvolutionType {
     GaussianBlur { radius: f32, sigma: f32 },
     BoxBlur { radius: f32 },
     Sharpen { intensity: f32 },
 }

pub struct ConvolutionFilter {
    radius: f32,
    sigma: Option<f32>,
    intensity: Option<f32>,
    filter_type: FilterType,
    selection: Option<CropSelection>,
    feather_radius: u32,
}

impl ConvolutionFilter {
    pub fn new_box_blur(radius: f32) -> Self {
        Self {
            radius: radius.max(1.0),
            sigma: None,
            intensity: Some(1.0),
            filter_type: FilterType::BoxBlur,
            selection: None,
            feather_radius: 0,
        }
    }
    pub fn with_intensity(mut self, intensity: f32) -> Self {
               self.intensity = Some(intensity);
                self
            }

    pub fn new_gaussian_blur(radius: f32, sigma: f32) -> Self {
        Self {
            radius,
            sigma: Some(sigma),
            intensity: Some(1.0),
            filter_type: FilterType::GaussianBlur,
            selection: None,
            feather_radius: 0,
        }
    }

    pub fn new_sharpen(intensity: f32) -> Self {
        Self {
            radius: 1.0,
            sigma: None,
            intensity: Some(intensity.clamp(0.0, 5.0)),
            filter_type: FilterType::Sharpen,
            selection: None,
            feather_radius: 0,
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
/// in order to get exactly the formula, you would need to allow different sigmas for x and y, allow arbitrary 
/// means instead of centering at radius, Include an arbitrary amplitude factor A, remove the normalization steps
/// if you want the raw Gaussian values: 
///     G₀(x,y) = A * exp(-(x-μₓ)²/(2σₓ²) - (y-μᵧ)²/(2σᵧ²))

    fn create_gaussian_kernel(&self) -> Vec<f32> {
        let size = (self.radius * 2.0 + 1.0) as usize;
        let mut kernel = vec![0.0; size * size];
        let sigma = self.sigma.unwrap_or(self.radius / 2.0);
        let sigma_sq = sigma * sigma;
        let mut sum = 0.0;

        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - self.radius;
                let dy = y as f32 - self.radius;
                let exp_term = -(dx * dx + dy * dy) / (2.0 * sigma_sq);
                let value = exp_term.exp() / (2.0 * std::f32::consts::PI * sigma_sq); // Gaussian function
                kernel[y * size + x] = value;
                sum += value;
            }
        }

        for value in kernel.iter_mut() {
            *value /= sum;   // normalize kernel - ensures the kernel weights sum to 1
        }
        kernel
    }

    fn create_box_kernel(&self) -> Vec<f32> {
        let size = (self.radius * 2.0 + 1.0) as usize;
        let total_size = size * size;
        let value = 1.0 / total_size as f32;
        vec![value; total_size]
    }

    fn create_sharpen_kernel(&self) -> Vec<f32> {
        let intensity = self.intensity.unwrap_or(1.0);
        let center = 1.0 + 4.0 * intensity;
        let edge = -intensity;
        
        vec![
            0.0,  edge, 0.0,
            edge, center, edge,
            0.0,  edge, 0.0
        ]
    }

    fn apply_kernel(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, kernel: &[f32], kernel_size: usize) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let width = image.width() as usize;
        let height = image.height() as usize;
        let mut output = image.clone();
        let half_kernel = (kernel_size / 2) as i32;

        for y in 0..height {
            for x in 0..width {
                let factor = self.calculate_feather_factor(x as i32, y as i32);
                if factor == 0.0 {
                    continue;
                }

                let mut r = 0.0;
                let mut g = 0.0;
                let mut b = 0.0;
                let mut a = 0.0;

                for ky in 0..kernel_size {
                    for kx in 0..kernel_size {
                        let img_x = (x as i32 + kx as i32 - half_kernel)
                            .clamp(0, width as i32 - 1) as u32;
                        let img_y = (y as i32 + ky as i32 - half_kernel)
                            .clamp(0, height as i32 - 1) as u32;

                        let k = kernel[ky * kernel_size + kx];
                        let pixel = image.get_pixel(img_x, img_y);

                        r += pixel[0] as f32 * k;
                        g += pixel[1] as f32 * k;
                        b += pixel[2] as f32 * k;
                        a += pixel[3] as f32 * k;
                    }
                }

                // Blend between original and filtered based on feather factor
                let pixel = image.get_pixel(x as u32, y as u32);
                let blend = |orig: u8, filtered: f32| -> u8 {
                    let filtered = filtered.clamp(0.0, 255.0) as u8;
                    ((filtered as f32 * factor + orig as f32 * (1.0 - factor)) as u8)
                };

                output.put_pixel(
                    x as u32,
                    y as u32,
                    Rgba([
                        blend(pixel[0], r),
                        blend(pixel[1], g),
                        blend(pixel[2], b),
                        blend(pixel[3], a),
                    ])
                );
            }
        }

        output
    }
}

impl ImageFilter for ConvolutionFilter {
    fn apply(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), FilterError> {
        let (kernel, size) = match self.filter_type {
            FilterType::GaussianBlur => {
                let kernel = self.create_gaussian_kernel();
                let size = (self.radius * 2.0 + 1.0) as usize;
                (kernel, size)
            },
            FilterType::BoxBlur => {
                let kernel = self.create_box_kernel();
                let size = (self.radius * 2.0 + 1.0) as usize;
                (kernel, size)
            },
            FilterType::Sharpen => {
                let kernel = self.create_sharpen_kernel();
                (kernel, 3)
            },
        };

        *image = self.apply_kernel(image, &kernel, size);
        Ok(())
    }
}