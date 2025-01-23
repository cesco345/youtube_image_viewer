// menu/file/open.rs

use crate::state::ImageState;
use crate::utils::display_image_with_zoom;
use fltk::{
    dialog::{FileDialog, FileDialogType},
    frame::Frame,
    image::RgbImage,
    enums::ColorDepth,
};
use std::{cell::RefCell, path::PathBuf, rc::Rc};

pub fn handle_open(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    let frame_open = frame.clone();
    let state_open = state.clone();
    
    let mut dialog = FileDialog::new(FileDialogType::BrowseFile);
    dialog.set_filter("Image Files\t*.{jpg,jpeg,png,gif,bmp,tif,tiff}");
    dialog.show();
    
    if let Some(filename) = dialog.filename().to_str() {
        if let Ok(img) = image::open(&filename) {
            let rgb_img = img.to_rgb8();
            let (width, height) = rgb_img.dimensions();
            
            if let Ok(mut fltk_image) = RgbImage::new(&rgb_img, width as i32, height as i32, ColorDepth::Rgb8) {
                let mut state = state_open.borrow_mut();
                state.path = Some(PathBuf::from(filename));
                state.zoom = 1.0;
                display_image_with_zoom(&frame_open, &mut fltk_image, state.zoom.into());
                state.image = Some(fltk_image);
            } else {
                println!("Failed to convert image to FLTK format: {}", filename);
            }
        } else {
            println!("Failed to load image: {}", filename);
        }
    }
}