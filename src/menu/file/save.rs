use crate::state::ImageState;
use fltk::{
    dialog::{FileDialog, FileDialogType, alert, message},
    frame::Frame,
    prelude::*,
};
use std::{cell::RefCell, path::PathBuf, rc::Rc};
use image::ImageFormat;

pub fn handle_save(_frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    let state_ref = state.borrow();
    match &state_ref.path {
        Some(path) => {
            if let Some(img) = &state_ref.image {
                if save_image(img, path) {
                    message(200, 200, "Image saved successfully!");
                } else {
                    alert(200, 200, "Failed to save image!");
                }
            } else {
                alert(200, 200, "No image to save!");
            }
        }
        None => {
            // If no path exists, redirect to Save As
            drop(state_ref); // Drop the borrow before calling handle_save_as
            handle_save_as(_frame, state); // Pass the Rc<RefCell> directly
        }
    }
}

pub fn handle_save_as(_frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    let mut dialog = FileDialog::new(FileDialogType::BrowseSaveFile);
    dialog.set_filter("Image Files\t*.{jpg,jpeg,png,tif,tiff}");
    dialog.show();

    if let Some(filename) = dialog.filename().to_str() {
        let path = PathBuf::from(filename);
        let mut state = state.borrow_mut();
        
        if let Some(img) = &state.image {
            if save_image(img, &path) {
                state.path = Some(path);
                message(200, 200, "Image saved successfully!");
            } else {
                alert(200, 200, "Failed to save image!");
            }
        }
    }
}

fn save_image(image: &fltk::image::RgbImage, path: &PathBuf) -> bool {
    // Use data dimensions instead of display dimensions
    let width = image.data_w() as u32;     // Changed from width()
    let height = image.data_h() as u32;    // Changed from height()
    let data = image.to_rgb_data();

    println!("Saving image with dimensions: {}x{}", width, height);

    if let Some(img_buffer) = image::RgbImage::from_raw(width, height, data) {
        let format = match path.extension().and_then(|ext| ext.to_str()) {
            Some("jpg") | Some("jpeg") => Some(ImageFormat::Jpeg),
            Some("png") => Some(ImageFormat::Png),
            Some("tif") | Some("tiff") => Some(ImageFormat::Tiff),
            _ => None,
        };

        if let Some(format) = format {
            // Add quality settings for JPEG
            if format == ImageFormat::Jpeg {
                if let Err(e) = img_buffer.save_with_format(path, format) {
                    println!("Failed to save image: {}", e);
                    return false;
                }
            } else {
                if let Err(e) = img_buffer.save_with_format(path, format) {
                    println!("Failed to save image: {}", e);
                    return false;
                }
            }
            println!("Successfully saved image with dimensions: {}x{}", width, height);
            return true;
        }
    }
    false
}