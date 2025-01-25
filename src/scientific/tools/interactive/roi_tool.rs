use fltk::{
    enums::{Color, Event},
    frame::Frame,
    prelude::*,
    draw,
    image::RgbImage,  // Added this import
};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;
use crate::scientific::layers::{Annotation, AnnotationType};
use crate::scientific::types::{ROIShape, ROITool};

struct InteractiveROIState {
    start_pos: Option<(i32, i32)>,
    current_shape: Option<ROIShape>,
    points: Vec<(i32, i32)>,
}

impl InteractiveROIState {
    fn new() -> Self {
        Self {
            start_pos: None,
            current_shape: None,
            points: Vec::new(),
        }
    }
}

pub fn start_interactive_roi(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    let interactive_state = Rc::new(RefCell::new(InteractiveROIState::new()));
    let frame_draw = frame.clone();

    setup_roi_drawing(frame, interactive_state.clone(), state.clone());
    setup_roi_handling(frame, interactive_state, state.clone(), frame_draw);
}

fn setup_roi_drawing(
    frame: &Rc<RefCell<Frame>>,
    interactive_state: Rc<RefCell<InteractiveROIState>>,
    state: Rc<RefCell<ImageState>>,
) {
    let frame = frame.clone();
    
    frame.borrow_mut().draw(move |f| {
        if let Some(mut img) = state.borrow().image.clone() {
            img.draw(f.x(), f.y(), f.width(), f.height());
        }
        
        if let Ok(interactive_ref) = interactive_state.try_borrow() {
            if let Some(shape) = &interactive_ref.current_shape {
                draw::set_draw_color(Color::Red);
                draw::set_line_style(draw::LineStyle::Solid, 2);
                
                match shape {
                    ROIShape::Rectangle { width, height } => {
                        if let Some(start) = interactive_ref.start_pos {
                            draw::draw_rect(start.0, start.1, *width, *height);
                        }
                    },
                    ROIShape::Ellipse { width, height } => {
                        if let Some(start) = interactive_ref.start_pos {
                            draw::draw_arc(start.0, start.1, *width, *height, 0.0, 360.0);
                        }
                    },
                    ROIShape::Polygon { points } | ROIShape::Line { points } => {
                        draw_polygon_or_line(points);
                    }
                }
            }
        }
    });
}

fn setup_roi_handling(
    frame: &Rc<RefCell<Frame>>,
    interactive_state: Rc<RefCell<InteractiveROIState>>,
    state: Rc<RefCell<ImageState>>,
    frame_draw: Rc<RefCell<Frame>>,
) {
    let interactive_state_clone = interactive_state.clone();
    
    frame.borrow_mut().handle(move |_, ev| {
        match ev {
            Event::Push => handle_push_event(&interactive_state, &frame_draw),
            Event::Drag => handle_drag_event(&interactive_state, &state, &frame_draw),
            Event::Released => handle_release_event(&interactive_state_clone, &state, &frame_draw),
            _ => false,
        }
    });
}

fn handle_push_event(
    interactive_state: &Rc<RefCell<InteractiveROIState>>, 
    frame_draw: &Rc<RefCell<Frame>>
) -> bool {
    let coords = fltk::app::event_coords();
    if let Ok(mut interactive_ref) = interactive_state.try_borrow_mut() {
        interactive_ref.start_pos = Some(coords);
        interactive_ref.points.push(coords);
        frame_draw.borrow_mut().redraw();
    }
    true
}

fn handle_drag_event(
    interactive_state: &Rc<RefCell<InteractiveROIState>>,
    state: &Rc<RefCell<ImageState>>,
    frame_draw: &Rc<RefCell<Frame>>,
) -> bool {
    let coords = fltk::app::event_coords();
    if let Ok(mut interactive_ref) = interactive_state.try_borrow_mut() {
        if let Some(start) = interactive_ref.start_pos {
            let width = coords.0 - start.0;
            let height = coords.1 - start.1;
            
            let shape = if let Ok(state_ref) = state.try_borrow() {
                match state_ref.scientific_state.get_active_roi_type() {
                    ROIShape::Rectangle { .. } => ROIShape::Rectangle { width, height },
                    ROIShape::Ellipse { .. } => ROIShape::Ellipse { width, height },
                    ROIShape::Polygon { .. } => {
                        interactive_ref.points.push(coords);
                        ROIShape::Polygon { points: interactive_ref.points.clone() }
                    },
                    ROIShape::Line { .. } => ROIShape::Line { 
                        points: vec![start, coords] 
                    },
                }
            } else {
                return true;
            };
            
            interactive_ref.current_shape = Some(shape);
            frame_draw.borrow_mut().redraw();
        }
    }
    true
}

fn handle_release_event(
    interactive_state: &Rc<RefCell<InteractiveROIState>>,
    state: &Rc<RefCell<ImageState>>,
    frame_draw: &Rc<RefCell<Frame>>,
) -> bool {
    let mut shape = None;
    let mut points = Vec::new();
    
    if let Ok(mut interactive_ref) = interactive_state.try_borrow_mut() {
        shape = interactive_ref.current_shape.take();
        points = interactive_ref.points.clone();
        interactive_ref.points.clear();
        interactive_ref.start_pos = None;
    }
    
    if let Some(shape) = shape {
        // Create ROI tool and annotation
        let roi_tool = ROITool::new(shape.clone(), (255, 0, 0), 2);
        if let Ok(mut state_ref) = state.try_borrow_mut() {
            create_roi_annotation(&mut state_ref, &points, &roi_tool);
            state_ref.scientific_state.set_roi_tool(roi_tool);
            println!("ROI annotation created and tool set");
        }
    }
    
    frame_draw.borrow_mut().redraw();
    true
}

fn draw_polygon_or_line(points: &[(i32, i32)]) {
    for i in 0..points.len().saturating_sub(1) {
        draw::draw_line(
            points[i].0, points[i].1,
            points[i + 1].0, points[i + 1].1,
        );
    }
    if points.len() > 2 {
        draw::draw_line(
            points[points.len() - 1].0, points[points.len() - 1].1,
            points[0].0, points[0].1,
        );
    }
}

fn create_roi_annotation(state_ref: &mut ImageState, points: &[(i32, i32)], roi_tool: &ROITool) {
    if let Some(img) = state_ref.image.clone() {
        // Create a blank image for the annotation overlay
        let blank_data = vec![0u8; (img.data_w() * img.data_h() * 3) as usize];
        let annotation_img = RgbImage::new(
            &blank_data,
            img.data_w(),
            img.data_h(),
            fltk::enums::ColorDepth::Rgb8
        ).unwrap();

        let annotation = Annotation {
            name: format!("ROI {}", state_ref.scientific_state.get_annotation_count() + 1),
            image: annotation_img,
            annotation_type: AnnotationType::ROI {
                color: roi_tool.color,
                line_width: roi_tool.line_width,
            },
            visible: true,
            coordinates: points.to_vec(),
        };

        println!("Creating ROI annotation with {} points", points.len());
        state_ref.scientific_state.add_annotation(annotation);
    }
}