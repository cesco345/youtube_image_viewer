//! this is the filter state management module that handles image filtering operations
//! and maintains the state of active filters and previews.

use fltk::image::RgbImage;
use fltk::prelude::*;  // in order to be able to import for ImageExt trait
use image::{ImageBuffer, Rgba};
use crate::menu::edit::filters::ImageFilter;

/// represents an error that can occur during the filtering process in human readable format
#[derive(Clone, Debug)]
pub struct FilterError {   
    pub message: String,
}

// implement the Display trait for FilterError and return the error message to be used with std::error::Error down below
impl std::fmt::Display for FilterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FilterError {}

#[derive(Clone)]
// image conversin between fltk and image crate formats
pub struct FilterState {
    is_preview_active: bool,
    current_filter: Option<String>,
}
// implement the FilterState struct
impl FilterState {
    // create a new instance of FilterState
    pub fn new() -> Self {
        Self {
            is_preview_active: false,
            current_filter: None,
        }
    }
// apply a filter to an image and return the result
    pub fn apply_filter<F: ImageFilter>(&self, image: &RgbImage, filter: &F) -> Result<Option<RgbImage>, FilterError> {
        // convert the fltk image to image buffer
        let mut image_buffer = Self::fltk_to_image_buffer(image);
        filter.apply(&mut image_buffer)?;
        Ok(Self::image_buffer_to_fltk(&image_buffer))
    }

    fn fltk_to_image_buffer(fltk_image: &RgbImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
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

    fn image_buffer_to_fltk(buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Option<RgbImage> {
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

    pub fn toggle_preview(&mut self) {
        self.is_preview_active = !self.is_preview_active;
    }

    pub fn is_preview_active(&self) -> bool {
        self.is_preview_active
    }

    pub fn get_current_filter(&self) -> Option<String> {
        self.current_filter.clone()
    }

    pub fn set_current_filter(&mut self, filter: Option<String>) {
        self.current_filter = filter;
    }
}
// implement Default trait and allow creating a new instance of FilterState with default values
impl Default for FilterState {
    fn default() -> Self {
        Self::new()
    }
}