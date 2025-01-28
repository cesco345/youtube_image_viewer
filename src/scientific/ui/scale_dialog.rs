use fltk::{
    dialog::{FileDialog, FileDialogType},
    prelude::*,
    input::{Input, FloatInput},
    window::Window,
    button::Button,
    group::{Pack, PackType},
    frame::Frame,
    enums::{Color, Align},
    menu::Choice,
    button::CheckButton,
};

use crate::scientific::types::LegendPosition;
use crate::scientific::reporting::CalibrationReport;
use std::rc::Rc;
use std::cell::RefCell;
use crate::state::ImageState;

pub fn show_scale_input_dialog(
    pixel_distance: f64, 
    state: &Rc<RefCell<ImageState>>,
    frame: &Rc<RefCell<Frame>>,
) -> Option<(f64, String, String)> {
    let result = Rc::new(RefCell::new(None));
    let mut win = Window::default()
        .with_size(350, 640)
        .with_label("Set Scale");
    win.set_color(Color::Background);
    let mut win = win.center_screen();
    
    let mut pack = Pack::default()
        .with_size(280, 580)
        .with_pos(10, 10);
    pack.set_spacing(10);

    // Instructions at the top
    let instructions_pack = Pack::default().with_size(280, 100);
    let mut instructions = Frame::default()
        .with_size(280, 100)
        .with_label(
            "How to calibrate multiple objectives:\n\
            1. Draw a line on a known distance\n\
            2. Enter real distance and objective (e.g. 5X)\n\
            3. Click 'Add Objective' to store and measure next\n\
            4. Repeat steps 1-3 for each objective\n\
            5. Click 'Finalize' when done with all objectives"
        );
    instructions.set_align(Align::Left | Align::Inside | Align::Top);
    instructions.set_label_color(Color::Foreground);
    instructions_pack.end();
    
    // Image name input
    let image_name_pack = Pack::default().with_size(280, 40);
    Frame::default()
        .with_size(280, 20)
        .with_label("Image Name:");
    let mut image_name_input = Input::default().with_size(280, 20);
    image_name_input.set_color(Color::Light3);
    image_name_input.set_text_color(Color::Black);
    image_name_input.set_value("Unknown");
    image_name_pack.end();
    
    // Legend position pack
    let legend_pack = Pack::default().with_size(280, 80);
    Frame::default()
        .with_size(280, 20)
        .with_label("Legend Position:");
    let mut legend_choice = Choice::default().with_size(280, 25);
    legend_choice.add_choice("Top Left|Top Right|Bottom Left|Bottom Right");
    
    if let Ok(state_ref) = state.try_borrow() {
        legend_choice.set_value(match state_ref.scientific_state.legend_position {
            LegendPosition::TopLeft => 0,
            LegendPosition::TopRight => 1,
            LegendPosition::BottomLeft => 2,
            LegendPosition::BottomRight => 3,
        });
    }

    let mut show_legend = CheckButton::default()
        .with_size(280, 25)
        .with_label("Show Scale Legend");
    
    if let Ok(state_ref) = state.try_borrow() {
        show_legend.set_checked(state_ref.scientific_state.show_legend);
    }
    legend_pack.end();
    
    // Pixel distance display
    let pixel_frame = Pack::default().with_size(280, 40);
    Frame::default()
        .with_size(280, 20)
        .with_label("Pixel distance (measured):");
    let mut pixel_value = Frame::default()
        .with_size(280, 20)
        .with_label(&format!("{:.2} px", pixel_distance));
    pixel_value.set_label_color(Color::Foreground);
    pixel_frame.end();
    
    // Known distance input
    let distance_pack = Pack::default().with_size(280, 40);
    Frame::default()
        .with_size(280, 20)
        .with_label("Known distance (real world):");
    let mut distance_input = Input::default().with_size(280, 20);
    distance_input.set_color(Color::Light3);
    distance_input.set_text_color(Color::Black);
    distance_input.set_selection_color(Color::Selection);
    distance_pack.end();
    
    // Unit input with default value
    let unit_pack = Pack::default().with_size(280, 40);
    Frame::default()
        .with_size(280, 20)
        .with_label("Unit (e.g., µm, mm):");
    let mut unit_input = Input::default().with_size(280, 20);
    unit_input.set_value("µm");
    unit_input.set_color(Color::Light3);
    unit_input.set_text_color(Color::Black);
    unit_input.set_selection_color(Color::Selection);
    unit_pack.end();

    // Objective input
    let objective_pack = Pack::default().with_size(280, 40);
    Frame::default()
        .with_size(280, 20)
        .with_label("Objective (e.g., 5X, 10X):");
    let mut objective_input = Input::default().with_size(280, 20);
    objective_input.set_color(Color::Light3);
    objective_input.set_text_color(Color::Black);
    objective_input.set_value("5X");
    
    if let Ok(state_ref) = state.try_borrow() {
        if let Some(channel) = state_ref.scientific_state.channels.first() {
            if let Some(obj) = &channel.metadata.objective {
                objective_input.set_value(obj);
            }
        }
    }
    objective_pack.end();
    
    // Scale preview
    let preview_pack = Pack::default().with_size(280, 30);
    let mut preview_label = Frame::default()
        .with_size(280, 30)
        .with_align(Align::Center);
    preview_label.set_label_color(Color::Foreground);
    preview_pack.end();
    
    // Button pack
    let mut button_pack = Pack::default()
        .with_size(280, 120)
        .with_type(PackType::Vertical);
    button_pack.set_spacing(10);

    // Action buttons
    let mut action_pack = Pack::default()
        .with_size(280, 35)
        .with_type(PackType::Horizontal);
    action_pack.set_spacing(10);

    let mut set_scale_btn = Button::default()
        .with_size(135, 35)
        .with_label("Finalize All");
    set_scale_btn.set_color(Color::Light3);
    set_scale_btn.set_tooltip("Save all calibrations and finish");

    let mut add_obj_btn = Button::default()
        .with_size(135, 35)
        .with_label("Add Objective");
    add_obj_btn.set_color(Color::Light3);
    add_obj_btn.set_tooltip("Store current calibration and measure next");
    action_pack.end();

    // Export button
    let export_pack = Pack::default()
        .with_size(280, 35)
        .with_type(PackType::Horizontal);
    let mut export_btn = Button::default()
        .with_size(280, 35)
        .with_label("Export Report");
    export_btn.set_color(Color::Light3);
    export_btn.set_tooltip("Save all stored calibrations to a file");
    export_pack.end();

    // Cancel button
    let cancel_pack = Pack::default()
        .with_size(280, 35)
        .with_type(PackType::Horizontal);
    let mut cancel_btn = Button::default()
        .with_size(280, 35)
        .with_label("Cancel");
    cancel_btn.set_color(Color::Light3);
    cancel_pack.end();

    button_pack.end();
    pack.end();
    win.end();
    win.make_modal(true);
    
    // Callbacks
    distance_input.set_callback({
        let unit_input = unit_input.clone();
        let mut preview_label = preview_label.clone();
        move |input| {
            if let Ok(value) = input.value().parse::<f64>() {
                let scale = pixel_distance / value;
                preview_label.set_label(&format!("Scale: {:.2} px/{}", 
                    scale, unit_input.value()));
            }
        }
    });
    
    show_legend.set_callback({
        let state = state.clone();
        let frame = frame.clone();
        move |btn| {
            if let Ok(mut state_ref) = state.try_borrow_mut() {
                state_ref.scientific_state.show_legend = btn.is_checked();
                frame.borrow_mut().redraw();
            }
        }
    });

    legend_choice.set_callback({
        let state = state.clone();
        let frame = frame.clone();
        move |choice| {
            if let Ok(mut state_ref) = state.try_borrow_mut() {
                let position = match choice.value() {
                    0 => LegendPosition::TopLeft,
                    1 => LegendPosition::TopRight,
                    2 => LegendPosition::BottomLeft,
                    _ => LegendPosition::BottomRight,
                };
                state_ref.scientific_state.set_legend_position(position);
                frame.borrow_mut().redraw();
            }
        }
    });

    // Add Objective callback
    add_obj_btn.set_callback({
        let mut distance_input = distance_input.clone();
        let unit_input = unit_input.clone();
        let mut objective_input = objective_input.clone();
        let image_name_input = image_name_input.clone();
        let state = state.clone();
        let frame = frame.clone();
        let mut win = win.clone();
        move |_| {
            if let Ok(real_distance) = distance_input.value().parse::<f64>() {
                if let Ok(mut state_ref) = state.try_borrow_mut() {
                    state_ref.scientific_state.add_objective_calibration(
                        objective_input.value(),
                        real_distance,
                        unit_input.value(),
                        pixel_distance,
                        Some(image_name_input.value())
                    );
                    
                    fltk::dialog::message_default(&format!(
                        "Calibration stored for {}:\n{:.2} pixels/{}\n\nDraw a new measurement line for the next objective.", 
                        objective_input.value(),
                        pixel_distance / real_distance,
                        unit_input.value()
                    ));

                    // Clear the form for next objective
                    objective_input.set_value("");
                    distance_input.set_value("");
                    win.hide(); // Hide dialog to allow new measurement
                }
            }
        }
    });

    // Set Scale (Finalize) callback
    set_scale_btn.set_callback({
        let distance_input = distance_input.clone();
        let unit_input = unit_input.clone();
        let objective_input = objective_input.clone();
        let image_name_input = image_name_input.clone();
        let result = result.clone();
        let show_legend = show_legend.clone();
        let legend_choice = legend_choice.clone();
        let state = state.clone();
        let frame = frame.clone();
        let mut win = win.clone();
        move |_| {
            if let Ok(real_distance) = distance_input.value().parse::<f64>() {
                if let Ok(mut state_ref) = state.try_borrow_mut() {
                    // Update legend settings
                    state_ref.scientific_state.show_legend = show_legend.is_checked();
                    state_ref.scientific_state.legend_position = match legend_choice.value() {
                        0 => LegendPosition::TopLeft,
                        1 => LegendPosition::TopRight,
                        2 => LegendPosition::BottomLeft,
                        _ => LegendPosition::BottomRight,
                    };
                    
                    // Add final calibration if form is filled
                    if !objective_input.value().is_empty() {
                        state_ref.scientific_state.add_objective_calibration(
                            objective_input.value(),
                            real_distance,
                            unit_input.value(),
                            pixel_distance,
                            Some(image_name_input.value())
                        );
                    }
    
                    // Store result
                    *result.borrow_mut() = Some((
                        real_distance, 
                        unit_input.value(),
                        objective_input.value()
                    ));
                    
                    // Ensure frame is redrawn before dialog closes
                    frame.borrow_mut().redraw();
    
                    // Show confirmation dialog
                    match fltk::dialog::choice2_default(
                        "Calibrations have been finalized. Would you like to export now?",
                        "Export",
                        "Close",
                        "Keep Editing"
                    ) {
                        Some(0) => {}, // Export - keep dialog open
                        Some(1) => {
                            frame.borrow_mut().redraw(); // Redraw frame before hiding
                            win.hide();
                        }, 
                        _ => {} // Keep editing or dialog closed
                    }
                }
            }
        }
    });

    // Export callback
    // Export callback
export_btn.set_callback({
    let state = state.clone();
    let frame = frame.clone();
    move |_| {
        if let Ok(state_ref) = state.try_borrow() {
            if state_ref.scientific_state.calibrations.is_empty() {
                fltk::dialog::alert_default(
                    "No calibrations to export.\n\n\
                    To add calibrations:\n\
                    1. Draw a line on your image\n\
                    2. Enter the known distance\n\
                    3. Click 'Add Objective' to store it\n\
                    4. Repeat for different objectives"
                );
                return;
            }
            
            let calibrations = state_ref.scientific_state.calibrations.clone();
            let report = CalibrationReport::new(calibrations);
            
            let mut dialog = FileDialog::new(FileDialogType::BrowseSaveFile);
            dialog.set_title("Save Calibration Report");
            dialog.set_filter("*.csv\t*.md");
            dialog.show();
            
            if let Some(path) = dialog.filename().to_str() {
                let export_result = if path.ends_with(".csv") {
                    report.export_csv(path)
                } else if path.ends_with(".md") {
                    report.export_markdown(path)
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "File must end with .csv or .md"
                    ))
                };

                match export_result {
                    Ok(_) => {
                        frame.borrow_mut().redraw();
                        fltk::dialog::message_default("Calibration report exported successfully!");
                    }
                    Err(e) => {
                        frame.borrow_mut().redraw();
                        fltk::dialog::alert_default(&format!("Failed to export: {}", e));
                    }
                }
            }
        }
    }
});

    // Cancel callback
    cancel_btn.set_callback({
        let mut win = win.clone();
        let frame = frame.clone();
        move |_| {
            frame.borrow_mut().redraw(); // Add this line
            win.hide();
        }
    });
    
    win.show();
    while win.shown() {
        match fltk::app::wait() {
            true => continue,
            false => {
                println!("Event loop error");
                return None;
            }
        }
    }
    
    // Return the final result
    let final_result = result.borrow().clone();
    drop(result);
    final_result
}