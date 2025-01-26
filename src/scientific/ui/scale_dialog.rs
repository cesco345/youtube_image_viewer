use fltk::{
    prelude::*,
    input::Input,
    window::Window,
    button::Button,
    group::{Pack, PackType},
    frame::Frame,
    enums::{Color, Align},
    menu::Choice,
    button::CheckButton,
};
use crate::scientific::types::LegendPosition;
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
        .with_size(300, 500)
        .with_label("Set Scale");
    win.set_color(Color::Background);
    
    let mut pack = Pack::default()
        .with_size(280, 450)
        .with_pos(10, 10);
    pack.set_spacing(10);
    
    // Add legend options under the current inputs
    let legend_pack = Pack::default().with_size(280, 80);
    Frame::default()
        .with_size(280, 20)
        .with_label("Legend Position:");
    let mut legend_choice = Choice::default().with_size(280, 25);
    legend_choice.add_choice("Top Left|Top Right|Bottom Left|Bottom Right");
    
    // Set initial position based on current state
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
    
    // Set initial checkbox state based on current visibility
    if let Ok(state_ref) = state.try_borrow() {
        show_legend.set_checked(state_ref.scientific_state.show_legend);
    }
    
    legend_pack.end();
    
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
        .with_size(280, 80)
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
    
    // Update visibility immediately when checkbox changes
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

    // Update position immediately when choice changes
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
    
    // Set Scale button callback
    set_scale_btn.set_callback({
        let mut win = win.clone();
        let distance_input = distance_input.clone();
        let unit_input = unit_input.clone();
        let objective_input = objective_input.clone();
        let result = result.clone();
        let show_legend = show_legend.clone();
        let legend_choice = legend_choice.clone();
        let state = state.clone();
        let frame = frame.clone();
        move |_| {
            if let Ok(real_distance) = distance_input.value().parse::<f64>() {
                if let Ok(mut state_ref) = state.try_borrow_mut() {
                    state_ref.scientific_state.show_legend = show_legend.is_checked();
                    state_ref.scientific_state.legend_position = match legend_choice.value() {
                        0 => LegendPosition::TopLeft,
                        1 => LegendPosition::TopRight,
                        2 => LegendPosition::BottomLeft,
                        _ => LegendPosition::BottomRight,
                    };
                }
                *result.borrow_mut() = Some((
                    real_distance, 
                    unit_input.value(),
                    objective_input.value()
                ));
                frame.borrow_mut().redraw();
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
        let show_legend = show_legend.clone();
        let legend_choice = legend_choice.clone();
        let frame = frame.clone();
        move |_| {
            if let Ok(real_distance) = distance_input.value().parse::<f64>() {
                if let Ok(mut state_ref) = state.try_borrow_mut() {
                    state_ref.scientific_state.show_legend = show_legend.is_checked();
                    state_ref.scientific_state.legend_position = match legend_choice.value() {
                        0 => LegendPosition::TopLeft,
                        1 => LegendPosition::TopRight,
                        2 => LegendPosition::BottomLeft,
                        _ => LegendPosition::BottomRight,
                    };
                    state_ref.scientific_state.set_scale(
                        pixel_distance,
                        real_distance,
                        unit_input.value(),
                        Some(objective_input.value())
                    );
                }
                objective_input.set_value("");  // Clear for next objective
                frame.borrow_mut().redraw();
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
    // Fixed: Store the result in a temporary value before the Rc gets dropped
    let final_result = result.borrow().clone();
    final_result
}
    