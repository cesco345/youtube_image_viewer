// scientific/tools/interactive/line_profile_tool.rs

use fltk::{
    enums::Event,
    frame::Frame,
    prelude::*,
    draw,
};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;

pub struct InteractiveProfile {
    points: Vec<(i32, i32)>,
}

impl InteractiveProfile {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }
}

pub fn start_interactive_profile(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    println!("Starting interactive profile");
    let interactive = Rc::new(RefCell::new(InteractiveProfile::new()));
    let frame_draw = frame.clone();
    
    {
        let interactive = interactive.clone();
        let state = state.clone();
        let frame = frame.clone();
        
        frame.borrow_mut().draw(move |f| {
            // Draw base image
            if let Some(mut img) = state.borrow().image.clone() {
                img.draw(f.x(), f.y(), f.width(), f.height());
            }
            
            // Draw profile line
            let points = interactive.borrow().points.clone();
            if !points.is_empty() {
                draw_profile(&points);
            }
        });
    }

    frame.borrow_mut().handle({
        let interactive = interactive.clone();
        let state = state.clone();
        
        move |_, ev| match ev {
            Event::Push => {
                let coords = fltk::app::event_coords();
                println!("Push event at ({}, {})", coords.0, coords.1);
                interactive.borrow_mut().points.push(coords);
                frame_draw.borrow_mut().redraw();
                true
            },
            Event::Drag => {
                let coords = fltk::app::event_coords();
                println!("Drag event at ({}, {})", coords.0, coords.1);
                interactive.borrow_mut().points.push(coords);
                frame_draw.borrow_mut().redraw();
                true
            },
            Event::Released => {
                let points = interactive.borrow().points.clone();
                println!("Release event - {} points collected", points.len());
                if points.len() >= 2 {
                    let mut state_ref = state.borrow_mut();
                    if let Some(profile) = state_ref.scientific_state.get_roi_intensity_profile(&points) {
                        println!("Profile calculated, showing dialog");
                        crate::scientific::ui::show_profile_dialog(&profile);
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

fn draw_profile(points: &[(i32, i32)]) {
    if points.len() < 2 {
        return;
    }
    
    draw::set_draw_color(fltk::enums::Color::Red);
    draw::set_line_style(draw::LineStyle::Solid, 2);
    
    // Draw line segments
    for window in points.windows(2) {
        let (x1, y1) = window[0];
        let (x2, y2) = window[1];
        draw::draw_line(x1, y1, x2, y2);
    }
    
    // Draw points
    for &(x, y) in points {
        draw::draw_circle(x as f64, y as f64, 3.0);
    }
}