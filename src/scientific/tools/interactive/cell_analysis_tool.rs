use fltk::{image::RgbImage, prelude::*, enums::Event};
use crate::scientific::{
    analysis::{CellMeasurement, CellAnalyzer, CellStatistics},
    types::{ROIShape, CellMeasurementMode},
    layers::{Annotation, AnnotationType},
    analysis::cell_statistics::StatisticalAnalysis,
};
use crate::scientific::IntensityProfile;

pub struct CellAnalysisTool {
    active: bool,
    analyzer: CellAnalyzer,
    measurement_mode: CellMeasurementMode,
    current_measurements: Vec<CellMeasurement>,
    current_roi: Option<ROIShape>,
    roi_points: Vec<(i32, i32)>,
}

impl CellAnalysisTool {
    pub fn new(calibration_scale: f64, unit: String) -> Self {
        Self {
            active: false,
            analyzer: CellAnalyzer::new(calibration_scale, unit),
            measurement_mode: CellMeasurementMode::Single,
            current_measurements: Vec::new(),
            current_roi: None,
            roi_points: Vec::new(),
        }
    }

    pub fn get_measurements(&self) -> &[CellMeasurement] {
        &self.current_measurements
    }

    pub fn handle_event(
        &mut self,
        event: Event,
        coords: (i32, i32)
    ) -> (bool, Option<ROIShape>, Vec<(i32, i32)>) {
        println!("CellAnalysisTool handling event: {:?} at coords: {:?}", event, coords);
        if !self.active {
            println!("Tool not active");
            return (false, None, Vec::new());
        }
    
        match event {
            Event::Push => {
                println!("Starting ROI at {:?}", coords);
                self.roi_points.clear();
                self.roi_points.push(coords);
                self.current_roi = Some(ROIShape::Polygon { points: vec![coords] });
                (true, self.current_roi.clone(), self.roi_points.clone())
            },
            Event::Drag => {
                println!("Updating ROI to {:?}", coords);
                if let Some(ROIShape::Polygon { ref mut points }) = self.current_roi {
                    points.push(coords);
                    self.roi_points.push(coords);
                }
                (true, self.current_roi.clone(), self.roi_points.clone())
            },
            Event::Released => {
                println!("Finishing ROI at {:?}", coords);
                if let Some(ROIShape::Polygon { ref mut points }) = self.current_roi.take() {
                    points.push(coords);
                    points.push(points[0]); // Close the polygon
                    self.roi_points.push(coords);
                    self.roi_points.push(self.roi_points[0]); // Close the polygon
                }
                (true, self.current_roi.clone(), self.roi_points.clone())
            },
            _ => (false, None, Vec::new()),
        }
    }

    pub fn process_intensity_profile(
        &mut self,
        intensity_profile: IntensityProfile,
        points: &[(i32, i32)],
        image_width: i32,
        image_height: i32
    ) -> Option<(CellMeasurement, Annotation)> {
        println!("Processing intensity profile with {} points", points.len());
        
        match self.analyzer.analyze_roi(
            &ROIShape::Polygon { points: points.to_vec() },
            &intensity_profile
        ) {
            Some(measurement) => {
                println!("Measurement created: Area={}, Perimeter={}", 
                        measurement.area, measurement.perimeter);
                self.current_measurements.push(measurement.clone());
                
                let annotation = self.create_roi_annotation(points, image_width, image_height);
                println!("Annotation created successfully");
                Some((measurement, annotation))
            }
            None => {
                println!("Failed to analyze ROI");
                None
            }
        }
    }

    pub fn create_roi_annotation(
        &self,
        points: &[(i32, i32)],
        width: i32,
        height: i32
    ) -> Annotation {
        println!("Creating ROI annotation with dimensions {}x{}", width, height);
    
        // Create properly sized image data
        let image_data = vec![0u8; (width * height * 3) as usize];
    
        let annotation = Annotation {
            name: format!("Cell Analysis ROI {}", self.current_measurements.len()),
            image: RgbImage::new(
                &image_data,
                width,
                height,
                fltk::enums::ColorDepth::Rgb8
            ).expect("Failed to create annotation image"),
            annotation_type: AnnotationType::ROI {
                color: (0, 255, 0),
                line_width: 2,
            },
            visible: true,
            coordinates: points.to_vec(),
        };
        
        println!("Created ROI annotation with {} points", points.len());
        annotation
    }

    // Changed to take an intensity profile directly instead of trying to get it from ScientificState
    pub fn analyze_roi(
        &mut self,
        points: &[(i32, i32)],
        intensity_profile: &IntensityProfile
    ) -> Option<CellMeasurement> {
        let points_vec = points.to_vec();
        self.analyzer.analyze_roi(
            &ROIShape::Polygon { points: points_vec },
            intensity_profile
        )
    }

    pub fn process_measurement(&mut self, intensity_profile: IntensityProfile, points: &[(i32, i32)]) {
        if let Some(measurement) = self.analyzer.analyze_roi(
            &ROIShape::Polygon { points: points.to_vec() },
            &intensity_profile
        ) {
            self.current_measurements.push(measurement);
        }
    }

    pub fn set_mode(&mut self, mode: CellMeasurementMode) {
        self.measurement_mode = mode;
    }

    pub fn clear_measurements(&mut self) {
        self.current_measurements.clear();
    }

    pub fn get_statistics(&self) -> Option<CellStatistics> {
        if self.current_measurements.is_empty() {
            None
        } else {
            Some(self.current_measurements.as_slice().analyze())
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.current_roi = None;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

pub trait CellAnalysisState {
    fn init_cell_analysis(&mut self, calibration_scale: f64, unit: String);
    fn start_cell_analysis(&mut self, mode: CellMeasurementMode);
    fn stop_cell_analysis(&mut self);
    fn is_analyzing_cells(&self) -> bool;
    fn get_cell_statistics(&self) -> Option<CellStatistics>;
    fn get_measurements(&self) -> Option<Vec<CellMeasurement>>;
}



