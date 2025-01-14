use crate::state::ImageState;
use fltk::{
    dialog::{alert, choice2},
    frame::Frame,
    prelude::*,
    image::RgbImage,
    enums::ColorDepth,
};
use std::{cell::RefCell, rc::Rc};
use crate::menu::file::save::handle_save_as;
use crate::utils::display_image;  // Add this import

pub fn handle_crop(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    let mut state_ref = state.borrow_mut();

    if state_ref.image.is_none() {
        alert(300, 300, "Please open an image first");
        return;
    }

    if let Some(current_image) = &state_ref.image {
        // Use data dimensions instead of display dimensions
        let img_w = current_image.data_w();
        let img_h = current_image.data_h();
        println!(
            "Original dimensions: {}x{}",
            img_w,
            img_h
        );

        let crop_w = img_w * 3 / 4;
        let crop_h = img_h * 3 / 4;
        let x = (img_w - crop_w) / 2;
        let y = (img_h - crop_h) / 2;

        // Create a new RGB image with the cropped dimensions
        let mut cropped_data = vec![0u8; (crop_w * crop_h * 3) as usize];
        let src_data = current_image.to_rgb_data();

        // Copy the pixel data directly from the source image
        for dy in 0..crop_h {
            for dx in 0..crop_w {
                let src_pos = (((y + dy) * img_w + (x + dx)) * 3) as usize;
                let dst_pos = ((dy * crop_w + dx) * 3) as usize;
                
                if src_pos + 2 < src_data.len() && dst_pos + 2 < cropped_data.len() {
                    cropped_data[dst_pos] = src_data[src_pos];
                    cropped_data[dst_pos + 1] = src_data[src_pos + 1];
                    cropped_data[dst_pos + 2] = src_data[src_pos + 2];
                }
            }
        }

        if let Ok(mut fltk_image) = RgbImage::new(
            &cropped_data,
            crop_w,
            crop_h,
            ColorDepth::Rgb8,
        ) {
            state_ref.image = Some(fltk_image.clone());
            state_ref.path = None;
            
            // Update the display using the utility function
            drop(state_ref); // Drop the mutable borrow before calling display_image
            let mut frame_mut = frame.borrow_mut();
            display_image(&mut frame_mut, &mut fltk_image, 1.0);
            
            if let Some(0) = choice2(
                300,
                300,
                "Image cropped successfully!\nWould you like to save the cropped image?",
                "Save",
                "Cancel",
                "",
            ) {
                handle_save_as(frame, state);
            }
        } else {
            alert(300, 300, "Failed to create cropped image");
        }
    }
}







