// src/scientific/tools/interactive/roi/properties.rs

use crate::scientific::types::{ROIShape, LineStyle, ROIMeasurements, ROITool};
use fltk::frame::Frame;
use std::{rc::Rc, cell::RefCell};

#[derive(Debug, Clone)]
pub struct ROIProperties {
    pub label: String,
    pub fill_color: Option<(u8, u8, u8)>,
    pub outline_color: (u8, u8, u8),
    pub line_width: i32,
    pub line_style: LineStyle,
    pub notes: Option<String>,
    pub is_visible: bool,
    pub is_selected: bool,
    pub is_locked: bool,
}

impl Default for ROIProperties {
    fn default() -> Self {
        Self {
            label: String::new(),
            fill_color: None,
            outline_color: (255, 0, 0), // Default red
            line_width: 2,
            line_style: LineStyle::Solid,
            notes: None,
            is_visible: true,
            is_selected: false,
            is_locked: false,
        }
    }
}

impl ROIProperties {
    pub fn new(color: (u8, u8, u8), line_width: i32) -> Self {
        Self {
            outline_color: color,
            line_width,
            ..Default::default()
        }
    }

    pub fn set_visibility(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    pub fn set_selection(&mut self, selected: bool) {
        self.is_selected = selected;
    }

    pub fn toggle_lock(&mut self) {
        self.is_locked = !self.is_locked;
    }

    pub fn update_style(&mut self, line_style: LineStyle, line_width: i32) {
        if !self.is_locked {
            self.line_style = line_style;
            self.line_width = line_width;
        }
    }

    pub fn update_colors(&mut self, outline: (u8, u8, u8), fill: Option<(u8, u8, u8)>) {
        if !self.is_locked {
            self.outline_color = outline;
            self.fill_color = fill;
        }
    }
}

#[derive(Debug)]
pub struct ROIState {
    pub properties: ROIProperties,
    pub current_shape: Option<ROIShape>,
    pub points: Vec<(i32, i32)>,
    pub is_drawing: bool,
    pub show_measurements: bool,
    frame: Option<Rc<RefCell<Frame>>>,
    active_tool: Option<Rc<RefCell<ROITool>>>,
}

impl ROIState {
    pub fn new() -> Self {
        Self {
            properties: ROIProperties::default(),
            current_shape: None,
            points: Vec::new(),
            is_drawing: false,
            show_measurements: false,
            frame: None,
            active_tool: None,
        }
    }

    pub fn set_frame(&mut self, frame: Rc<RefCell<Frame>>) {
        self.frame = Some(frame);
    }

    pub fn get_frame(&self) -> Option<Rc<RefCell<Frame>>> {
        self.frame.clone()
    }

    pub fn get_active_tool_mut(&self) -> Option<Rc<RefCell<ROITool>>> {
        self.active_tool.clone()
    }

    pub fn set_active_tool(&mut self, tool: ROITool) {
        self.active_tool = Some(Rc::new(RefCell::new(tool)));
    }

    pub fn start_drawing(&mut self, shape_type: ROIShape) {
        if !self.properties.is_locked {
            self.is_drawing = true;
            self.current_shape = Some(shape_type);
            self.points.clear();
        }
    }

    pub fn add_point(&mut self, point: (i32, i32)) {
        if self.is_drawing && !self.properties.is_locked {
            self.points.push(point);
            self.update_shape();
        }
    }

    pub fn finish_drawing(&mut self) -> Option<ROIShape> {
        if self.is_drawing && !self.properties.is_locked {
            self.is_drawing = false;
            self.update_shape();
            self.current_shape.clone()
        } else {
            None
        }
    }

    fn update_shape(&mut self) {
        if let Some(shape) = &mut self.current_shape {
            match shape {
                ROIShape::Polygon { points } => {
                    *points = self.points.clone();
                },
                ROIShape::Line { points } => {
                    *points = self.points.clone();
                },
                ROIShape::Rectangle { width, height } => {
                    if self.points.len() >= 2 {
                        let dx = (self.points[1].0 - self.points[0].0).abs();
                        let dy = (self.points[1].1 - self.points[0].1).abs();
                        *width = dx;
                        *height = dy;
                    }
                },
                ROIShape::Ellipse { width, height } => {
                    if self.points.len() >= 2 {
                        let dx = (self.points[1].0 - self.points[0].0).abs();
                        let dy = (self.points[1].1 - self.points[0].1).abs();
                        *width = dx;
                        *height = dy;
                    }
                },
            }
        }
    }

    pub fn clear(&mut self) {
        if !self.properties.is_locked {
            self.points.clear();
            self.current_shape = None;
            self.is_drawing = false;
        }
    }

    pub fn is_active(&self) -> bool {
        self.current_shape.is_some() || !self.points.is_empty()
    }

    pub fn lock(&mut self) {
        self.properties.is_locked = true;
    }

    pub fn unlock(&mut self) {
        self.properties.is_locked = false;
    }

    pub fn set_properties(&mut self, properties: ROIProperties) {
        if !self.properties.is_locked {
            self.properties = properties;
        }
    }

    pub fn get_measurements(&self) -> Option<ROIMeasurements> {
        match &self.current_shape {
            Some(shape) => {
                // Note: This would require ImageData to calculate
                // For now just return placeholder measurements
                Some(ROIMeasurements {
                    id: 0,
                    shape_type: shape.clone(),
                    area: 0.0,
                    perimeter: 0.0,
                    circularity: 0.0,
                    mean_intensity: 0.0,
                    min_intensity: 0.0,
                    max_intensity: 0.0,
                    integrated_density: 0.0,
                    std_dev: 0.0,
                    aspect_ratio: 1.0,
                    roundness: 1.0,
                    solidity: 1.0,
                    is_calibrated: false,
                    units: "pixels".to_string(),
                    notes: None,
                })
            },
            None => None
        }
    }

    pub fn set_notes(&mut self, notes: Option<String>) {
        if !self.properties.is_locked {
            self.properties.notes = notes;
        }
    }

    pub fn toggle_measurements(&mut self) {
        self.show_measurements = !self.show_measurements;
    }

    pub fn get_bounding_box(&self) -> Option<(i32, i32, i32, i32)> {
        if self.points.is_empty() {
            return None;
        }

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for &(x, y) in &self.points {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }

        Some((min_x, min_y, max_x - min_x, max_y - min_y))
    }
}