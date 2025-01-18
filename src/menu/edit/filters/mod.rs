pub mod basic;
mod handlers;
mod dialog;

pub use handlers::*;
pub use dialog::*;

use fltk::image::RgbImage;
use fltk::prelude::*; // Add this for ImageExt trait
use image::{ImageBuffer, Rgba};
use crate::state::FilterError;

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