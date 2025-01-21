// src/menu/edit/layers/dialog.rs
use fltk::{
    window::Window,
    button::Button,
    frame::Frame,
    group::Pack,
    menu::Choice,
    prelude::*,
};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;
use super::color_tool::start_interactive_color;

pub fn show_new_layer_dialog(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>
) -> bool {
    let mut dialog = Window::default()
        .with_size(400, 380)
        .with_label("Add New Layer");
    dialog.make_modal(true);

    let mut pack = Pack::new(10, 10, 380, 360, "");
    pack.set_spacing(10);

    // Title and Instructions
    Frame::default()
        .with_size(380, 25)
        .with_label("Create New Color Layer")
        .with_pos(0, 0);

    let instruction_text = "1. Select a color below\n2. Click Apply\n3. Click and drag on the image to select the area\n4. Repeat for additional areas with the same color";
    Frame::default()
        .with_size(380, 80)
        .with_label(instruction_text)
        .with_pos(0, 35);

    // Color selection
    Frame::default()
        .with_size(380, 25)
        .with_label("Select Color:")
        .with_pos(0, 125);

    let mut color_choice = Choice::new(20, 150, 180, 25, "");
    color_choice.add_choice("Red|Green|Blue|Yellow|Purple|Cyan");
    color_choice.set_value(0);  // Default to Red

    let dialog_rc = Rc::new(RefCell::new(dialog));
    let result = Rc::new(RefCell::new(false));

    let mut cancel = Button::new(190, 320, 85, 25, "Cancel");
    let mut apply = Button::new(285, 320, 85, 25, "Apply");

    pack.end();
    dialog_rc.borrow_mut().end();

    let dialog_rc_cancel = dialog_rc.clone();
    cancel.set_callback(move |_| {
        dialog_rc_cancel.borrow_mut().hide();
    });

    let dialog_rc_ok = dialog_rc.clone();
    let frame_rc = frame.clone();
    let state_rc = state.clone();
    let result_rc = result.clone();

    apply.set_callback(move |_| {
        let color = match color_choice.value() {
            0 => (255, 0, 0),      // Red
            1 => (0, 255, 0),      // Green
            2 => (0, 0, 255),      // Blue
            3 => (255, 255, 0),    // Yellow
            4 => (255, 0, 255),    // Purple
            5 => (0, 255, 255),    // Cyan
            _ => (255, 0, 0),      // Default to Red
        };

        // Hide dialog before starting interactive selection
        dialog_rc_ok.borrow_mut().hide();

        // Start interactive color area selection
        start_interactive_color(&frame_rc, &state_rc, color);
        
        *result_rc.borrow_mut() = true;
    });

    dialog_rc.borrow_mut().show();
    while dialog_rc.borrow().shown() {
        fltk::app::wait();
    }

    let return_value = *result.borrow();
    return_value
}