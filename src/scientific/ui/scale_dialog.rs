use fltk::{
    prelude::*,
    input::Input,
    window::Window,
    button::Button,
    group::{Pack, PackType},
    frame::Frame,
    enums::{Color, Align},
};
use std::rc::Rc;
use std::cell::RefCell;
use crate::state::ImageState;

pub fn show_scale_input_dialog(pixel_distance: f64, state: &Rc<RefCell<ImageState>>) -> Option<(f64, String, String)> {
    let result = Rc::new(RefCell::new(None));
    let mut win = Window::default()
        .with_size(300, 400)  // Made taller for buttons
        .with_label("Set Scale");
    win.set_color(Color::Background);
    
    let mut pack = Pack::default()
        .with_size(280, 350)
        .with_pos(10, 10);
    pack.set_spacing(10);  // Reduced spacing to fit all elements
    
    // Pixel distance display
    let pixel_frame = Pack::default().with_size(280, 40);
    Frame::default()
        .with_size(280, 20)
        .with_label("Pixel distance:");
    let mut pixel_value = Frame::default()
        .with_size(280, 20)
        .with_label(&format!("{:.2} px", pixel_distance));
    pixel_value.set_label_color(Color::Foreground);
    pixel_frame.end();
    
    // Known distance input
    let distance_pack = Pack::default().with_size(280, 40);
    Frame::default()
        .with_size(280, 20)
        .with_label("Known distance:");
    let mut distance_input = Input::default().with_size(280, 20);
    distance_input.set_color(Color::Light3);
    distance_input.set_text_color(Color::Black);
    distance_input.set_selection_color(Color::Selection);
    distance_pack.end();
    
    // Unit input
    let unit_pack = Pack::default().with_size(280, 40);
    Frame::default()
        .with_size(280, 20)
        .with_label("Unit:");
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
        .with_label("Objective:");
    let mut objective_input = Input::default().with_size(280, 20);
    objective_input.set_color(Color::Light3);
    objective_input.set_text_color(Color::Black);
    objective_input.set_value("5X");  // Default value
    
    // Get current objective if any
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
    
    // Button pack for all buttons
    let mut button_pack = Pack::default()
        .with_size(280, 80)  // Increased height for two rows of buttons
        .with_type(PackType::Vertical);
    button_pack.set_spacing(10);

    // First row of buttons
    let mut action_pack = Pack::default()
        .with_size(280, 35)
        .with_type(PackType::Horizontal);
    action_pack.set_spacing(10);

    let mut set_scale_btn = Button::default()
        .with_size(135, 35)
        .with_label("Set Scale");
    set_scale_btn.set_color(Color::Light3);

    let mut add_obj_btn = Button::default()
        .with_size(135, 35)
        .with_label("Add Objective");
    add_obj_btn.set_color(Color::Light3);
    action_pack.end();

    // Second row - Cancel button
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
    
    // Update preview when distance changes
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
    
    // Set Scale button callback
    set_scale_btn.set_callback({
        let mut win = win.clone();
        let distance_input = distance_input.clone();
        let unit_input = unit_input.clone();
        let objective_input = objective_input.clone();
        let result = result.clone();
        move |_| {
            if let Ok(real_distance) = distance_input.value().parse::<f64>() {
                *result.borrow_mut() = Some((
                    real_distance, 
                    unit_input.value(),
                    objective_input.value()
                ));
                win.hide();
            }
        }
    });

    // Add Objective button callback
    add_obj_btn.set_callback({
        let distance_input = distance_input.clone();
        let unit_input = unit_input.clone();
        let mut objective_input = objective_input.clone();
        let state = state.clone();
        move |_| {
            if let Ok(real_distance) = distance_input.value().parse::<f64>() {
                if let Ok(mut state_ref) = state.try_borrow_mut() {
                    state_ref.scientific_state.set_scale(
                        pixel_distance,
                        real_distance,
                        unit_input.value(),
                        Some(objective_input.value())
                    );
                }
                objective_input.set_value("");  // Clear for next objective
            }
        }
    });

    cancel_btn.set_callback({
        let mut win = win.clone();
        move |_| win.hide()
    });
    
    win.show();
    while win.shown() {
        fltk::app::wait();
    }
    
    let return_value = match &*result.borrow() {
        Some((dist, unit, obj)) => Some((
            *dist,
            unit.clone(),
            obj.clone()
        )),
        None => None
    };

    return_value
}