use fltk::{frame::Frame, prelude::*};
use std::{rc::Rc, cell::RefCell, path::PathBuf};
use crate::state::ImageState;
use super::{
    dialog,
    image_watermark::ImageWatermark,
    text_watermark::TextWatermark,
    WatermarkOptions,
    WatermarkPosition,
    Position,
    BlendMode,
};
use image::Rgba;

pub fn handle_add_watermark_from_path(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    watermark_path: PathBuf
) {
    println!("Starting handle_add_watermark");
    
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        println!("Successfully borrowed state");
        
        let current_image = if let Some(img) = &state_ref.image {
            println!("Found current image in state");
            img.clone()
        } else {
            println!("No image found in state");
            return;
        };

        let options = WatermarkOptions {
            position: WatermarkPosition::BottomRight(Position::new(20, 20)),
            opacity: 0.3,
            blend_mode: BlendMode::Normal,
            rotation: None,
            scale: Some(0.25),
            padding: Some(10),
            repeat: false,
        };

        state_ref.watermark_state.set_options(options);
        
        println!("Attempting to load watermark from: {:?}", watermark_path);
        
        match ImageWatermark::from_path(&watermark_path) {
            Ok(watermark) => {
                println!("Successfully loaded watermark image");
                state_ref.watermark_state.set_watermark(watermark);
                
                if let Ok(Some(new_image)) = state_ref.watermark_state.apply_watermark(&current_image) {
                    println!("Successfully applied watermark");
                    frame.borrow_mut().set_image(Some(new_image.clone()));
                    frame.borrow_mut().redraw();
                    state_ref.image = Some(new_image);
                } else {
                    println!("Failed to apply watermark");
                }
            },
            Err(e) => {
                println!("Failed to load watermark: {}", e);
            }
        }
    } else {
        println!("Failed to borrow state");
    }
}

pub fn handle_add_text_watermark(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    text: String
) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        println!("Starting text watermark");
        
        let current_image = if let Some(img) = &state_ref.image {
            println!("Found current image in state");
            img.clone()
        } else {
            println!("No image found in state");
            return;
        };

        let options = WatermarkOptions {
            position: WatermarkPosition::BottomRight(Position::new(20, 20)),
            opacity: 0.8,
            blend_mode: BlendMode::Normal,
            rotation: None,
            scale: Some(0.25),
            padding: Some(10),
            repeat: false,
        };

        state_ref.watermark_state.set_options(options);
        
        match TextWatermark::new(text, Rgba([0, 0, 0, 255]), 32.0) {
            Ok(watermark) => {
                println!("Created text watermark");
                state_ref.watermark_state.set_text_watermark(watermark);
                
                if let Ok(Some(new_image)) = state_ref.watermark_state.apply_watermark(&current_image) {
                    println!("Successfully applied text watermark");
                    frame.borrow_mut().set_image(Some(new_image.clone()));
                    frame.borrow_mut().redraw();
                    state_ref.image = Some(new_image);
                }
            },
            Err(e) => {
                println!("Failed to create text watermark: {}", e);
            }
        }
    }
}

pub fn handle_add_watermark(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    let default_path = PathBuf::from("./images/watermark1.jpg");
    handle_add_watermark_from_path(frame, state, default_path);
}

pub fn handle_edit_watermark(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    if let Ok(state_ref) = state.try_borrow() {
        if state_ref.watermark_state.has_watermark() {
            drop(state_ref); // Release borrow before showing dialog
            dialog::show_edit_watermark_dialog(frame, state);
        } else {
            println!("No watermark to edit");
        }
    }
}

pub fn handle_remove_watermark(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    println!("Starting watermark removal tool");
    super::removal_tool::start_watermark_removal(frame, state);
}

pub fn handle_toggle_preview(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        let preview_active = state_ref.watermark_state.is_preview_active();
        let current_zoom = state_ref.zoom;
        state_ref.watermark_state.toggle_preview();
        
        if let Some(ref image) = state_ref.image.clone() {
            if preview_active {
                // Reload original image when disabling preview
                if let Some(path) = &state_ref.path {
                    if let Ok(img) = image::open(path) {
                        let fltk_image = fltk::image::RgbImage::new(
                            &img.to_rgb8().into_raw(),
                            img.width() as i32,
                            img.height() as i32,
                            fltk::enums::ColorDepth::Rgb8
                        ).unwrap();
                        
                        // Use display_image to maintain proper scaling
                        crate::utils::image::display_image(frame, &fltk_image, current_zoom.into());
                        state_ref.image = Some(fltk_image);
                        return;
                    }
                }
                crate::utils::image::display_image(frame, image, current_zoom.into());
            } else {
                if let Ok(Some(preview_image)) = state_ref.watermark_state.apply_watermark(image) {
                    crate::utils::image::display_image(frame, &preview_image, current_zoom.into());
                }
            }
        }
    }
}