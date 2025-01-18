use fltk::{
    window::Window,
    button::Button,
    input::FloatInput,
    frame::Frame,
    group::Pack,
    prelude::*,
};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;

pub fn show_filter_dialog(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    filter_type: &str
) -> bool {
    let mut dialog = Window::default()
        .with_size(400, 300)
        .with_label(&format!("Apply {} Filter", filter_type));
    dialog.make_modal(true);

    let mut pack = Pack::new(10, 10, 380, 280, "");
    pack.set_spacing(10);

    // create the title frame based on filter type and add it to the pack
    Frame::default()
        .with_size(380, 25)
        .with_label(&format!("Adjusting {} filter", filter_type))
        .with_pos(0, 0);

    // the parameter label based on filter type and add it to the pack
    let param_label = match filter_type {
        "grayscale" | "sepia" | "invert" => "Intensity (0.0 - 1.0):",
        "brightness" => "Level (-1.0 to 1.0):",
        "contrast" | "saturation" => "Amount (0.0 - 2.0):",
        "threshold" => "Threshold (0.0 - 1.0):",
        "hue" => "Angle (0 - 360):",
        _ => "Value:",
    };
    
    Frame::default()
        .with_size(380, 25)
        .with_label(param_label)
        .with_pos(0, 35);

    let mut value_input = FloatInput::new(20, 60, 180, 25, "");
    
    // set default value based on filter type and add it to the pack
    let default_value = match filter_type {
        "brightness" => "0.0",
        "contrast" | "saturation" => "1.0",
        "hue" => "180.0",
        _ => "0.5",
    };
    value_input.set_value(default_value);

    let dialog_rc = Rc::new(RefCell::new(dialog));
    let result = Rc::new(RefCell::new(false));

    // the control buttons and add them to the pack
    let mut cancel = Button::new(190, 240, 85, 25, "Cancel");
    let mut ok = Button::new(285, 240, 85, 25, "Apply");

    pack.end();
    dialog_rc.borrow_mut().end();

    // closes the dialog without applying the filter and hides it
    let dialog_rc_cancel = dialog_rc.clone();
    cancel.set_callback(move |_| {
        dialog_rc_cancel.borrow_mut().hide();
    });

    // buttons and dialog input from the user - ok callback to apply the filter
    let dialog_rc_ok = dialog_rc.clone();
    let frame_rc = frame.clone();
    let state_rc = state.clone();
    let result_rc = result.clone();
    let filter_type = filter_type.to_string();

    ok.set_callback(move |_| {

        // parse and validate the input value and apply the filter
        if let Ok(value) = value_input.value().parse::<f32>() {

            // we need to apply value range constraints based on the filter type and apply the filter
            let adjusted_value = match filter_type.as_str() {
                "brightness" => value.clamp(-1.0, 1.0),
                "contrast" | "saturation" => value.clamp(0.0, 2.0),
                "hue" => value % 360.0,
                _ => value.clamp(0.0, 1.0),
            };

            match filter_type.as_str() {
                "grayscale" => super::handlers::handle_apply_grayscale(&frame_rc, &state_rc, adjusted_value),
                "sepia" => super::handlers::handle_apply_sepia(&frame_rc, &state_rc, adjusted_value),
                "brightness" => super::handlers::handle_apply_brightness(&frame_rc, &state_rc, adjusted_value),
                "contrast" => super::handlers::handle_apply_contrast(&frame_rc, &state_rc, adjusted_value),
                "saturation" => super::handlers::handle_apply_saturation(&frame_rc, &state_rc, adjusted_value),
              
                "threshold" => super::handlers::handle_apply_threshold(&frame_rc, &state_rc, adjusted_value),
                "hue" => super::handlers::handle_apply_hue(&frame_rc, &state_rc, adjusted_value),
                _ => println!("Unknown filter type: {}", filter_type),
            }

            *result_rc.borrow_mut() = true;
        } else {
            println!("Invalid input value");
        }
        dialog_rc_ok.borrow_mut().hide();
    });

    dialog_rc.borrow_mut().show();
    while dialog_rc.borrow().shown() {
        fltk::app::wait();
    }

    let final_result = *result.borrow();
    final_result
}