// src/menu/edit/layers/handlers.rs
use std::{rc::Rc, cell::RefCell};
use fltk::frame::Frame;
use fltk::prelude::*;  // Add this for WidgetExt trait
use crate::state::ImageState;
use super::color_tool::start_interactive_color;

pub fn handle_toggle_preview(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        state_ref.layer_state.toggle_preview();
        
        // Update the displayed image based on preview state
        if state_ref.layer_state.is_preview_active() {
            if let Some(composite) = state_ref.layer_state.get_composite_image() {
                state_ref.image = Some(composite.clone());
                frame.borrow_mut().set_image(Some(composite));
                frame.borrow_mut().redraw();
            }
        } else {
            if let Some(original) = state_ref.layer_state.get_original_image() {
                let original_image = original.clone();
                state_ref.image = Some(original_image.clone());
                frame.borrow_mut().set_image(Some(original_image));
                frame.borrow_mut().redraw();
            }
        }
    }
}

pub fn handle_create_layer(
    frame: &Rc<RefCell<Frame>>,
    state: &Rc<RefCell<ImageState>>,
    color: (u8, u8, u8)
) {
    println!("Starting handle_create_layer with color RGB({}, {}, {})", 
             color.0, color.1, color.2);
    
    start_interactive_color(frame, state, color);
}