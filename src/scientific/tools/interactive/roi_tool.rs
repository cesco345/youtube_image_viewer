use fltk::{
    enums::{Color, Event},
    frame::Frame,
    prelude::*,
    draw,
    image::RgbImage,
};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;
use crate::scientific::layers::{Annotation, AnnotationType};
use crate::scientific::types::{ROIShape, ROITool};
use crate::scientific::tools::interactive::cell_analysis_tool::CellAnalysisState;

struct ScalingInfo {
    scale: f32,
    offset_x: i32,
    offset_y: i32,
    frame_x: i32,
    frame_y: i32,
    img_w: i32,
    img_h: i32,
}

pub struct InteractiveROIState {
    start_pos: Option<(i32, i32)>,
    current_shape: Option<ROIShape>,
    points: Vec<(i32, i32)>,
    base_image: Option<RgbImage>,
    scaling: Option<ScalingInfo>,
}

impl InteractiveROIState {
    pub fn new() -> Self {
        Self {
            start_pos: None,
            current_shape: None,
            points: Vec::new(),
            base_image: None,
            scaling: None,
        }
    }

    fn update_scaling(&mut self, frame: &Frame, img_w: i32, img_h: i32) {
        let frame_w = frame.width() as f32;
        let frame_h = frame.height() as f32;
        let img_w = img_w as f32;
        let img_h = img_h as f32;

        // Calculate aspect ratios
        let frame_aspect = frame_w / frame_h;
        let img_aspect = img_w / img_h;

        // Determine scaling and offsets
        let (scale, offset_x, offset_y) = if frame_aspect > img_aspect {
            // Frame is wider than image
            let scale = frame_h / img_h;
            let offset_x = ((frame_w - (img_w * scale)) / 2.0) as i32;
            let offset_y = 0;
            (scale, offset_x, offset_y)
        } else {
            // Frame is taller than image
            let scale = frame_w / img_w;
            let offset_x = 0;
            let offset_y = ((frame_h - (img_h * scale)) / 2.0) as i32;
            (scale, offset_x, offset_y)
        };

        self.scaling = Some(ScalingInfo {
            scale,
            offset_x,
            offset_y,
            frame_x: frame.x(),
            frame_y: frame.y(),
            img_w: img_w as i32,
            img_h: img_h as i32,
        });
    }

    fn display_to_image_coords(&self, display_x: i32, display_y: i32) -> Option<(i32, i32)> {
        self.scaling.as_ref().map(|scaling| {
            // Adjust for frame position
            let rel_x = display_x - scaling.frame_x - scaling.offset_x;
            let rel_y = display_y - scaling.frame_y - scaling.offset_y;

            // Convert to image coordinates
            let img_x = (rel_x as f32 / scaling.scale) as i32;
            let img_y = (rel_y as f32 / scaling.scale) as i32;

            // Clamp to image boundaries
            let img_x = img_x.clamp(0, scaling.img_w - 1);
            let img_y = img_y.clamp(0, scaling.img_h - 1);

            (img_x, img_y)
        })
    }

    fn image_to_display_coords(&self, image_x: i32, image_y: i32) -> Option<(i32, i32)> {
        self.scaling.as_ref().map(|scaling| {
            // Convert to display coordinates
            let display_x = (image_x as f32 * scaling.scale) as i32 + scaling.offset_x + scaling.frame_x;
            let display_y = (image_y as f32 * scaling.scale) as i32 + scaling.offset_y + scaling.frame_y;
            (display_x, display_y)
        })
    }
}

pub fn start_interactive_roi(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    println!("Starting interactive ROI");
    let interactive_state = Rc::new(RefCell::new(InteractiveROIState::new()));
    
    // Initialize scaling information
    if let Ok(state_ref) = state.try_borrow() {
        if let Some(img) = &state_ref.image {
            println!("Storing base image and calculating scaling");
            let mut interactive_ref = interactive_state.borrow_mut();
            interactive_ref.base_image = Some(img.clone());
            interactive_ref.update_scaling(&frame.borrow(), img.data_w(), img.data_h());
        }
    }

    let frame_draw = frame.clone();
    let state_clone = state.clone();

    // Set up drawing
    {
        let interactive_state = interactive_state.clone();
        let state = state.clone();
        
        frame.borrow_mut().draw(move |f| {
            // Draw base image first
            if let Ok(state_ref) = state.try_borrow() {
                if let Some(base_img) = state_ref.scientific_state.get_base_image() {
                    let mut img_copy = base_img.copy();
                    img_copy.draw(f.x(), f.y(), f.width(), f.height());
                }
            }
            
            // Draw current ROI
            if let Ok(interactive_ref) = interactive_state.try_borrow() {
                if !interactive_ref.points.is_empty() {
                    let draw_color = if state.try_borrow().map(|s| s.scientific_state.is_analyzing_cells()).unwrap_or(false) {
                        Color::from_rgb(0, 255, 0)
                    } else {
                        Color::Red
                    };
                    
                    draw::set_draw_color(draw_color);
                    draw::set_line_style(draw::LineStyle::Solid, 2);
                    
                    // Convert points to display coordinates for drawing
                    let display_points: Vec<(i32, i32)> = interactive_ref.points.iter()
                        .filter_map(|&p| interactive_ref.image_to_display_coords(p.0, p.1))
                        .collect();
                    
                    draw_polygon(&display_points);
                    for &point in &display_points {
                        draw_vertex_marker(point.0, point.1);
                    }
                }
            }
            
            // Draw existing annotations
            if let Ok(state_ref) = state.try_borrow() {
                if let Some(composite_img) = state_ref.scientific_state.get_composite_image() {
                    let mut composite_copy = composite_img.copy();
                    composite_copy.draw(f.x(), f.y(), f.width(), f.height());
                }
            }
        });
    }

    // Event handling
    frame.borrow_mut().handle(move |_, ev| {
        match ev {
            Event::Push => {
                let coords = fltk::app::event_coords();
                println!("ROI Push at {:?}", coords);
                if let Ok(mut interactive_ref) = interactive_state.try_borrow_mut() {
                    if let Some(image_coords) = interactive_ref.display_to_image_coords(coords.0, coords.1) {
                        println!("Converted to image coordinates: {:?}", image_coords);
                        interactive_ref.start_pos = Some(image_coords);
                        interactive_ref.points.clear();
                        interactive_ref.points.push(image_coords);
                    }
                }
                frame_draw.borrow_mut().redraw();
                true
            },
            Event::Drag => {
                let coords = fltk::app::event_coords();
                println!("ROI Drag at {:?}", coords);
                if let Ok(mut interactive_ref) = interactive_state.try_borrow_mut() {
                    if let Some(image_coords) = interactive_ref.display_to_image_coords(coords.0, coords.1) {
                        println!("Converted to image coordinates: {:?}", image_coords);
                        interactive_ref.points.push(image_coords);
                    }
                }
                frame_draw.borrow_mut().redraw();
                true
            },
            Event::Released => {
                let coords = fltk::app::event_coords();
                println!("ROI Release at {:?}", coords);
                
                let points = {
                    let mut points = Vec::new();
                    if let Ok(mut interactive_ref) = interactive_state.try_borrow_mut() {
                        if let Some(image_coords) = interactive_ref.display_to_image_coords(coords.0, coords.1) {
                            interactive_ref.points.push(image_coords);
                            if !interactive_ref.points.is_empty() {
                                let first_point = interactive_ref.points[0];
                                interactive_ref.points.push(first_point);
                            }
                            points = interactive_ref.points.clone();
                        }
                    }
                    points
                };

                // Process ROI
                if points.len() >= 3 {
                    if let Ok(mut state_ref) = state_clone.try_borrow_mut() {
                        let (width, height) = if let Some(ref img) = state_ref.image {
                            (img.data_w(), img.data_h())
                        } else {
                            (1, 1)
                        };

                        if state_ref.scientific_state.is_analyzing_cells() {
                            let profile = state_ref.scientific_state.get_roi_intensity_profile(&points);
                            if let Some(prof) = profile {
                                if let Some(cell_tool) = &mut state_ref.scientific_state.cell_analysis_tool {
                                    cell_tool.process_measurement(prof, &points);
                                    let annotation = cell_tool.create_roi_annotation(&points, width, height);
                                    state_ref.scientific_state.add_annotation(annotation);
                                }
                            }
                        } else {
                            let roi_tool = ROITool::new(
                                ROIShape::Polygon { points: points.clone() },
                                (255, 0, 0),
                                2
                            );
                            
                            if let Some(cell_tool) = &mut state_ref.scientific_state.cell_analysis_tool {
                                let annotation = cell_tool.create_roi_annotation(&points, width, height);
                                state_ref.scientific_state.add_annotation(annotation);
                            }
                            
                            state_ref.scientific_state.set_roi_tool(roi_tool);
                        }
                    }
                }
                
                frame_draw.borrow_mut().redraw();
                true
            },
            _ => false,
        }
    });
}

fn draw_vertex_marker(x: i32, y: i32) {
    draw::draw_circle(x as f64, y as f64, 3.0);
    draw::draw_line(x - 3, y, x + 3, y);
    draw::draw_line(x, y - 3, x, y + 3);
}

fn draw_polygon(points: &[(i32, i32)]) {
    if points.len() < 2 {
        return;
    }
    
    for i in 0..points.len() - 1 {
        draw::draw_line(
            points[i].0, points[i].1,
            points[i + 1].0, points[i + 1].1,
        );
    }
    
    if points.len() > 2 {
        let last_idx = points.len() - 1;
        draw::draw_line(
            points[last_idx].0, points[last_idx].1,
            points[0].0, points[0].1,
        );
    }
}

