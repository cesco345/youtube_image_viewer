use fltk::image::RgbImage;
use crate::scientific::{
    types::{ROIShape, MeasurementTool},
    layers::{Annotation, AnnotationType},
    analysis::IntensityProfile,
};
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct CellMeasurement {
    pub timestamp: chrono::DateTime<Utc>,
    pub area: f64,
    pub perimeter: f64,
    pub mean_intensity: f64,
    pub min_intensity: f64,
    pub max_intensity: f64,
    pub circularity: f64,
    pub calibration_unit: String,
}

impl CellMeasurement {
    pub fn new(
        area: f64,
        perimeter: f64,
        mean_intensity: f64,
        min_intensity: f64,
        max_intensity: f64,
        calibration_unit: String,
    ) -> Self {
        let circularity = if perimeter > 0.0 {
            4.0 * std::f64::consts::PI * area / (perimeter * perimeter)
        } else {
            0.0
        };

        Self {
            timestamp: Utc::now(),
            area,
            perimeter,
            mean_intensity,
            min_intensity,
            max_intensity,
            circularity,
            calibration_unit,
        }
    }

    pub fn format_area(&self) -> String {
        format!("{:.2} {}²", self.area, self.calibration_unit)
    }

    pub fn format_perimeter(&self) -> String {
        format!("{:.2} {}", self.perimeter, self.calibration_unit)
    }

    pub fn format_circularity(&self) -> String {
        format!("{:.3}", self.circularity)
    }

    pub fn format_intensities(&self) -> (String, String, String) {
        (
            format!("{:.1}", self.mean_intensity),
            format!("{:.1}", self.min_intensity),
            format!("{:.1}", self.max_intensity),
        )
    }
}

pub struct CellAnalyzer {
    calibration_scale: f64,  // pixels per unit
    calibration_unit: String,
    measurements: Vec<CellMeasurement>,
}

impl CellAnalyzer {
    pub fn new(calibration_scale: f64, calibration_unit: String) -> Self {
        Self {
            calibration_scale,
            calibration_unit,
            measurements: Vec::new(),
        }
    }

    pub fn analyze_roi(&mut self, roi: &ROIShape, intensity_profile: &IntensityProfile) -> Option<CellMeasurement> {
        let (area_pixels, perimeter_pixels) = match roi {
            ROIShape::Polygon { points } => self.calculate_polygon_metrics(points),
            ROIShape::Ellipse { width, height } => self.calculate_ellipse_metrics(*width, *height),
            ROIShape::Rectangle { width, height } => self.calculate_rectangle_metrics(*width, *height),
            _ => return None,
        };

        // Convert to calibrated units
        let area = area_pixels / (self.calibration_scale * self.calibration_scale);
        let perimeter = perimeter_pixels / self.calibration_scale;

        // Calculate intensity metrics from the profile
        let intensities = intensity_profile.get_values();
        if intensities.is_empty() {
            return None;
        }

        let mean_intensity = intensities.iter().sum::<f64>() / intensities.len() as f64;
        let min_intensity = *intensities.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0);
        let max_intensity = *intensities.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0);

        let measurement = CellMeasurement::new(
            area,
            perimeter,
            mean_intensity,
            min_intensity,
            max_intensity,
            self.calibration_unit.clone(),
        );

        self.measurements.push(measurement.clone());
        Some(measurement)
    }

    fn calculate_polygon_metrics(&self, points: &[(i32, i32)]) -> (f64, f64) {
        if points.len() < 3 {
            return (0.0, 0.0);
        }

        let mut area = 0.0;
        let mut perimeter = 0.0;

        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            // Area calculation using shoelace formula
            area += points[i].0 as f64 * points[j].1 as f64;
            area -= points[j].0 as f64 * points[i].1 as f64;

            // Perimeter calculation
            let dx = (points[j].0 - points[i].0) as f64;
            let dy = (points[j].1 - points[i].1) as f64;
            perimeter += (dx * dx + dy * dy).sqrt();
        }

        area = area.abs() / 2.0;
        (area, perimeter)
    }

    fn calculate_ellipse_metrics(&self, width: i32, height: i32) -> (f64, f64) {
        let a = width as f64 / 2.0;
        let b = height as f64 / 2.0;
        
        // Area of ellipse = π * a * b
        let area = std::f64::consts::PI * a * b;
        
        // Ramanujan approximation for ellipse perimeter
        let h = ((a - b) * (a - b)) / ((a + b) * (a + b));
        let perimeter = std::f64::consts::PI * (a + b) * (1.0 + (3.0 * h) / (10.0 + (4.0 - 3.0 * h).sqrt()));
        
        (area, perimeter)
    }

    fn calculate_rectangle_metrics(&self, width: i32, height: i32) -> (f64, f64) {
        let w = width as f64;
        let h = height as f64;
        let area = w * h;
        let perimeter = 2.0 * (w + h);
        (area, perimeter)
    }

    pub fn get_measurements(&self) -> &Vec<CellMeasurement> {
        &self.measurements
    }

    pub fn clear_measurements(&mut self) {
        self.measurements.clear();
    }

    pub fn create_measurement_annotation(&self, measurement: &CellMeasurement, roi: &ROIShape) -> Annotation {
        let name = format!("Cell Measurement {}", Utc::now().format("%H:%M:%S"));
        
        Annotation {
            name,
            image: RgbImage::new(&[], 1, 1, fltk::enums::ColorDepth::Rgb8).unwrap(),
            annotation_type: AnnotationType::Measurement {
                length: measurement.area as f32,
                unit: format!("{}²", self.calibration_unit),
            },
            visible: true,
            coordinates: match roi {
                ROIShape::Polygon { points } => points.clone(),
                ROIShape::Ellipse { width, height } => vec![(0, 0), (*width, *height)],
                ROIShape::Rectangle { width, height } => vec![(0, 0), (*width, *height)],
                _ => vec![],
            },
        }
    }
}