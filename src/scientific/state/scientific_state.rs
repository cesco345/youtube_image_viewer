use fltk::{image::RgbImage, prelude::*};
use std::collections::HashMap;
use crate::scientific::{
    layers::{Channel, Annotation, AnnotationType,Metadata, Calibration},
    analysis::{IntensityProfile},
    calibration::SpatialCalibration,    
    types::{ROIShape, ROITool, MeasurementTool},
};

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
}

impl ScientificState {
    pub fn new() -> Self {
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
        if self.channels.is_empty() {
            return None;
        }
    
        let first = &self.channels[0].image;
        let mut composite = first.to_rgb_data();
        let (width, height) = (first.data_w(), first.data_h());
    
        // Blend visible channels
        for channel in self.channels.iter().filter(|c| c.visible) {
            self.blend_channel(&mut composite, channel);
        }
    
        // Create single overlay for all annotations
        if self.show_overlay {
            let mut overlay = composite.clone();
            
            // Draw all persistent annotations
            for annotation in self.annotations.iter().filter(|a| a.visible) {
                self.overlay_annotation(&mut overlay, annotation);
            }
    
            // Draw any in-progress ROI
            if !self.current_roi_points.is_empty() && self.roi_tool.is_some() {
                self.draw_in_progress(&mut overlay, width, height, &self.current_roi_points, (255, 0, 0));
            }
    
            composite = overlay;
        }
    
        Some(RgbImage::new(&composite, width, height, fltk::enums::ColorDepth::Rgb8).unwrap())
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
    
        // Store calibration information for future measurements
        self.calibration.pixels_per_unit = pixels_per_unit as f32;
        self.calibration.unit = unit.clone();
    
        if let Some(obj_name) = objective {
            if let Some(channel) = self.channels.first_mut() {
                channel.metadata.objective = Some(obj_name);
                channel.metadata.scale_calibration = Some((pixels_per_unit as f32, unit.clone()));
            }
        }
        // Create a valid image for the annotation
        if let Some(channel) = self.channels.first() {
            let mut data = vec![0u8; (channel.image.data_w() * channel.image.data_h() * 3) as usize];
            let scale_image = RgbImage::new(
                &data,
                channel.image.data_w(),
                channel.image.data_h(),
                fltk::enums::ColorDepth::Rgb8
            ).unwrap();
    
            let annotation = Annotation {
                name: format!("Scale Bar ({} {})", real_distance, unit),
                image: scale_image,
                annotation_type: AnnotationType::Scale {
                    pixels_per_unit: pixels_per_unit as f32,
                    unit: unit,
                },
                visible: true,
                coordinates: self.current_roi_points.clone(),
            };
            println!("Scale bar annotation added with {} points", self.current_roi_points.len());
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

    pub fn get_intensity_profile(&mut self, line_points: &[(i32, i32)]) -> Option<IntensityProfile> {
        if self.channels.is_empty() {
            return None;
        }
    
        if let Some(img) = self.get_composite_image() {
            // Store all line points for later redrawing
            let points = line_points.to_vec();
            println!("Creating line profile annotation with {} points", points.len());
            
            let annotation = Annotation {
                name: format!("Line Profile {}", self.annotations.len() + 1),
                image: img.clone(),
                annotation_type: AnnotationType::Measurement {
                    length: points.len() as f32,
                    unit: "px".to_string(),
                },
                visible: true,
                coordinates: points,  // Store all points for redrawing
            };
            
            self.annotations.push(annotation);
            println!("Line profile annotation added");
        }
    
        Some(IntensityProfile::new(line_points, &self.channels))
    }

    pub fn set_measurement_tool(&mut self, tool: MeasurementTool) {
        self.measurement_tool = Some(tool);
        self.roi_tool = None;
        self.clear_points();
    }

    fn overlay_annotation(&self, composite: &mut Vec<u8>, annotation: &Annotation) {
        let (width, height) = (self.channels[0].image.data_w(), self.channels[0].image.data_h());
        
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
            println!("Not enough points for scale bar");
            return;
        }
    
        let (x1, y1) = points[0];
        let (x2, y2) = points[1];
        
        println!("Drawing scale line from ({}, {}) to ({}, {})", x1, y1, x2, y2);
        
        // Draw main line
        self.draw_line(composite, width, height, (x1, y1), (x2, y2), (255, 255, 255), 2);
        
        // Draw tick marks at ends
        let tick_length: f32 = 5.0;
        let dx = x2 - x1;
        let dy = y2 - y1;
        let distance = ((dx * dx + dy * dy) as f32).sqrt();
        
        // Calculate perpendicular vector for ticks
        let norm_x = -dy as f32 / distance;
        let norm_y = dx as f32 / distance;
        
        // Draw ticks at both ends
        let tick1_end = (
            (x1 as f32 + norm_x * tick_length) as i32,
            (y1 as f32 + norm_y * tick_length) as i32
        );
        let tick2_end = (
            (x2 as f32 + norm_x * tick_length) as i32,
            (y2 as f32 + norm_y * tick_length) as i32
        );
        
        self.draw_line(composite, width, height, (x1, y1), tick1_end, (255, 255, 255), 2);
        self.draw_line(composite, width, height, (x2, y2), tick2_end, (255, 255, 255), 2);
        
        // Calculate and draw the measurement text
        let real_distance = distance / pixels_per_unit;
        let text = format!("{:.1} µm", real_distance);
        
        // Position text above the line
        self.draw_text_on_pixels(
            composite,
            width,
            height,
            (x1 + x2) / 2 - 20,
            (y1 + y2) / 2 - 15,
            &text,
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
    pub fn add_objective_calibration(&mut self, objective: String, real_distance: f64, unit: String, pixel_distance: f64) {
        let pixels_per_unit = pixel_distance / real_distance;
        
        // Store calibration in metadata or a new calibration map
        // This could be added to your Metadata struct or as a new field in ScientificState
        let calibration = Calibration {
            objective,
            pixels_per_unit: pixels_per_unit as f32,
            unit,
            pixel_distance,
            real_distance,
        };
        
        self.calibrations.push(calibration);
    }


}