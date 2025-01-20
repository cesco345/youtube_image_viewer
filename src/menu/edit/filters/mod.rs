// External crate imports
use image::{ImageBuffer, Rgba};
use fltk::image::RgbImage;
use fltk::prelude::*;

// Internal imports
use crate::state::FilterError;

// Module declarations - public modules
pub mod basic;
pub mod advanced;
pub mod handlers;
pub mod dialog;

// Module declarations - private modules
mod pixelate_tool;
mod convolution_tool;
mod edge_detection_tool;
mod noise_tool;
mod vignette_tool;
mod posterize_tool;
mod motion_blur_tool;

// Public re-exports
pub use dialog::show_filter_dialog;
pub use handlers::*;
pub use pixelate_tool::start_interactive_pixelate;
pub use convolution_tool::start_interactive_convolution;
pub use edge_detection_tool::start_interactive_edge_detection;
pub use noise_tool::start_interactive_noise;
pub use vignette_tool::start_interactive_vignette;
pub use posterize_tool::start_interactive_posterize;
pub use motion_blur_tool::start_interactive_motion_blur;

pub use advanced::{ConvolutionType, EdgeDetectionMethod};  // Re-export from advanced module



pub trait ImageFilter {
    fn apply(&self, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), FilterError>;
}

// Helper function to convert between FLTK and image crate formats
pub(crate) fn fltk_to_image_buffer(fltk_image: &RgbImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let data = fltk_image.to_rgb_data();
    let width = fltk_image.data_w() as u32;
    let height = fltk_image.data_h() as u32;

    let mut rgba_data = Vec::with_capacity((width * height * 4) as usize);
    
    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 3) as usize;
            if idx + 2 < data.len() {
                rgba_data.push(data[idx]);     // R
                rgba_data.push(data[idx + 1]); // G
                rgba_data.push(data[idx + 2]); // B
                rgba_data.push(255);           // A
            }
        }
    }

    ImageBuffer::from_raw(width, height, rgba_data)
        .unwrap_or_else(|| ImageBuffer::new(width, height))
}

pub(crate) fn image_buffer_to_fltk(buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Option<RgbImage> {
    let width = buffer.width() as i32;
    let height = buffer.height() as i32;
    let mut rgb_data = Vec::with_capacity((width * height * 3) as usize);

    for pixel in buffer.pixels() {
        rgb_data.push(pixel[0]);
        rgb_data.push(pixel[1]);
        rgb_data.push(pixel[2]);
    }

    RgbImage::new(&rgb_data, width, height, fltk::enums::ColorDepth::Rgb8).ok()
}