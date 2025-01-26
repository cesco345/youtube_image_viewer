// in scientific/tools/handlers/scale.rs

use fltk::{prelude::*, frame::Frame};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;
use crate::scientific::rendering::ScaleRenderer;
use crate::scientific::rendering::FrameRenderer;

pub fn handle_toggle_scale_legend(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        state_ref.scientific_state.toggle_legend();
        FrameRenderer::setup_scientific_frame(frame, state);
        frame.borrow_mut().redraw();
    }
}