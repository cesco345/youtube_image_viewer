use fltk::{
    image::RgbImage,
    enums::{Color, Event},
    frame::Frame,
    draw,
    prelude::*,
    group::{Group, Pack},
    button::{RadioRoundButton, Button},
    window::Window,
};

use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;
use crate::scientific::{
    layers::{Annotation, AnnotationType},
    types::{ROIShape, ROITool, CellMeasurementMode},
    tools::interactive::cell_analysis_tool::CellAnalysisState,
    ui::{
        cell_analysis::dialog::show_cell_analysis_dialog,
        roi::show_batch_analysis_dialog,
        
    },
};
pub use crate::scientific::ui::roi::show_roi_mode_dialog;

// Shape selector struct to manage the floating toolbar
#[derive(Clone)]
struct ShapeSelector {
    window: Window,
    shape_selection: Rc<RefCell<ROIShape>>,
}

impl ShapeSelector {
    fn new(on_shape_change: impl Fn(ROIShape) + 'static) -> Self {
        let mut window = Window::default()
            .with_size(150, 200)
            .with_label("ROI Tools");
            
        window.set_color(Color::from_rgb(120, 120, 120));
        
        let mut group = Group::new(5, 5, 90, 190, "");
        group.set_color(Color::from_rgb(80, 80, 80));

        let mut rect_radio = RadioRoundButton::new(10, 10, 80, 25, "Rectangle");
        let mut ellipse_radio = RadioRoundButton::new(10, 45, 80, 25, "Ellipse");
        let mut polygon_radio = RadioRoundButton::new(10, 80, 80, 25, "Polygon");
        let mut line_radio = RadioRoundButton::new(10, 115, 80, 25, "Line");

        // Style radio buttons
        for radio in [&mut rect_radio, &mut ellipse_radio, &mut polygon_radio, &mut line_radio] {
            radio.set_color(Color::from_rgb(80, 80, 80));
            radio.set_selection_color(Color::from_rgb(0, 255, 0));
            radio.set_label_color(Color::White);
        }

        rect_radio.set_value(true); // Default selection

        let shape_selection = Rc::new(RefCell::new(ROIShape::Rectangle { width: 0, height: 0 }));
        let on_shape_change = Rc::new(on_shape_change);

        macro_rules! setup_radio_callback {
            ($radio:ident, $shape:expr) => {
                let shape_selection = shape_selection.clone();
                let on_shape_change = on_shape_change.clone();
                $radio.set_callback(move |_| {
                    *shape_selection.borrow_mut() = $shape;
                    on_shape_change($shape);
                });
            };
        }

        setup_radio_callback!(rect_radio, ROIShape::Rectangle { width: 0, height: 0 });
        setup_radio_callback!(ellipse_radio, ROIShape::Ellipse { width: 0, height: 0 });
        setup_radio_callback!(polygon_radio, ROIShape::Polygon { points: Vec::new() });
        setup_radio_callback!(line_radio, ROIShape::Line { points: Vec::new() });

        group.end();
        window.end();
        
        // Position the window in the top-right corner
        window.set_pos(
            (fltk::app::screen_size().0 - 110.0) as i32,
            50
        );

        Self {
            window,
            shape_selection,
        }
    }

    fn show(&mut self) {
        self.window.show();
    }

    fn hide(&mut self) {
        self.window.hide();
    }

    fn get_current_shape(&self) -> ROIShape {
        self.shape_selection.borrow().clone()
    }
}

#[derive(Debug, Clone)]
pub struct ROIProperties {
    pub outline_color: (u8, u8, u8),
    pub line_width: i32,
}

impl Default for ROIProperties {
    fn default() -> Self {
        Self {
            outline_color: (255, 0, 0),
            line_width: 2,
        }
    }
}

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
    properties: ROIProperties,
    active_shape_type: ROIShape,
}

impl InteractiveROIState {
    pub fn new() -> Self {
        Self {
            start_pos: None,
            current_shape: None,
            points: Vec::new(),
            base_image: None,
            scaling: None,
            properties: ROIProperties::default(),
            active_shape_type: ROIShape::Rectangle { width: 0, height: 0 },
        }
    }

    fn calculate_rectangle(&self, start: (i32, i32), current: (i32, i32)) -> (i32, i32, i32, i32) {
        let width = (current.0 - start.0).abs();
        let height = (current.1 - start.1).abs();
        let x = if current.0 < start.0 { current.0 } else { start.0 };
        let y = if current.1 < start.1 { current.1 } else { start.1 };
        (x, y, width, height)
    }
    pub fn show_roi_mode_dialog(on_select: impl Fn(ROIShape) + 'static) -> ShapeSelector {
        ShapeSelector::new(on_select)
    }

    pub fn set_shape_type(&mut self, shape_type: ROIShape) {
        self.active_shape_type = shape_type;
        self.clear();
    }

    pub fn clear(&mut self) {
        self.start_pos = None;
        self.current_shape = None;
        self.points.clear();
    }

    fn update_scaling(&mut self, frame: &Frame, img_w: i32, img_h: i32) {
        let frame_w = frame.width() as f32;
        let frame_h = frame.height() as f32;
        let img_w = img_w as f32;
        let img_h = img_h as f32;

        let frame_aspect = frame_w / frame_h;
        let img_aspect = img_w / img_h;

        let (scale, offset_x, offset_y) = if frame_aspect > img_aspect {
            let scale = frame_h / img_h;
            let offset_x = ((frame_w - (img_w * scale)) / 2.0) as i32;
            (scale, offset_x, 0)
        } else {
            let scale = frame_w / img_w;
            let offset_y = ((frame_h - (img_h * scale)) / 2.0) as i32;
            (scale, 0, offset_y)
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
            let rel_x = (display_x - scaling.frame_x - scaling.offset_x) as f32;
            let rel_y = (display_y - scaling.frame_y - scaling.offset_y) as f32;

            let img_x = (rel_x / scaling.scale) as i32;
            let img_y = (rel_y / scaling.scale) as i32;

            (
                img_x.clamp(0, scaling.img_w - 1),
                img_y.clamp(0, scaling.img_h - 1)
            )
        })
    }

    fn image_to_display_coords(&self, image_x: i32, image_y: i32) -> Option<(i32, i32)> {
        self.scaling.as_ref().map(|scaling| {
            let display_x = (image_x as f32 * scaling.scale) as i32 + scaling.offset_x + scaling.frame_x;
            let display_y = (image_y as f32 * scaling.scale) as i32 + scaling.offset_y + scaling.frame_y;
            (display_x, display_y)
        })
    }
}

pub fn start_interactive_roi(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    println!("Starting interactive ROI");
    let interactive_state = Rc::new(RefCell::new(InteractiveROIState::new()));
    
    // Initialize scaling
    {
        if let Ok(mut state_ref) = state.try_borrow_mut() {
            if let Some(img) = &state_ref.image {
                println!("Calculating scaling with original image dimensions");
                let mut interactive_ref = interactive_state.borrow_mut();
                interactive_ref.update_scaling(&frame.borrow(), img.data_w(), img.data_h());
            }
        }
    }

    // Create the shape selector first but don't show it yet
    let shape_selector = {
        let interactive_state_select = interactive_state.clone();
        Rc::new(RefCell::new(ShapeSelector::new(move |shape| {
            if let Ok(mut state) = interactive_state_select.try_borrow_mut() {
                state.set_shape_type(shape);
                state.clear();
            }
        })))
    };
    // Show initial mode dialog
    let interactive_state_select = interactive_state.clone();
    let shape_selector_clone = shape_selector.clone();
    show_roi_mode_dialog(move |shape| {
        if let Ok(mut state) = interactive_state_select.try_borrow_mut() {
            state.set_shape_type(shape);
            state.clear();
        }
        // After selecting a shape in the dialog, show the persistent selector
        shape_selector_clone.borrow_mut().show();
    });
    let frame_draw = frame.clone();
    let state_draw = state.clone();
    let interactive_state_draw = interactive_state.clone();

    frame.borrow_mut().draw(move |f| {
        if let Ok(state_ref) = state_draw.try_borrow() {
            let (img_w, img_h) = if let Some(ref img) = state_ref.image {
                (img.data_w() as f32, img.data_h() as f32)
            } else {
                return;
            };

            let frame_w = f.width() as f32;
            let frame_h = f.height() as f32;
            
            let frame_aspect = frame_w / frame_h;
            let img_aspect = img_w / img_h;
            
            let (scale, offset_x, offset_y) = if frame_aspect > img_aspect {
                let scale = frame_h / img_h;
                let offset_x = ((frame_w - (img_w * scale)) / 2.0) as i32;
                (scale, offset_x, 0)
            } else {
                let scale = frame_w / img_w;
                let offset_y = ((frame_h - (img_h * scale)) / 2.0) as i32;
                (scale, 0, offset_y)
            };
    
            if let Some(img) = &state_ref.image {
                let mut img_copy = img.copy();
                img_copy.draw(
                    f.x() + offset_x,
                    f.y() + offset_y,
                    (img_w * scale) as i32,
                    (img_h * scale) as i32
                );
            }

            if state_ref.scientific_state.show_drawing_layer {
                if let Some(composite_img) = state_ref.scientific_state.get_composite_image() {
                    let mut composite_copy = composite_img.copy();
                    composite_copy.draw(
                        f.x() + offset_x,
                        f.y() + offset_y,
                        (img_w * scale) as i32,
                        (img_h * scale) as i32
                    );
                }
            }
        }
        
        if let Ok(interactive_ref) = interactive_state_draw.try_borrow() {
            if !interactive_ref.points.is_empty() {
                let draw_color = if state_draw.try_borrow().map(|s| s.scientific_state.is_analyzing_cells()).unwrap_or(false) {
                    Color::from_rgb(0, 255, 0)
                } else {
                    Color::Red
                };
                
                draw::set_draw_color(draw_color);
                draw::set_line_style(draw::LineStyle::Solid, 2);
                
                let display_points: Vec<(i32, i32)> = interactive_ref.points.iter()
                    .filter_map(|&p| interactive_ref.image_to_display_coords(p.0, p.1))
                    .collect();
                
                if !display_points.is_empty() {
                    draw_polygon(&display_points, &interactive_ref.active_shape_type);
                    for &point in &display_points {
                        draw_vertex_marker(point.0, point.1);
                    }
                }
            }
        }
    });

    let state_handle = state.clone();
    let interactive_state_handle = interactive_state.clone();
    let frame_handle = frame_draw;
    let shape_selector_handle = shape_selector.clone();

    let handler = move |_: &mut Frame, ev: Event| -> bool {
        match ev {
            Event::Show => {
                shape_selector_handle.borrow_mut().show();
                true
            },
            Event::Hide => {
                shape_selector_handle.borrow_mut().hide();
                true
            },

Event::Push => {
    let coords = fltk::app::event_coords();
    if let Ok(mut interactive_ref) = interactive_state_handle.try_borrow_mut() {
        // Update shape type from selector
        if let Ok(selector) = shape_selector_handle.try_borrow() {
            interactive_ref.active_shape_type = selector.get_current_shape();
        }
        if let Some(image_coords) = interactive_ref.display_to_image_coords(coords.0, coords.1) {
            interactive_ref.start_pos = Some(image_coords);
            interactive_ref.points.clear();
            interactive_ref.points.push(image_coords);
        }
    }
    frame_handle.borrow_mut().redraw();
    true
},
Event::Drag => {
    let coords = fltk::app::event_coords();
    if let Ok(mut interactive_ref) = interactive_state_handle.try_borrow_mut() {
        if let Some(image_coords) = interactive_ref.display_to_image_coords(coords.0, coords.1) {
            match interactive_ref.active_shape_type {
                ROIShape::Rectangle { .. } => {
                    if let Some(start) = interactive_ref.start_pos {
                        let width = (image_coords.0 - start.0).abs();
                        let height = (image_coords.1 - start.1).abs();
                        let x = image_coords.0.min(start.0);
                        let y = image_coords.1.min(start.1);
                        
                        interactive_ref.points = vec![
                            (x, y),
                            (x + width, y),
                            (x + width, y + height),
                            (x, y + height),
                            (x, y),
                        ];
                        interactive_ref.current_shape = Some(ROIShape::Rectangle { width, height });
                    }
                },
                ROIShape::Ellipse { .. } => {
                    if let Some(start) = interactive_ref.start_pos {
                        let width = (image_coords.0 - start.0).abs();
                        let height = (image_coords.1 - start.1).abs();
                        let x = start.0.min(image_coords.0);
                        let y = start.1.min(image_coords.1);
                        
                        interactive_ref.points = vec![
                            (x, y),
                            (x + width, y + height)
                        ];
                        interactive_ref.current_shape = Some(ROIShape::Ellipse { width, height });
                    }
                },
                ROIShape::Line { .. } => {
                    if let Some(start) = interactive_ref.start_pos {
                        interactive_ref.points = vec![start, image_coords];
                        interactive_ref.current_shape = Some(ROIShape::Line { 
                            points: vec![start, image_coords] 
                        });
                    }
                },
                ROIShape::Polygon { .. } => {
                    if interactive_ref.points.last() != Some(&image_coords) {
                        interactive_ref.points.push(image_coords);
                    }
                },
                _ => {}
            }
            frame_handle.borrow_mut().redraw();
        }
    }
    true
},
Event::Released => {
    let coords = fltk::app::event_coords();
    
    let points = {
        let mut points = Vec::new();
        if let Ok(mut interactive_ref) = interactive_state_handle.try_borrow_mut() {
            if let Some(image_coords) = interactive_ref.display_to_image_coords(coords.0, coords.1) {
                match interactive_ref.active_shape_type {
                    ROIShape::Rectangle { .. } => {
                        if let Some(start) = interactive_ref.start_pos {
                            let width = (image_coords.0 - start.0).abs();
                            let height = (image_coords.1 - start.1).abs();
                            let x = image_coords.0.min(start.0);
                            let y = image_coords.1.min(start.1);
                            
                            points = vec![
                                (x, y),
                                (x + width, y),
                                (x + width, y + height),
                                (x, y + height),
                                (x, y),
                            ];
                            interactive_ref.current_shape = Some(ROIShape::Rectangle { width, height });
                        }
                    },
                    ROIShape::Ellipse { .. } => {
                        if let Some(start) = interactive_ref.start_pos {
                            let width = (image_coords.0 - start.0).abs();
                            let height = (image_coords.1 - start.1).abs();
                            let x = start.0.min(image_coords.0);
                            let y = start.1.min(image_coords.1);
                            
                            let center_x = x + width/2;
                            let center_y = y + height/2;
                            
                            let num_points = 36;
                            points = (0..=num_points).map(|i| {
                                let angle = (i as f64 * 2.0 * std::f64::consts::PI / num_points as f64);
                                let point_x = center_x + ((width as f64 * angle.cos()) / 2.0) as i32;
                                let point_y = center_y + ((height as f64 * angle.sin()) / 2.0) as i32;
                                (point_x, point_y)
                            }).collect();
                    
                            interactive_ref.current_shape = Some(ROIShape::Ellipse { width, height });
                            interactive_ref.points = points.clone();
                        }
                    },
                    ROIShape::Line { .. } => {
                        if let Some(start) = interactive_ref.start_pos {
                            points = vec![start, image_coords];
                            interactive_ref.current_shape = Some(ROIShape::Line { 
                                points: points.clone()
                            });
                        }
                    },
                    ROIShape::Polygon { .. } => {
                        interactive_ref.points.push(image_coords);
                        points = interactive_ref.points.clone();
                        interactive_ref.current_shape = Some(ROIShape::Polygon { 
                            points: points.clone() 
                        });
                    },
                }
            }
        }
        points
    };

    let should_store = match &points[..] {
        _ if points.is_empty() => false,
        [start, end] if start == end => false,
        _ if points.len() >= 2 => true,
        _ => false,
    };

    if should_store {
        if let Ok(mut state_ref) = state_handle.try_borrow_mut() {
            let (width, height) = if let Some(img) = &state_ref.image {
                (img.data_w(), img.data_h())
            } else {
                (1, 1)
            };

            if let Ok(interactive_ref) = interactive_state_handle.try_borrow() {
                if let Some(current_shape) = &interactive_ref.current_shape {
                    let roi_tool = ROITool::new(
                        current_shape.clone(),
                        (255, 0, 0),
                        2
                    );

                    if let Some(cell_tool) = &mut state_ref.scientific_state.cell_analysis_tool {
                        let annotation = cell_tool.create_roi_annotation(&points, width, height);
                        state_ref.scientific_state.add_annotation(annotation);
                    }

                    state_ref.scientific_state.set_roi_tool(roi_tool);

                    if let Some(profile) = state_ref.scientific_state.get_roi_intensity_profile(&points) {
                        crate::scientific::ui::show_profile_dialog(&profile);
                    }

                    if state_ref.scientific_state.is_analyzing_cells() {
                        if let Some(profile) = state_ref.scientific_state.get_roi_intensity_profile(&points) {
                            if let Some(cell_tool) = &mut state_ref.scientific_state.cell_analysis_tool {
                                cell_tool.process_measurement(profile, &points);
                                
                                if let Some(measurements) = state_ref.scientific_state.get_measurements() {
                                    if let Some(latest_measurement) = measurements.last() {
                                        show_cell_analysis_dialog(
                                            &frame_handle,
                                            &state_handle,
                                            latest_measurement
                                        );
                                    }
                                }

                                if state_ref.scientific_state.get_measurement_mode() == CellMeasurementMode::Batch {
                                    if let Some(all_measurements) = state_ref.scientific_state.get_measurements() {
                                        if all_measurements.len() > 1 {
                                            show_batch_analysis_dialog(
                                                &frame_handle,
                                                &state_handle,
                                                &all_measurements
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    frame_handle.borrow_mut().redraw();
    true
},
_ => false,
}
};

frame.borrow_mut().handle(handler);
}

fn draw_vertex_marker(x: i32, y: i32) {
draw::draw_circle(x as f64, y as f64, 3.0);
draw::draw_line(x - 3, y, x + 3, y);
draw::draw_line(x, y - 3, x, y + 3);
}

fn draw_polygon(points: &[(i32, i32)], active_shape: &ROIShape) {
if points.len() < 2 {
return;
}

match active_shape {
ROIShape::Ellipse { .. } => {
if points.len() >= 2 {
    let (x1, y1) = points[0];
    let (x2, y2) = points[1];
    
    let width = (x2 - x1).abs();
    let height = (y2 - y1).abs();
    let left = x1.min(x2);
    let top = y1.min(y2);
    
    draw::draw_arc(left, top, width, height, 0.0, 360.0);
}
},
_ => {
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
}
}