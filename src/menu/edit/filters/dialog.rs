// src/menu/edit/filters/dialog.rs

// Standard library imports
use std::{rc::Rc, cell::RefCell};

// FLTK imports
use fltk::{
    window::Window,
    button::Button,
    input::FloatInput,
    frame::Frame,
    group::Pack,
    menu::Choice,
    prelude::*,
};

// Internal state imports
use crate::state::ImageState;

// Filter methods and tools
use super::start_interactive_pixelate;
use super::start_interactive_edge_detection;
use super::start_interactive_noise;
use super::start_interactive_vignette;
use super::start_interactive_posterize;
use super::start_interactive_motion_blur;

// Advanced filter types
use super::advanced::EdgeDetectionMethod;

// Handler functions
use super::handlers::{
    handle_apply_grayscale,
    handle_apply_sepia,
    handle_apply_brightness,
    handle_apply_contrast,
    handle_apply_saturation,
    handle_apply_threshold,
    handle_apply_hue,
    handle_apply_box_blur,
    handle_apply_gaussian_blur,
    handle_apply_sharpen,
    //handle_apply_vignette,
    handle_apply_posterize,
    handle_apply_motion_blur,
};

pub fn show_filter_dialog(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    filter_type: &str
) -> bool {
    // Create a taller window to accommodate instructions
    let mut dialog = Window::default()
        .with_size(400, 380)  // Increased height
        .with_label(&format!("Apply {} Filter", filter_type));
    dialog.make_modal(true);

    let mut pack = Pack::new(10, 10, 380, 360, "");  // Increased height
    pack.set_spacing(10);

    // Title frame
    Frame::default()
        .with_size(380, 25)
        .with_label(&format!("Adjusting {} filter", filter_type))
        .with_pos(0, 0);

    // Instructions frame at the top
    let instruction_text = "After clicking Apply, click and drag on the image\nto select the area where you want to apply the filter.";
    Frame::default()
        .with_size(380, 40)
        .with_label(instruction_text)
        .with_pos(0, 35);

    // Add method choice for edge detection
    let mut method_choice = None;
    if filter_type == "edge_detection" {
        Frame::default()
            .with_size(380, 25)
            .with_label("Edge Detection Method:")
            .with_pos(0, 85);  // Adjusted position

        let mut choice = Choice::new(20, 110, 180, 25, "");  // Adjusted position
        choice.add_choice("Sobel|Canny");
        choice.set_value(0);  // Default to Sobel
        method_choice = Some(choice);

        Frame::default()
            .with_size(380, 25)
            .with_label("Threshold (0.0 - 1.0):")
            .with_pos(0, 145);  // Adjusted position
    }

    let param_label = match filter_type {
        "grayscale" | "sepia" | "invert" => "Intensity (0.0 - 1.0):",
        "brightness" => "Level (-1.0 to 1.0):",
        "contrast" | "saturation" => "Amount (0.0 - 2.0):",
        "threshold" => "Threshold (0.0 - 1.0):",
        "hue" => "Angle (0 - 360):",
        "box_blur" | "gaussian_blur" => "Radius (1.0 - 10.0):",
        "sharpen" => "Intensity (0.0 - 5.0):",
        "edge_detection" => "",  // Already handled above
        "noise" => "Amount (0.0 - 1.0):",
        "vignette" => "Intensity (0.0 - 1.0):",
        "posterize" => "Levels (2 - 8):",
        "pixelate" => "Block Size (2 - 32):",
        "motion_blur" => "Angle (0 - 360):",    
        _ => "Value:",
    };
    
    if filter_type != "edge_detection" {
        Frame::default()
            .with_size(380, 25)
            .with_label(param_label)
            .with_pos(0, 85);  // Adjusted position
    }

    let mut value_input = FloatInput::new(
        20, 
        if filter_type == "edge_detection" { 170 } else { 110 },  // Adjusted position
        180, 
        25, 
        ""
    );
    
    let default_value = match filter_type {
        "brightness" => "0.0",
        "contrast" | "saturation" => "1.0",
        "hue" => "180.0",
        "box_blur" | "gaussian_blur" => "3.0",
        "sharpen" => "1.0",
        "edge_detection" | "noise" | "vignette" => "0.5",
        "posterize" => "4",
        "pixelate" => "8",
        "motion_blur" => "45",
        _ => "0.5",
    };
    value_input.set_value(default_value);

    let dialog_rc = Rc::new(RefCell::new(dialog));
    let result = Rc::new(RefCell::new(false));

    let mut cancel = Button::new(190, 320, 85, 25, "Cancel");  // Adjusted position
    let mut ok = Button::new(285, 320, 85, 25, "Apply");      // Adjusted position

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
    let filter_type = filter_type.to_string();

    ok.set_callback(move |_| {
        if let Ok(value) = value_input.value().parse::<f32>() {
            let adjusted_value = match filter_type.as_str() {
                "brightness" => value.clamp(-1.0, 1.0),
                "contrast" | "saturation" => value.clamp(0.0, 2.0),
                "hue" => value % 360.0,
                "box_blur" | "gaussian_blur" => value.clamp(1.0, 10.0),
                "sharpen" => value.clamp(0.0, 5.0),
                "edge_detection" | "noise" | "vignette" => value.clamp(0.0, 1.0),
                "posterize" => value.clamp(2.0, 8.0),
                "pixelate" => value.clamp(2.0, 32.0),
                "motion_blur" => value % 360.0,
                _ => value.clamp(0.0, 1.0),
            };
        
            match filter_type.as_str() {
                "grayscale" => handle_apply_grayscale(&frame_rc, &state_rc, adjusted_value),
                "sepia" => handle_apply_sepia(&frame_rc, &state_rc, adjusted_value),
                "brightness" => handle_apply_brightness(&frame_rc, &state_rc, adjusted_value),
                "contrast" => handle_apply_contrast(&frame_rc, &state_rc, adjusted_value),
                "saturation" => handle_apply_saturation(&frame_rc, &state_rc, adjusted_value),
                "threshold" => handle_apply_threshold(&frame_rc, &state_rc, adjusted_value),
                "hue" => handle_apply_hue(&frame_rc, &state_rc, adjusted_value),
                "box_blur" => handle_apply_box_blur(&frame_rc, &state_rc, adjusted_value),
                "gaussian_blur" => handle_apply_gaussian_blur(&frame_rc, &state_rc, adjusted_value),
                "sharpen" => handle_apply_sharpen(&frame_rc, &state_rc, adjusted_value),
                "edge_detection" => {
                    let method = if let Some(ref choice) = method_choice {
                        match choice.value() {
                            0 => EdgeDetectionMethod::Sobel,
                            1 => EdgeDetectionMethod::Canny,
                            _ => EdgeDetectionMethod::Sobel,
                        }
                    } else {
                        EdgeDetectionMethod::Sobel
                    };
                    start_interactive_edge_detection(&frame_rc, &state_rc, adjusted_value, method);
                },
                "noise" => start_interactive_noise(&frame_rc, &state_rc, adjusted_value),
                "vignette" => start_interactive_vignette(&frame_rc, &state_rc, adjusted_value),
                "posterize" => start_interactive_posterize(&frame_rc, &state_rc, adjusted_value as u8),
                "pixelate" => {
                    let block_size = adjusted_value as u32;
                    start_interactive_pixelate(&frame_rc, &state_rc, block_size);
                },
                "motion_blur" => start_interactive_motion_blur(&frame_rc, &state_rc, adjusted_value),
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

    let return_value = *result.borrow();
    return_value
}