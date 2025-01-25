use fltk::{
    enums::{Color, Event, Font, Align},
    frame::Frame,
    prelude::*,
    draw,
    image::RgbImage,
};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;
use crate::scientific::{
    layers::{Annotation, AnnotationType},
    calibration::SpatialCalibration,
};

#[derive(Clone)]
pub struct MeasurementTool {
    pub calibration: SpatialCalibration,
    pub line_color: (u8, u8, u8),
    pub line_width: i32,
}

impl MeasurementTool {
    pub fn new(calibration: SpatialCalibration, line_color: (u8, u8, u8), line_width: i32) -> Self {
        Self {
            calibration,
            line_color,
            line_width,
        }
    }

    pub fn create_measurement(&self, points: Vec<(i32, i32)>, name: String, _image: RgbImage) -> Annotation {
        let length = self.calculate_distance(&points);
        
        Annotation {
            name,
            image: RgbImage::new(&[], 1, 1, fltk::enums::ColorDepth::Rgb8).unwrap(), // Same fix as scale bar
            annotation_type: AnnotationType::Measurement {
                length,
                unit: self.calibration.unit.clone(),
            },
            visible: true,
            coordinates: points,
        }
    }

    pub fn calculate_distance(&self, points: &[(i32, i32)]) -> f32 {
        if points.len() < 2 {
            return 0.0;
        }

        let mut total_distance = 0.0;
        for i in 0..points.len() - 1 {
            let real_point1 = self.calibration.pixel_to_real(points[i]);
            let real_point2 = self.calibration.pixel_to_real(points[i + 1]);
            
            total_distance += ((real_point2.0 - real_point1.0).powi(2) +
                             (real_point2.1 - real_point1.1).powi(2)).sqrt();
        }
        
        total_distance
    }

    pub fn create_scale_bar(&self, length: f32, position: (i32, i32), image: RgbImage) -> Annotation {
        let pixel_length = (length * self.calibration.pixels_per_unit) as i32;
        
        Annotation {
            name: "Scale Bar".to_string(),
            image,
            annotation_type: AnnotationType::Scale {
                pixels_per_unit: self.calibration.pixels_per_unit,
                unit: self.calibration.unit.clone(),
            },
            visible: true,
            coordinates: vec![
                position,
                (position.0 + pixel_length, position.1),
            ],
        }
    }

    pub fn handle_measurement(&self, state_ref: &mut ImageState, points: &[(i32, i32)]) {
        if points.len() >= 2 {
            if let Some(img) = state_ref.image.clone() {
                let annotation = self.create_measurement(
                    points.to_vec(),
                    format!("Measurement {}", state_ref.scientific_state.get_annotation_count() + 1),
                    img
                );
                state_ref.scientific_state.add_annotation(annotation);
            }
        }
    }
}

pub fn start_interactive_measurement(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>, tool: MeasurementTool) {
    let frame = frame.clone();
    let state = state.clone();
    let tool = Rc::new(tool);
    
    // Setup draw callback
    let draw_frame = frame.clone();
    let draw_state = state.clone();
    let draw_tool = tool.clone();
    
    {
        let mut frame_ref = frame.borrow_mut();
        frame_ref.draw(move |_| {
            if let Ok(state_ref) = draw_state.try_borrow() {
                if let Some(mut img) = state_ref.image.clone() {
                    let frame_ref = draw_frame.borrow();
                    img.draw(0, 0, frame_ref.width(), frame_ref.height());
                }

                // Draw current measurement line
                let points = &state_ref.scientific_state.current_measurement_points;
                if points.len() >= 2 {
                    draw::set_draw_color(Color::from_rgb(
                        draw_tool.line_color.0,
                        draw_tool.line_color.1,
                        draw_tool.line_color.2
                    ));
                    draw::set_line_style(draw::LineStyle::Solid, draw_tool.line_width);
                    
                    // Draw all line segments
                    for i in 0..points.len() - 1 {
                        draw::draw_line(
                            points[i].0, points[i].1,
                            points[i + 1].0, points[i + 1].1,
                        );
                    }

                    // Draw points at each vertex
                    for &(x, y) in points {
                        draw::draw_circle(x as f64, y as f64, 3.0);
                    }

                    // Draw distance if calibration is available
                    let distance = draw_tool.calculate_distance(points);
                    let last_point = points.last().unwrap();
                    draw::set_font(Font::Helvetica, 12);
                    draw::draw_text2(
                        &format!("{:.1} {}", distance, draw_tool.calibration.unit),
                        last_point.0 + 10,
                        last_point.1 + 10,
                        0, 0,
                        Align::Left,
                    );
                }
            }
        });
    }

    // Setup event handling
    let handle_frame = frame.clone();
    let handle_state = state;
    let handle_tool = tool;
    
    frame.borrow_mut().handle(move |_, ev| match ev {
        Event::Push => {
            let coords = fltk::app::event_coords();
            if let Ok(mut state_ref) = handle_state.try_borrow_mut() {
                state_ref.scientific_state.add_point(coords);
                handle_frame.borrow_mut().redraw();
            }
            true
        },
        Event::KeyUp => {
            // Handle Enter key to finish measurement
            if fltk::app::event_key() == fltk::enums::Key::Enter {
                if let Ok(mut state_ref) = handle_state.try_borrow_mut() {
                    let points = state_ref.scientific_state.current_measurement_points.clone();
                    if points.len() >= 2 {
                        if let Some(img) = state_ref.image.clone() {
                            let annotation = handle_tool.create_measurement(
                                points,
                                format!("Measurement {}", state_ref.scientific_state.get_annotation_count() + 1),
                                img
                            );
                            state_ref.scientific_state.add_annotation(annotation);
                            state_ref.scientific_state.clear_points();
                            handle_frame.borrow_mut().redraw();
                        }
                    }
                }
                true
            } else {
                false
            }
        },
        _ => false,
    });
}