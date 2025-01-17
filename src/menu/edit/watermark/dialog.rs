use fltk::{
    window::Window,
    button::{Button, RadioButton},
    input::{Input, FloatInput},
    frame::Frame,
    group::{Pack, Group},
    menu::Choice,
    dialog::{FileDialog, FileDialogType},
    prelude::*,
};
use std::{rc::Rc, cell::RefCell, path::PathBuf};
use crate::state::ImageState;
use super::{BlendMode, WatermarkPosition, Position};

pub fn show_watermark_dialog(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) -> bool {
    let mut dialog = Window::default()
        .with_size(400, 300)
        .with_label("Add Watermark");
    dialog.make_modal(true);

    let mut pack = Pack::new(10, 10, 380, 280, "");
    pack.set_spacing(10);

    // Create radio group for watermark type
    let type_group = Group::new(0, 0, 380, 120, "");
    Frame::new(0, 0, 380, 25, "Choose Watermark Type:");
    
    let mut radio_image = RadioButton::new(20, 30, 120, 25, "Image File");
    let mut radio_text = RadioButton::new(20, 60, 120, 25, "Text");
    radio_image.set(true);
    radio_text.set_value(false);
    type_group.end();

    // File selection area
    let file_group = Group::new(0, 120, 380, 60, "");
    let file_path = Frame::new(20, 130, 340, 25, "No file selected");
    let mut file_btn = Button::new(20, 155, 150, 25, "Select Image File");
    file_group.end();

    // Text input area
    let mut text_group = Group::new(0, 120, 380, 60, "");
    Frame::new(20, 130, 70, 25, "Text:");
    let text_input = Input::new(90, 130, 270, 25, "");
    text_group.end();
    text_group.hide();

    let dialog_rc = Rc::new(RefCell::new(dialog));
    let result = Rc::new(RefCell::new(false));
    let selected_path = Rc::new(RefCell::new(None::<PathBuf>));
    
    // Control buttons at bottom
    let mut cancel = Button::new(190, 240, 85, 25, "Cancel");
    let mut ok = Button::new(285, 240, 85, 25, "Apply");

    pack.end();
    dialog_rc.borrow_mut().end();

    // Radio button callbacks
    let file_group_rc = Rc::new(RefCell::new(file_group));
    let text_group_rc = Rc::new(RefCell::new(text_group));
    
    radio_image.set_callback({
        let file_group_rc = file_group_rc.clone();
        let text_group_rc = text_group_rc.clone();
        move |_| {
            file_group_rc.borrow_mut().show();
            text_group_rc.borrow_mut().hide();
        }
    });

    radio_text.set_callback({
        let file_group_rc = file_group_rc.clone();
        let text_group_rc = text_group_rc.clone();
        move |_| {
            file_group_rc.borrow_mut().hide();
            text_group_rc.borrow_mut().show();
        }
    });

    // File button callback
    let selected_path_rc = selected_path.clone();
    let file_path_rc = Rc::new(RefCell::new(file_path));
    file_btn.set_callback(move |_| {
        let mut file_dialog = FileDialog::new(FileDialogType::BrowseFile);
        file_dialog.set_filter("Image Files\t*.{jpg,jpeg,png,gif,bmp}");
        file_dialog.show();
        
        if let Some(path) = file_dialog.filename().to_str() {
            *selected_path_rc.borrow_mut() = Some(PathBuf::from(path));
            file_path_rc.borrow_mut().set_label(path);
        }
    });

    // Cancel callback
    let dialog_rc_cancel = dialog_rc.clone();
    cancel.set_callback(move |_| {
        dialog_rc_cancel.borrow_mut().hide();
    });

    // OK callback
    let dialog_rc_ok = dialog_rc.clone();
    let frame_rc = frame.clone();
    let state_rc = state.clone();
    let text_input_rc = text_input.clone();
    let selected_path_rc = selected_path.clone();
    let result_rc = result.clone();

    ok.set_callback(move |_| {
        if radio_image.is_set() {
            if let Some(path) = selected_path_rc.borrow().clone() {
                *result_rc.borrow_mut() = true;
                super::handlers::handle_add_watermark_from_path(&frame_rc, &state_rc, path);
            }
        } else if radio_text.is_set() && !text_input_rc.value().is_empty() {
            *result_rc.borrow_mut() = true;
            super::handlers::handle_add_text_watermark(&frame_rc, &state_rc, text_input_rc.value());
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

pub fn show_edit_watermark_dialog(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) -> bool {
    let mut dialog = Window::default()
        .with_size(400, 400)
        .with_label("Edit Watermark");
    dialog.make_modal(true);

    let current_options = {
        let state_ref = state.borrow();
        let has_watermark = state_ref.watermark_state.has_watermark();
        if !has_watermark {
            return false;
        }
        state_ref.get_watermark_options()
    };

    let mut pack = Pack::new(10, 10, 380, 380, "");
    pack.set_spacing(10);

    // Position selection
    Frame::new(0, 0, 380, 25, "Position:");
    let mut pos_choice = Choice::new(20, 35, 180, 25, "");
    pos_choice.add_choice("Top Left|Top Right|Bottom Left|Bottom Right|Center");
    let _ = match current_options.position {
        WatermarkPosition::TopLeft(_) => pos_choice.set_value(0),
        WatermarkPosition::TopRight(_) => pos_choice.set_value(1),
        WatermarkPosition::BottomLeft(_) => pos_choice.set_value(2),
        WatermarkPosition::BottomRight(_) => pos_choice.set_value(3),
        WatermarkPosition::Center(_) => pos_choice.set_value(4),
        _ => pos_choice.set_value(0),
    };

    // Opacity control
    Frame::new(0, 70, 380, 25, "Opacity (0.0 - 1.0):");
    let mut opacity_input = FloatInput::new(20, 95, 180, 25, "");
    opacity_input.set_value(&format!("{:.2}", current_options.opacity));

    // Blend mode selection
    Frame::new(0, 130, 380, 25, "Blend Mode:");
    let mut blend_choice = Choice::new(20, 155, 180, 25, "");
    blend_choice.add_choice("Normal|Multiply|Screen|Overlay");
    let _ = match current_options.blend_mode {
        BlendMode::Normal => blend_choice.set_value(0),
        BlendMode::Multiply => blend_choice.set_value(1),
        BlendMode::Screen => blend_choice.set_value(2),
        BlendMode::Overlay => blend_choice.set_value(3),
        _ => blend_choice.set_value(0),
    };

    // Scale control
    Frame::new(0, 190, 380, 25, "Scale:");
    let mut scale_input = FloatInput::new(20, 215, 180, 25, "");
    if let Some(scale) = current_options.scale {
        scale_input.set_value(&format!("{:.2}", scale));
    }

    let dialog_rc = Rc::new(RefCell::new(dialog));
    let result = Rc::new(RefCell::new(false));

    // Control buttons
    let mut cancel = Button::new(190, 350, 85, 25, "Cancel");
    let mut ok = Button::new(285, 350, 85, 25, "Apply");

    pack.end();
    dialog_rc.borrow_mut().end();

    // Cancel callback
    let dialog_rc_cancel = dialog_rc.clone();
    cancel.set_callback(move |_| {
        dialog_rc_cancel.borrow_mut().hide();
    });

    // OK callback
    let dialog_rc_ok = dialog_rc.clone();
    let result_rc = result.clone();
    let frame_rc = frame.clone();
    let state_rc = state.clone();

    ok.set_callback(move |_| {
        let mut new_options = current_options.clone();

        // Update position
        new_options.position = match pos_choice.value() {
            0 => WatermarkPosition::TopLeft(Position::new(20, 20)),
            1 => WatermarkPosition::TopRight(Position::new(20, 20)),
            2 => WatermarkPosition::BottomLeft(Position::new(20, 20)),
            3 => WatermarkPosition::BottomRight(Position::new(20, 20)),
            4 => WatermarkPosition::Center(Position::new(0, 0)),
            _ => WatermarkPosition::TopLeft(Position::new(20, 20)),
        };

        // Update opacity
        if let Ok(opacity) = opacity_input.value().parse::<f32>() {
            new_options.opacity = opacity.clamp(0.0, 1.0);
        }

        // Update blend mode
        new_options.blend_mode = match blend_choice.value() {
            0 => BlendMode::Normal,
            1 => BlendMode::Multiply,
            2 => BlendMode::Screen,
            3 => BlendMode::Overlay,
            _ => BlendMode::Normal,
        };

        // Update scale
        if let Ok(scale) = scale_input.value().parse::<f32>() {
            new_options.scale = Some(scale.max(0.0));
        }

        // Apply changes
        if let Ok(mut state_ref) = state_rc.try_borrow_mut() {
            if state_ref.watermark_state.update_watermark_options(new_options).is_ok() {
                if let Some(image) = state_ref.image.clone() {
                    if let Ok(Some(new_image)) = state_ref.watermark_state.apply_watermark(&image) {
                        frame_rc.borrow_mut().set_image(Some(new_image.clone()));
                        frame_rc.borrow_mut().redraw();
                        state_ref.image = Some(new_image);
                    }
                }
            }
        }

        *result_rc.borrow_mut() = true;
        dialog_rc_ok.borrow_mut().hide();
    });

    dialog_rc.borrow_mut().show();
    while dialog_rc.borrow().shown() {
        fltk::app::wait();
    }

    let final_result = *result.borrow();
    final_result
}
