//src/scientific/ui/measurement_dialog.rs
use fltk::{
    window::Window,
    button::Button,
    input::FloatInput,
    group::Pack,
    menu::Choice,
    prelude::*,
};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;
use crate::scientific::{self, Annotation, AnnotationType};
use crate::scientific::state::scientific_state::ScientificState;

fn add_scale_bar(state: &mut ScientificState, length: f32, unit: String) {
    if let Some(img) = state.get_composite_image() {
        let annotation = Annotation {
            name: "Scale Bar".to_string(),
            image: img,
            annotation_type: AnnotationType::Scale {
                pixels_per_unit: length,
                unit,
            },
            visible: true,
            coordinates: vec![(0, 0)],
        };
        state.add_annotation(annotation);
    }
}

pub fn show_measurement_dialog(state: &Rc<RefCell<ImageState>>) -> bool {
    let mut window = Window::default()
        .with_size(300, 200)
        .with_label("Add Measurement");
    window.make_modal(true);
 
    let mut pack = Pack::new(10, 10, 280, 180, "");
    pack.set_spacing(10);
 
    let mut tool_choice = Choice::new(10, 10, 150, 25, "Tool:");
    tool_choice.add_choice("Scale Bar|Line Measurement|ROI|Text");
    
    let mut unit_choice = Choice::new(10, 45, 100, 25, "Unit:");
    unit_choice.add_choice("μm|mm|px");
    
    let value_input = FloatInput::new(120, 45, 70, 25, "Value:");
    
    let mut add_btn = Button::new(10, 150, 70, 25, "Add");
    let state_clone = state.clone();
    let window_ref = Rc::new(RefCell::new(window));
    let window_ref_add = window_ref.clone();
    add_btn.set_callback(move |_| {
        let mut state_ref = state_clone.borrow_mut();
        if let Ok(value) = value_input.value().parse::<f32>() {
            let unit = match unit_choice.value() {
                0 => "μm",
                1 => "mm",
                _ => "px",
            }.to_string();
            add_scale_bar(&mut state_ref.scientific_state, value, unit);
        }
        window_ref_add.borrow_mut().hide();
    });
 
    let mut cancel_btn = Button::new(90, 150, 70, 25, "Cancel");
    let window_ref_cancel = window_ref.clone();
    cancel_btn.set_callback(move |_| window_ref_cancel.borrow_mut().hide());
 
    pack.end();
    window_ref.borrow_mut().end();
    window_ref.borrow_mut().show();
 
    while window_ref.borrow().shown() {
        fltk::app::wait();
    }
 
    true
 }