use fltk::{
    enums::Event,
    frame::Frame,
    prelude::*,
 };
 use std::{rc::Rc, cell::RefCell};
 use crate::state::ImageState;
 use crate::scientific::ui::show_scale_input_dialog;
 use crate::scientific::rendering::{ScaleRenderer, frame_renderer::FrameRenderer};
 
 pub struct InteractiveScale {
    points: Vec<(i32, i32)>,
 }
 
 impl InteractiveScale {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }
 }
 
 pub fn start_interactive_scale(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    // Show instruction message
    fltk::dialog::message_default(
        "Click and drag to draw a line along a known distance in your image.\n\
        Release the mouse button when done."
    );
    let interactive = Rc::new(RefCell::new(InteractiveScale::new()));
    let interactive_draw = interactive.clone();
    let state_draw = state.clone();
    
    frame.borrow_mut().draw(move |f| {
        let state_ref = state_draw.borrow();
        
        // Draw base image only once
        if let Some(mut draw_img) = state_ref.image.clone() {
            draw_img.draw(f.x(), f.y(), f.width(), f.height());
            
            // Draw existing scale if enabled
            if state_ref.scientific_state.show_legend {
                ScaleRenderer::draw_legend(
                    f.x(),
                    f.y(),
                    f.width(), 
                    f.height(),
                    state_ref.scientific_state.legend_position,
                    &state_ref.scientific_state.calibration.unit,
                    state_ref.scientific_state.calibration.pixels_per_unit,
                );
            }
            
            // Draw interactive preview
            let points = interactive_draw.borrow().points.clone();
            if !points.is_empty() {
                ScaleRenderer::draw_scale_preview(
                    points[0],
                    points.get(1).copied(),
                    &state_ref.scientific_state.calibration.unit
                );
            }
        }
    });
 
    // Set up event handling
    frame.borrow_mut().handle({
        let interactive = interactive.clone();
        let state = state.clone();
        let frame = frame.clone();
        
        move |_, ev| match ev {
            Event::Push => {
                let coords = fltk::app::event_coords();
                interactive.borrow_mut().points.clear();
                interactive.borrow_mut().points.push(coords);
                frame.borrow_mut().redraw();
                true
            },
            Event::Drag => {
                let coords = fltk::app::event_coords();
                if interactive.borrow().points.len() == 1 {
                    interactive.borrow_mut().points.push(coords);
                } else if interactive.borrow().points.len() == 2 {
                    interactive.borrow_mut().points[1] = coords;
                }
                frame.borrow_mut().redraw();
                true
            },
            Event::Released => {
                let points = interactive.borrow().points.clone();
                if points.len() >= 2 {
                    let (x1, y1) = points[0];
                    let (x2, y2) = points[1];
                    let pixel_distance = ((x2 - x1).pow(2) as f64 + 
                                       (y2 - y1).pow(2) as f64).sqrt();
                    
                    if let Some((real_distance, unit, objective)) = show_scale_input_dialog(pixel_distance, &state, &frame) {
                        if let Ok(mut state_ref) = state.try_borrow_mut() {
                            state_ref.scientific_state.set_scale(
                                pixel_distance,
                                real_distance,
                                unit,
                                Some(objective)
                            );
                            FrameRenderer::setup_scientific_frame(&frame, &state);
                        }
                    }
                }
                interactive.borrow_mut().points.clear();
                frame.borrow_mut().redraw();
                true
            },
            _ => false,
        }
    });
 }