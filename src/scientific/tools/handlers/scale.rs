use fltk::{prelude::*, frame::Frame};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;
use crate::scientific::rendering::{ScaleRenderer, FrameRenderer};
use crate::scientific::ui::show_calibration_welcome_dialog;
use crate::scientific::tools::interactive::start_interactive_scale;

pub fn handle_toggle_scale_legend(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        state_ref.scientific_state.toggle_legend();
        FrameRenderer::setup_scientific_frame(frame, state);
        frame.borrow_mut().redraw();
    }
}

pub fn handle_start_calibration(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    println!("Handler called"); // Debug print
    let result = show_calibration_welcome_dialog();
    println!("Dialog result: {}", result); // Debug print
    
    if result {
        println!("Starting calibration process"); // Debug print
        let frame_ref = frame.clone();
        let state_ref = state.clone();
        start_interactive_scale(&frame_ref, &state_ref);
    } else {
        println!("Calibration cancelled"); // Debug print
    }
}