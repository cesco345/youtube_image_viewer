use fltk::{
    enums::{Event, Font, Color, Align},
    frame::Frame,
    prelude::*,
    draw,
};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;
use crate::scientific::layers::{Annotation, AnnotationType};
use crate::scientific::ui::show_scale_input_dialog;

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
    let interactive = Rc::new(RefCell::new(InteractiveScale::new()));
    let frame_draw = frame.clone();
    
    {
        let interactive = interactive.clone();
        let state = state.clone();
        
        frame.borrow_mut().draw(move |f| {
            // Draw the base image
            if let Some(mut img) = state.borrow().image.clone() {
                img.draw(f.x(), f.y(), f.width(), f.height());
            }
            
            // Draw the preview line and points
            let points = interactive.borrow().points.clone();
            if !points.is_empty() {
                // Draw guide text
                if points.len() < 2 {
                    draw::set_font(Font::Helvetica, 14);
                    draw::set_draw_color(Color::White);
                    draw::draw_text2(
                        "Click and drag to draw scale line",
                        f.x() + 10,
                        f.y() + 20,
                        0, 0,
                        Align::Left,
                    );
                }
                
                // Draw the line
                draw::set_draw_color(Color::White);
                draw::set_line_style(draw::LineStyle::Solid, 4);
                
                if points.len() >= 2 {
                    let (x1, y1) = points[0];
                    let (x2, y2) = points[1];
                    draw::draw_line(x1, y1, x2, y2);
                    
                    // Draw endpoints
                    draw::draw_circle(x1 as f64, y1 as f64, 3.0);
                    draw::draw_circle(x2 as f64, y2 as f64, 3.0);
                    
                    // Show pixel distance
                    let dx = x2 - x1;
                    let dy = y2 - y1;
                    let distance = ((dx * dx + dy * dy) as f64).sqrt();
                    draw::draw_text2(
                        &format!("{:.1} px", distance),
                        x2 + 10,
                        y2 + 10,
                        0, 0,
                        Align::Left,
                    );
                }
            }
        });
    }

    frame.borrow_mut().handle({
        let interactive = interactive.clone();
        let state = state.clone();
        let frame_draw = frame_draw.clone();
        
        move |_, ev| match ev {
            Event::Push => {
                let coords = fltk::app::event_coords();
                interactive.borrow_mut().points.clear();
                interactive.borrow_mut().points.push(coords);
                frame_draw.borrow_mut().redraw();
                true
            },
            Event::Drag => {
                let coords = fltk::app::event_coords();
                if interactive.borrow().points.len() == 1 {
                    interactive.borrow_mut().points.push(coords);
                } else if interactive.borrow().points.len() == 2 {
                    interactive.borrow_mut().points[1] = coords;
                }
                frame_draw.borrow_mut().redraw();
                true
            },
            Event::Released => {
                let points = interactive.borrow().points.clone();
                if points.len() >= 2 {
                    let (x1, y1) = points[0];
                    let (x2, y2) = points[1];
                    let pixel_distance = ((x2 - x1).pow(2) as f64 + 
                                       (y2 - y1).pow(2) as f64).sqrt();
                    
                    if let Some((real_distance, unit, objective)) = show_scale_input_dialog(pixel_distance, &state) {
                        if let Ok(mut state_ref) = state.try_borrow_mut() {
                            state_ref.scientific_state.set_scale(
                                pixel_distance,
                                real_distance,
                                unit,
                                Some(objective)
                            );
                            frame_draw.borrow_mut().redraw();
                        }
                    }
                }
                interactive.borrow_mut().points.clear();
                frame_draw.borrow_mut().redraw();
                true
            },
            _ => false,
        }
    });
}