use fltk::{
    window::Window,
    button::Button,
    group::Pack,
    menu::Choice,
    input::FloatInput,
    prelude::*,
    frame::Frame,
};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;
use crate::scientific::types::{ROITool, ROIShape}; // Import from types module

pub fn show_roi_dialog(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) -> bool {
    let window = Rc::new(RefCell::new(
        Window::default()
            .with_size(300, 200)
            .with_label("ROI Tool")
    ));
    
    window.borrow_mut().make_modal(true);

    let mut pack = Pack::new(10, 10, 280, 180, "");
    pack.set_spacing(10);

    let mut shape_choice = Choice::new(10, 10, 150, 25, "Shape:");
    shape_choice.add_choice("Rectangle|Ellipse|Polygon|Line");
    shape_choice.set_value(0); // Default to Rectangle
    
    let mut width_input = FloatInput::new(120, 45, 70, 25, "Width:");
    let mut height_input = FloatInput::new(120, 80, 70, 25, "Height:");
    width_input.set_value("100");  // Default width
    height_input.set_value("100"); // Default height
    
    let mut add_btn = Button::new(10, 150, 70, 25, "Add");
    let state_clone = state.clone();
    let frame_clone = frame.clone();
    let width_input = Rc::new(width_input);
    let height_input = Rc::new(height_input);
    let shape_choice = Rc::new(shape_choice);
    let window_clone = window.clone();
    
    {
        let width_input = width_input.clone();
        let height_input = height_input.clone();
        let shape_choice = shape_choice.clone();
        
        add_btn.set_callback(move |_| {
            let mut state_ref = state_clone.borrow_mut();
            let shape = match shape_choice.value() {
                0 => ROIShape::Rectangle {
                    width: width_input.value().parse().unwrap_or(100),
                    height: height_input.value().parse().unwrap_or(100),
                },
                1 => ROIShape::Ellipse {
                    width: width_input.value().parse().unwrap_or(100),
                    height: height_input.value().parse().unwrap_or(100),
                },
                2 => ROIShape::Polygon {
                    points: vec![], // Will be set by mouse interaction
                },
                _ => ROIShape::Line {
                    points: vec![], // Will be set by mouse interaction
                },
            };
            
            let roi_tool = ROITool::new(
                shape,
                (255, 0, 0), // Red color
                2,          // Line width
            );
            
            state_ref.scientific_state.set_roi_tool(roi_tool);
            frame_clone.borrow_mut().redraw();
            window_clone.borrow_mut().hide(); // Hide dialog after adding ROI
        });
    }

    let mut cancel_btn = Button::new(90, 150, 70, 25, "Cancel");
    let window_clone = window.clone();
    cancel_btn.set_callback(move |_| window_clone.borrow_mut().hide());

    pack.end();
    window.borrow_mut().end();
    window.borrow_mut().show();

    while window.borrow().shown() {
        fltk::app::wait();
    }

    true
}