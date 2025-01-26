//sr
use crate::state::ImageState;
use crate::utils::image::display_image_with_zoom;
use fltk::{
    prelude::*,
    dialog::{FileDialog, FileDialogType},
    frame::Frame,
    image::RgbImage,
    enums::ColorDepth,
};
use std::{cell::RefCell, path::PathBuf, rc::Rc};

pub fn handle_open(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    let mut dialog = FileDialog::new(FileDialogType::BrowseFile);
    dialog.set_filter("Image Files\t*.{jpg,jpeg,png,gif,bmp,tif,tiff}");
    dialog.show();
    
    if let Some(filename) = dialog.filename().to_str() {
        if let Ok(img) = image::open(&filename) {
            let rgb_img = img.to_rgb8();
            let (width, height) = rgb_img.dimensions();
            
            if let Ok(mut fltk_image) = RgbImage::new(&rgb_img, width as i32, height as i32, ColorDepth::Rgb8) {
                if let Ok(mut state_ref) = state.try_borrow_mut() {
                    state_ref.path = Some(PathBuf::from(filename));
                    state_ref.zoom = 1.0;
                    state_ref.image = Some(fltk_image.clone());
                    display_image_with_zoom(frame, &mut fltk_image, 1.0, state);
                }
            }
        }
    }
}
