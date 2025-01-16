use fltk::{
    window::Window,
    button::{Button, RadioButton},
    input::Input,
    frame::Frame,
    group::{Pack, Group},
    dialog::{FileDialog, FileDialogType},
    prelude::*,
};
use std::{rc::Rc, cell::RefCell, path::PathBuf};
use crate::state::ImageState;

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

    let dialog_result = Rc::new(RefCell::new(false));
    let selected_path = Rc::new(RefCell::new(None::<PathBuf>));
    
    // Control buttons at bottom
    let mut cancel = Button::new(190, 240, 85, 25, "Cancel");
    let mut ok = Button::new(285, 240, 85, 25, "Apply");

    pack.end();
    dialog.end();

    let dialog_rc = Rc::new(RefCell::new(dialog));
    
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
    let result_rc = dialog_result.clone();
    // OK callback in dialog.rs - replace the existing callback with this:
ok.set_callback(move |_| {
    if radio_image.is_set() {
        if let Some(path) = selected_path_rc.borrow().clone() {
            *result_rc.borrow_mut() = true;
            super::handlers::handle_add_watermark_from_path(&frame_rc, &state_rc, path);  // Changed this line
        }
    } else if radio_text.is_set() && !text_input_rc.value().is_empty() {
        *result_rc.borrow_mut() = true;
        super::handlers::handle_add_text_watermark(&frame_rc, &state_rc, text_input_rc.value());  // And this line
    }
    
    dialog_rc_ok.borrow_mut().hide();
});
    dialog_rc.borrow_mut().show();
    while dialog_rc.borrow().shown() {
        fltk::app::wait();
    }

    let final_result = *dialog_result.borrow();
    final_result
}

pub fn edit_watermark_dialog(_state: &Rc<RefCell<ImageState>>) -> bool {
    let mut dialog = Window::default()
        .with_size(400, 300)
        .with_label("Edit Watermark");
    dialog.make_modal(true);

    let mut pack = Pack::new(10, 10, 380, 280, "");
    pack.set_spacing(10);
    let dialog_result = Rc::new(RefCell::new(false));

    // Add buttons
    let mut cancel = Button::new(0, 0, 180, 25, "Cancel");
    let mut ok = Button::new(190, 0, 180, 25, "OK");

    pack.end();
    dialog.end();

    let dialog_rc = Rc::new(RefCell::new(dialog));

    {
        let dialog_rc = dialog_rc.clone();
        cancel.set_callback(move |_| {
            dialog_rc.borrow_mut().hide();
        });
    }

    {
        let dialog_rc = dialog_rc.clone();
        let result = dialog_result.clone();
        ok.set_callback(move |_| {
            *result.borrow_mut() = true;
            dialog_rc.borrow_mut().hide();
        });
    }

    dialog_rc.borrow_mut().show();
    while dialog_rc.borrow().shown() {
        fltk::app::wait();
    }

    let final_result = *dialog_result.borrow();
    final_result
}
