//src/scientific/state/scientific_state.rs
use fltk::{image::RgbImage, prelude::*, frame::Frame};
use std::{collections::HashMap, rc::Rc, cell::RefCell};
use chrono::Utc;
use crate::{
scientific::{
    layers::{Channel, Annotation, AnnotationType,Metadata, Calibration},
    analysis::{IntensityProfile, CellStatistics, CellMeasurement},
    calibration::SpatialCalibration,    
    types::{ROIShape, ROITool, MeasurementTool, LegendPosition, CellMeasurementMode},
    tools::interactive::cell_analysis_tool::{CellAnalysisTool, CellAnalysisState},
},
state::ImageState,
};


/// state for the scientific image viewer
pub struct ScientificState {
    pub channels: Vec<Channel>,
    pub annotations: Vec<Annotation>,
    pub channel_groups: HashMap<String, Vec<usize>>,
    pub active_channel: Option<usize>,
    pub calibration: SpatialCalibration,
    pub roi_tool: Option<ROITool>,
    pub measurement_tool: Option<MeasurementTool>,
    pub current_roi_points: Vec<(i32, i32)>,
    pub current_measurement_points: Vec<(i32, i32)>,
    pub show_overlay: bool,
    pub calibrations: Vec<Calibration>,
    pub show_legend: bool,
    pub legend_position: LegendPosition,
    pub cell_analysis_tool: Option<CellAnalysisTool>,
    frame: Option<Rc<RefCell<Frame>>>,
    state: Option<Rc<RefCell<ImageState>>>,
    measurements: Vec<CellMeasurement>,
    pub base_image: Option<RgbImage>,
    pub measurement_mode: CellMeasurementMode,
    pub show_preview_layer: bool,  // Add this field
    pub preview_layer: Option<RgbImage>,
    pub show_base_image: bool, 
    pub show_drawing_layer: bool,

}
// implementation block for ScientificState
impl ScientificState {
    pub fn new() -> Self {
        println!("Initializing ScientificState with show_legend=true");  // Debug print
        Self {
            channels: Vec::new(),
            annotations: Vec::new(),
            channel_groups: HashMap::new(),
            active_channel: None,
            calibration: SpatialCalibration::new("µm".to_string()),
            roi_tool: None,
            measurement_tool: None,
            current_roi_points: Vec::new(),
            current_measurement_points: Vec::new(),
            show_overlay: true,
            calibrations: Vec::new(),
            show_legend: false,  // adding a default value that will be removed later
            legend_position: LegendPosition::BottomRight,
            cell_analysis_tool: None,
            frame: None,
            state: None,
            measurements: Vec::new(),
            base_image: None,
            measurement_mode: CellMeasurementMode::Single,
            show_preview_layer: true,
            preview_layer: None,
            show_base_image: true,
            show_drawing_layer: true,

        }
    }
    // Add toggle method
    pub fn toggle_drawing_layer(&mut self) {
        self.show_drawing_layer = !self.show_drawing_layer;
        println!("Drawing layer display toggled: {}", self.show_drawing_layer);
    }
    pub fn toggle_preview_layer(&mut self) {
        self.show_preview_layer = !self.show_preview_layer;
        println!("Preview layer toggled: {}", self.show_preview_layer);
    }
    // Add toggle method
    pub fn toggle_base_image(&mut self) {
        self.show_base_image = !self.show_base_image;
        println!("Base image display toggled: {}", self.show_base_image);
    }
    // Add this method to get measurements
    pub fn get_measurements(&self) -> Option<Vec<CellMeasurement>> {
        if self.measurements.is_empty() {
            None
        } else {
            Some(self.measurements.clone())
        }
    }
    pub fn get_measurement_mode(&self) -> CellMeasurementMode {
        self.measurement_mode
    }
    pub fn store_base_image(&mut self, image: RgbImage) {
        println!("Storing base image in scientific state");
        
        // Get the image dimensions
        let width = image.data_w();
        let height = image.data_h();
        
        // Create a base image with exact same dimensions
        let base_image = RgbImage::new(
            &image.to_rgb_data(),
            width,
            height,
            fltk::enums::ColorDepth::Rgb8
        ).unwrap();
        
        self.base_image = Some(base_image);
        
        // Create channel with same dimensions
        println!("Creating default channel for base image");
        let channel = Channel::new(
            "Base".to_string(),
            image,
            550.0,
            (255, 255, 255)
        );
        
        self.channels.clear();
        self.channels.push(channel);
        println!("Channel created and stored. Total channels: {}", self.channels.len());
    }
    pub fn get_base_image(&self) -> Option<RgbImage> {
        self.base_image.clone()
    }

    // Add this method to store measurements
    pub fn add_cell_measurement(&mut self, measurement: CellMeasurement) {
        self.measurements.push(measurement);
    }
    // these are the methods for cell analysis integration
    pub fn set_frame(&mut self, frame: Rc<RefCell<Frame>>) {
        self.frame = Some(frame);
    }

    pub fn set_state(&mut self, state: Rc<RefCell<ImageState>>) {
        self.state = Some(state);
    }

    pub fn get_frame(&self) -> Option<Rc<RefCell<Frame>>> {
        self.frame.clone()
    }

    pub fn get_state(&self) -> Option<Rc<RefCell<ImageState>>> {
        self.state.clone()
    }
    // Add ROI update method
    pub fn update_current_roi(&mut self, points: Vec<(i32, i32)>) {
        if let Some(roi) = &mut self.roi_tool {
            roi.shape = ROIShape::Polygon { points };
        }
    }

    // Keep your existing clear_points method and add this for ROI clearing
    pub fn clear_current_roi(&mut self) {
        self.roi_tool = None;
        self.clear_points();
    }

    // Add ROI intensity profile method
    pub fn get_roi_intensity_profile(&mut self, points: &[(i32, i32)]) -> Option<IntensityProfile> {
        println!("Attempting to get ROI intensity profile");
        println!("Channels available: {}", self.channels.len());
        
        if self.channels.is_empty() {
            println!("No channels available for intensity profile");
            return None;
        }
    
        println!("Creating intensity profile with {} points", points.len());
        Some(IntensityProfile::new(points, &self.channels))
    }

    pub fn toggle_legend(&mut self) {
        self.show_legend = !self.show_legend;
        
        // update visibility of all scale annotations
        for annotation in &mut self.annotations {
            if let AnnotationType::Scale { .. } = annotation.annotation_type {
                annotation.visible = self.show_legend;
            }
        }
    }
    pub fn set_legend_position(&mut self, position: LegendPosition) {
        self.legend_position = position;
        // update coordinates for all scale annotations
        if let Some(channel) = self.channels.first() {
            let width = channel.image.data_w();
            let height = channel.image.data_h();
            
            let (x_offset, y_offset) = match self.legend_position {
                LegendPosition::TopLeft => (20, 20),
                LegendPosition::TopRight => (width - 120, 20),
                LegendPosition::BottomLeft => (20, height - 50),
                LegendPosition::BottomRight => (width - 120, height - 50),
            };

            // update coordinates for all scale annotations
            for annotation in &mut self.annotations {
                if let AnnotationType::Scale { .. } = annotation.annotation_type {
                    if annotation.coordinates.len() >= 2 {
                        let scale_length = annotation.coordinates[1].0 - annotation.coordinates[0].0;
                        annotation.coordinates = vec![
                            (x_offset, y_offset),
                            (x_offset + scale_length, y_offset)
                        ];
                    }
                }
            }
        }
    }

    pub fn get_active_roi_type(&self) -> ROIShape {
        if let Some(roi_tool) = &self.roi_tool {
            roi_tool.shape.clone()
        } else {
            ROIShape::Rectangle {
                width: 100,
                height: 100,
            }
        }
    }

    pub fn add_point(&mut self, point: (i32, i32)) {
        if self.roi_tool.is_some() {
            self.current_roi_points.push(point);
            println!("Added ROI point: {:?}", point);
        } else if self.measurement_tool.is_some() {
            self.current_measurement_points.push(point);
            println!("Added measurement point: {:?}", point);
        }
    }

    pub fn clear_points(&mut self) {
        self.current_roi_points.clear();
        self.current_measurement_points.clear();
        println!("Cleared points");
    }

    pub fn set_show_overlay(&mut self, show: bool) {
        self.show_overlay = show;
    }

    pub fn add_channel(&mut self, channel: Channel) -> usize {
        let id = self.channels.len();
        self.group_channel(id, &channel);
        self.channels.push(channel);
        self.active_channel = Some(id);
        id
    }

    fn group_channel(&mut self, id: usize, channel: &Channel) {
        let group_name = match channel.wavelength {
            w if w < 400.0 => "UV",
            w if w < 500.0 => "Blue",
            w if w < 600.0 => "Green",
            _ => "Red",
        };
        self.channel_groups.entry(group_name.to_string())
            .or_insert_with(Vec::new)
            .push(id);
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        println!("Adding annotation: {}", annotation.name);
        println!("Annotation has {} coordinates", annotation.coordinates.len());
        self.annotations.push(annotation);
    }

    pub fn get_annotation_count(&self) -> usize {
        self.annotations.len()
    }

    pub fn get_composite_image(&self) -> Option<RgbImage> {
        // Get dimensions from base image
        let (width, height) = if let Some(base_img) = &self.base_image {
            (base_img.data_w(), base_img.data_h())
        } else {
            return None;
        };

        // Start with a new empty image of the same dimensions
        let mut composite = vec![0u8; (width * height * 3) as usize];

        // Copy base image data if it exists
        if let Some(base_img) = &self.base_image {
            composite.copy_from_slice(&base_img.to_rgb_data());
        }

        // Only draw overlays if show_overlay is true
        if self.show_overlay {
            // Draw all persistent annotations
            for annotation in self.annotations.iter().filter(|a| a.visible) {
                self.overlay_annotation(&mut composite, annotation);
            }
        }

        // Store the current composite as preview layer
        if let Some(preview) = &self.preview_layer {
            if self.show_preview_layer {
                let preview_data = preview.to_rgb_data();
                composite.copy_from_slice(&preview_data);
            }
        }

        // Create new RgbImage with same dimensions as base
        Some(RgbImage::new(
            &composite,
            width,
            height,
            fltk::enums::ColorDepth::Rgb8
        ).unwrap())
    }



    pub fn blend_channel(&self, composite: &mut Vec<u8>, channel: &Channel) {
        let data = channel.image.to_rgb_data();
        let intensity_factor = match channel.wavelength {
            w if w < 400.0 => 0.8,
            w if w < 500.0 => 1.0,
            w if w < 600.0 => 0.9,
            _ => 0.85
        };
        
        for (i, pixel) in data.chunks(3).enumerate() {
            let idx = i * 3;
            if idx + 2 < composite.len() {
                let intensity = (pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / (3.0 * 255.0);
                for c in 0..3 {
                    let color_val = match c {
                        0 => channel.pseudo_color.0,
                        1 => channel.pseudo_color.1,
                        _ => channel.pseudo_color.2,
                    };
                    composite[idx + c] = ((1.0 - channel.opacity) * composite[idx + c] as f32 +
                        channel.opacity * intensity * intensity_factor * color_val as f32) as u8;
                }
            }
        }
    }

    pub fn set_scale(&mut self, pixel_distance: f64, real_distance: f64, unit: String, objective: Option<String>) {
        println!("Setting scale: {} pixels = {} {}", pixel_distance, real_distance, unit);
        let pixels_per_unit = pixel_distance / real_distance;
        self.calibration = SpatialCalibration::new(unit.clone());
        self.calibration.add_point((0, 0), (0.0, 0.0));
        self.calibration.add_point((pixels_per_unit as i32, 0), (1.0, 0.0));
    
        // Store calibration information
        self.calibration.pixels_per_unit = pixels_per_unit as f32;
        self.calibration.unit = unit.clone();
    
        if let Some(obj_name) = objective {
            if let Some(channel) = self.channels.first_mut() {
                channel.metadata.objective = Some(obj_name);
                channel.metadata.scale_calibration = Some((pixels_per_unit as f32, unit.clone()));
            }
        }
    
        // Remove existing scale bar annotations first
        self.annotations.retain(|anno| !matches!(anno.annotation_type, AnnotationType::Scale { .. }));
    
        // Only create new scale annotation if we have an image
        if let Some(channel) = self.channels.first() {
            let width = channel.image.data_w();
            let height = channel.image.data_h();
            
            let (x_offset, y_offset) = match self.legend_position {
                LegendPosition::TopLeft => (20, 20),
                LegendPosition::TopRight => (width - 120, 20),
                LegendPosition::BottomLeft => (20, height - 50),
                LegendPosition::BottomRight => (width - 120, height - 50),
            };
    
            let scale_coordinates = vec![
                (x_offset, y_offset),
                (x_offset + (100.0 * pixels_per_unit) as i32, y_offset)
            ];
    
            let annotation = Annotation {
                name: format!("Scale Bar ({} {})", 100, unit),
                image: RgbImage::new(
                    &vec![0u8; (width * height * 3) as usize],
                    width,
                    height,
                    fltk::enums::ColorDepth::Rgb8
                ).unwrap(),
                annotation_type: AnnotationType::Scale {
                    pixels_per_unit: pixels_per_unit as f32,
                    unit,
                },
                visible: self.show_legend,  // Use current visibility state
                coordinates: scale_coordinates,
            };
            
            self.annotations.push(annotation);
        }
    }
    
    pub fn set_roi_tool(&mut self, tool: ROITool) {
        let annotation = Annotation {
            name: format!("ROI {}", self.annotations.len() + 1),
            image: RgbImage::new(&[], 1, 1, fltk::enums::ColorDepth::Rgb8).unwrap(), // Minimal dummy image
            annotation_type: AnnotationType::ROI {
                color: tool.color,
                line_width: tool.line_width,
            },
            visible: true,
            coordinates: self.current_roi_points.clone(),
        };
        println!("Adding ROI annotation with {} points", self.current_roi_points.len());
        self.annotations.push(annotation);
        
        self.roi_tool = Some(tool);
        self.measurement_tool = None;
    }

    pub fn set_measurement_tool(&mut self, tool: MeasurementTool) {
        self.measurement_tool = Some(tool);
        self.roi_tool = None;
        self.clear_points();
    }

    fn overlay_annotation(&self, composite: &mut Vec<u8>, annotation: &Annotation) {
        // Only proceed if the annotation is visible
        if !annotation.visible {
            return;
        }
        
        let (width, height) = if let Some(base_img) = &self.base_image {
            (base_img.data_w(), base_img.data_h())
        } else {
            return;
        };
        
        match &annotation.annotation_type {
            AnnotationType::ROI { color, line_width } => {
                println!("Drawing ROI annotation with {} points", annotation.coordinates.len());
                self.draw_roi(composite, width, height, &annotation.coordinates, *color, *line_width);
            },
            AnnotationType::Scale { pixels_per_unit, unit } => {
                println!("Drawing scale bar annotation");
                self.draw_scale_bar(composite, width, height, &annotation.coordinates, *pixels_per_unit);
            },
            AnnotationType::Measurement { length, unit } => {
                println!("Drawing measurement annotation");
                self.draw_measurement(composite, width, height, &annotation.coordinates, *length, unit);
            },
            _ => {}
        }
    }

    fn draw_in_progress(&self, composite: &mut Vec<u8>, width: i32, height: i32, points: &[(i32, i32)], color: (u8, u8, u8)) {
        if points.len() < 2 {
            return;
        }

        for i in 0..points.len() - 1 {
            self.draw_line(composite, width, height, points[i], points[i + 1], color, 1);
        }

        if self.roi_tool.is_some() && points.len() > 2 {
            self.draw_line(composite, width, height, points[points.len() - 1], points[0], color, 1);
        }
    }

    fn draw_roi(&self, composite: &mut Vec<u8>, width: i32, height: i32, points: &[(i32, i32)], color: (u8, u8, u8), line_width: i32) {
        if points.len() < 2 {
            return;
        }

        for i in 0..points.len() - 1 {
            self.draw_line(composite, width, height, points[i], points[i + 1], color, line_width);
        }
        
        // Close the shape by connecting last point to first point
        if points.len() > 2 {
            self.draw_line(composite, width, height, points[points.len() - 1], points[0], color, line_width);
        }
    }


    fn draw_scale_bar(&self, composite: &mut Vec<u8>, width: i32, height: i32, points: &[(i32, i32)], pixels_per_unit: f32) {
        if points.len() < 2 {
            return;
        }

        let (x1, y1) = points[0];
        let (x2, y2) = points[1];
        
        // Draw main scale bar line
        self.draw_line(
            composite,
            width,
            height,
            (x1, y1),
            (x2, y2),
            (255, 255, 255),
            2
        );
        
        // Draw tick marks
        let tick_height = 5;
        let tick_points = [
            ((x1, y1 - tick_height), (x1, y1 + tick_height)),
            ((x2, y2 - tick_height), (x2, y2 + tick_height))
        ];
        
        for (start, end) in tick_points.iter() {
            self.draw_line(composite, width, height, *start, *end, (255, 255, 255), 1);
        }
        
        // Draw scale text
        self.draw_text_on_pixels(
            composite,
            width,
            height,
            x1,
            y1 - 15,
            &format!("100 {}", self.calibration.unit),
            (255, 255, 255)
        );
    }

    
    
    // Helper function to draw text directly on pixels
    fn draw_text_on_pixels(&self, composite: &mut Vec<u8>, width: i32, height: i32, x: i32, y: i32, text: &str, color: (u8, u8, u8)) {
        // Simple rectangle background for text
        for dy in -1..10 {
            for dx in -1..(text.len() as i32 * 6 + 1) {
                let px = x + dx;
                let py = y + dy;
                if px >= 0 && px < width && py >= 0 && py < height {
                    let idx = (py * width + px) as usize * 3;
                    if idx + 2 < composite.len() {
                        composite[idx] = 0;  // Black background
                        composite[idx + 1] = 0;
                        composite[idx + 2] = 0;
                    }
                }
            }
        }
        
        // Draw text in white (very basic font rendering)
        for (i, _) in text.chars().enumerate() {
            let px = x + (i as i32 * 6);
            let py = y;
            if px >= 0 && px < width && py >= 0 && py < height {
                let idx = (py * width + px) as usize * 3;
                if idx + 2 < composite.len() {
                    composite[idx] = color.0;
                    composite[idx + 1] = color.1;
                    composite[idx + 2] = color.2;
                }
            }
        }
    }

    fn draw_measurement(&self, composite: &mut Vec<u8>, width: i32, height: i32, points: &[(i32, i32)], length: f32, unit: &str) {
        if points.len() < 2 {
            return;
        }

        let (x1, y1) = points[0];
        let (x2, y2) = points[1];

        // Draw the measurement line
        self.draw_line(composite, width, height, (x1, y1), (x2, y2), (255, 255, 0), 2);

        // Calculate and display the real-world distance
        let (distance, _) = self.calculate_real_distance((x1, y1), (x2, y2));
        let text = format!("{:.2} {}", distance, unit);

        // Position text above the line
        let text_x = (x1 + x2) / 2 - 20;
        let text_y = (y1 + y2) / 2 - 15;

        self.draw_text_on_pixels(
            composite,
            width,
            height,
            text_x,
            text_y,
            &text,
            (255, 255, 0)
        );
    }


    fn draw_line(&self, composite: &mut Vec<u8>, width: i32, height: i32, start: (i32, i32), end: (i32, i32), color: (u8, u8, u8), _line_width: i32) {
        let mut x0 = start.0;
        let mut y0 = start.1;
        let x1 = end.0;
        let y1 = end.1;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && x0 < width && y0 >= 0 && y0 < height {
                let idx = (y0 * width + x0) as usize * 3;
                if idx + 2 < composite.len() {
                    composite[idx] = color.0;
                    composite[idx + 1] = color.1;
                    composite[idx + 2] = color.2;
                }
            }

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
    /// Calculate real-world distance between two points using current calibration
    pub fn calculate_real_distance(&self, point1: (i32, i32), point2: (i32, i32)) -> (f64, String) {
        let dx = (point2.0 - point1.0) as f64;
        let dy = (point2.1 - point1.1) as f64;
        let pixel_distance = (dx * dx + dy * dy).sqrt();
        let real_distance = pixel_distance / self.calibration.pixels_per_unit as f64;
        (real_distance, self.calibration.unit.clone())
    }
    /// Add measurement annotation with real-world units
    pub fn add_measurement(&mut self, start: (i32, i32), end: (i32, i32)) {
        let (distance, unit) = self.calculate_real_distance(start, end);
        println!("Adding measurement: {:.2} {}", distance, unit.clone());
        
        // Create annotation without storing image
        let annotation = Annotation {
            name: format!("Measurement: {:.2} {}", distance, unit.clone()),
            image: RgbImage::new(&[], 1, 1, fltk::enums::ColorDepth::Rgb8).unwrap(), // Minimal dummy image
            annotation_type: AnnotationType::Measurement {
                length: distance as f32,
                unit: unit,  // Move occurs here
            },
            visible: true,
            coordinates: vec![start, end],
        };
        
        self.annotations.push(annotation);
    }

    /// Calculate area of an ROI in real-world units
    pub fn calculate_roi_area(&self, points: &[(i32, i32)]) -> (f64, String) {
        if points.len() < 3 {
            return (0.0, self.calibration.unit.clone() + "²");
        }

        // Calculate area in pixels using shoelace formula
        let mut area = 0.0;
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            area += points[i].0 as f64 * points[j].1 as f64;
            area -= points[j].0 as f64 * points[i].1 as f64;
        }
        area = (area / 2.0).abs();

        // Convert to real units (squared)
        let pixels_per_unit = self.calibration.pixels_per_unit as f64;
        let real_area = area / (pixels_per_unit * pixels_per_unit);
        
        (real_area, self.calibration.unit.clone() + "²")
    }
    pub fn toggle_annotation_visibility(&mut self, index: usize) {
        if let Some(annotation) = self.annotations.get_mut(index) {
            annotation.visible = !annotation.visible;
            println!("Toggled visibility for annotation {}: {}", index, annotation.visible);
        }
    }
    
    pub fn get_visible_annotations(&self) -> Vec<&Annotation> {
        self.annotations.iter()
            .filter(|a| a.visible)
            .collect()
    }
    pub fn add_objective_calibration(
        &mut self, 
        objective: String, 
        real_distance: f64, 
        unit: String, 
        pixel_distance: f64,
        image_name: Option<String> 
    ) {
        let pixels_per_unit = pixel_distance / real_distance;
        
        let calibration = Calibration {
            objective: objective.clone(),
            pixels_per_unit: pixels_per_unit as f32,
            unit: unit.clone(),
            pixel_distance,
            real_distance,
            timestamp: Utc::now(),
            image_name, 
            //image_name: self.path.as_ref().map(|p| p.file_name().unwrap().to_string_lossy().into_owned()),
            notes: None,
        };
        
        self.calibrations.push(calibration);
    }


}
// Implement CellAnalysisState trait for ScientificState
impl CellAnalysisState for ScientificState {
    fn init_cell_analysis(&mut self, calibration_scale: f64, unit: String) {
        self.cell_analysis_tool = Some(CellAnalysisTool::new(calibration_scale, unit));
    }

    fn start_cell_analysis(&mut self, mode: CellMeasurementMode) {
        self.measurement_mode = mode;
        if let Some(tool) = &mut self.cell_analysis_tool {
            tool.set_mode(mode);
            tool.activate();
        }
    }

    fn stop_cell_analysis(&mut self) {
        if let Some(tool) = &mut self.cell_analysis_tool {
            tool.deactivate();
        }
    }

    fn is_analyzing_cells(&self) -> bool {
        self.cell_analysis_tool
            .as_ref()
            .map_or(false, |tool| tool.is_active())
    }

    fn get_cell_statistics(&self) -> Option<CellStatistics> {
        self.cell_analysis_tool
            .as_ref()
            .and_then(|tool| tool.get_statistics())
    }
    fn get_measurements(&self) -> Option<Vec<CellMeasurement>> {
        if let Some(tool) = &self.cell_analysis_tool {
            Some(tool.get_measurements().to_vec())
        } else {
            None
        }
    }
}