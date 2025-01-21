// src/menu/edit/layers/handlers.rs
use std::{rc::Rc, cell::RefCell};
use fltk::frame::Frame;
use crate::state::ImageState;
use super::color_tool::start_interactive_color;

pub fn handle_create_layer(
    frame: &Rc<RefCell<Frame>>,
    state: &Rc<RefCell<ImageState>>,
    color: (u8, u8, u8)
) {
    println!("Starting handle_create_layer with color RGB({}, {}, {})", 
             color.0, color.1, color.2);
    
    start_interactive_color(frame, state, color);
}