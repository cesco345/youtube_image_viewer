use fltk::{
    window::Window,
    button::{Button, CheckButton},
    frame::Frame,
    group::{Pack, Scroll, PackType},
    menu::Choice,
    prelude::*,
    enums::{Color, FrameType},
};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;

pub fn show_new_layer_dialog(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) -> bool {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        if state_ref.image.is_none() {
            return false;
        }
        state_ref.layer_state.update_groups();
    }

    let window = Window::default()
        .with_size(300, 400)
        .with_label("Layer Manager");
    let window = Rc::new(RefCell::new(window));
    window.borrow_mut().make_modal(true);

    let scroll = Scroll::new(10, 10, 280, 340, "");
    scroll.begin();
    
    let mut pack = Pack::new(0, 0, 260, 340, "");
    pack.set_spacing(5);

    if let Ok(state_ref) = state.try_borrow() {
        // create and add groups first
        for (group_id, group) in state_ref.layer_state.get_groups().iter().enumerate() {
            add_group_widget(&mut pack, group_id, group, state.clone(), frame.clone());
            // create and add layers under each group
            for &layer_index in &group.layer_indices {
                if let Some(layer) = state_ref.layer_state.get_layer(layer_index) {
                    add_layer_widget(&mut pack, layer_index, layer, state.clone(), frame.clone());
                }
            }
        }
    }

    pack.end();
    scroll.end();

    let mut color_choice = Choice::new(10, 360, 100, 25, "");
    color_choice.add_choice("Red|Green|Blue|Yellow|Purple");
    color_choice.set_value(0);

    let mut add_btn = Button::new(120, 360, 70, 25, "Add");
    let state_clone = state.clone();
    let frame_clone = frame.clone();
    let window_clone = window.clone();
    
    add_btn.set_callback(move |_| {
        let color = match color_choice.value() {
            0 => (255, 0, 0),    
            1 => (0, 255, 0),    
            2 => (0, 0, 255),    
            3 => (255, 255, 0),  
            4 => (255, 0, 255),  
            _ => (255, 0, 0),    
        };
        window_clone.borrow_mut().hide();
        super::handlers::handle_create_layer(&frame_clone, &state_clone, color);
    });

    let mut close_btn = Button::new(200, 360, 70, 25, "Close");
    let window_clone = window.clone();
    let state_clone = state.clone();
    let frame_clone = frame.clone();

    close_btn.set_callback(move |_| {
        if let Ok(mut state_ref) = state_clone.try_borrow_mut() {
            while state_ref.layer_state.get_layer_count() > 0 {
                state_ref.layer_state.remove_layer(0);
            }
            
            if let Some(original) = state_ref.layer_state.get_original_image() {
                let original_image = original.clone();
                state_ref.image = Some(original_image.clone());
                frame_clone.borrow_mut().set_image(Some(original_image));
                frame_clone.borrow_mut().redraw();
            }
        }
        window_clone.borrow_mut().hide();
    });

    window.borrow_mut().end();
    window.borrow_mut().show();

    while window.borrow().shown() {
        fltk::app::wait();
    }

    true
}

fn add_group_widget(
    pack: &mut Pack,
    group_id: usize,
    group: &crate::state::LayerGroup,
    state: Rc<RefCell<ImageState>>,
    frame: Rc<RefCell<Frame>>
) {
    let mut group_pack = Pack::new(0, 0, 260, 30, "");
    group_pack.set_type(PackType::Horizontal);
    group_pack.set_spacing(5);

    let mut visibility_check = CheckButton::new(0, 0, 30, 30, "");
    visibility_check.set_value(group.visible);
    let state_clone = state.clone();
    let frame_clone = frame.clone();
    
    visibility_check.set_callback(move |btn| {
        if let Ok(mut state_ref) = state_clone.try_borrow_mut() {
            state_ref.layer_state.set_group_visibility(group_id, btn.value());
            if let Some(composite) = state_ref.layer_state.get_composite_image() {
                state_ref.image = Some(composite.clone());
                frame_clone.borrow_mut().set_image(Some(composite));
                frame_clone.borrow_mut().redraw();
            }
        }
    });

    let mut color_preview = Frame::new(35, 0, 30, 30, "");
    color_preview.set_frame(FrameType::FlatBox);
    color_preview.set_color(Color::from_rgb(group.color.0, group.color.1, group.color.2));

    Frame::new(70, 0, 120, 30, &*group.name);
    
    group_pack.end();
}

fn add_layer_widget(
    pack: &mut Pack,
    index: usize,
    layer: &crate::state::Layer,
    state: Rc<RefCell<ImageState>>,
    frame: Rc<RefCell<Frame>>
) {
    let mut layer_pack = Pack::new(20, 0, 240, 30, "");  // Indented to show hierarchy
    layer_pack.set_type(PackType::Horizontal);
    layer_pack.set_spacing(5);

    let mut visibility_check = CheckButton::new(0, 0, 30, 30, "");
    visibility_check.set_value(layer.visible);
    let state_clone = state.clone();
    let frame_clone = frame.clone();
    visibility_check.set_callback(move |btn| {
        if let Ok(mut state_ref) = state_clone.try_borrow_mut() {
            state_ref.layer_state.set_layer_visibility(index, btn.value());
            if let Some(composite) = state_ref.layer_state.get_composite_image() {
                state_ref.image = Some(composite.clone());
                frame_clone.borrow_mut().set_image(Some(composite));
                frame_clone.borrow_mut().redraw();
            }
        }
    });

    let mut color_preview = Frame::new(35, 0, 30, 30, "");
    color_preview.set_frame(FrameType::FlatBox);
    color_preview.set_color(Color::from_rgb(layer.color.0, layer.color.1, layer.color.2));

    Frame::new(70, 0, 100, 30, &*layer.name);

    let mut delete_btn = Button::new(180, 0, 30, 30, "@1+");
    let state_clone = state.clone();
    let frame_clone = frame.clone();
    delete_btn.set_callback(move |_| {
        if let Ok(mut state_ref) = state_clone.try_borrow_mut() {
            if state_ref.layer_state.remove_layer(index) {
                if let Some(original) = state_ref.layer_state.get_original_image() {
                    if state_ref.layer_state.get_layer_count() == 0 {
                        let original_image = original.clone();
                        state_ref.image = Some(original_image.clone());
                        frame_clone.borrow_mut().set_image(Some(original_image));
                    } else if let Some(composite) = state_ref.layer_state.get_composite_image() {
                        state_ref.image = Some(composite.clone());
                        frame_clone.borrow_mut().set_image(Some(composite));
                    }
                    frame_clone.borrow_mut().redraw();
                }
            }
        }
    });

    layer_pack.end();
}