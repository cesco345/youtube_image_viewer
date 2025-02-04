// src/scientific/tools/interactive/roi/measurements.rs

use crate::scientific::calibration::SpatialCalibration;
use crate::scientific::types::{ROIShape, ROIMeasurements};
use crate::utils::image_data::ImageData;


#[derive(Debug)]
struct IntensityStatistics {
    mean: f64,
    min: f64,
    max: f64,
    integrated: f64,
    std_dev: f64,
}

pub struct MeasurementCalculator {
    calibration: Option<SpatialCalibration>,
}

impl MeasurementCalculator {
    pub fn new(calibration: Option<SpatialCalibration>) -> Self {
        Self { calibration }
    }

    fn generate_ellipse_points(&self, width: i32, height: i32) -> Vec<(i32, i32)> {
        let mut points = Vec::new();
        let center_x = width / 2;
        let center_y = height / 2;
        let a = width as f64 / 2.0;
        let b = height as f64 / 2.0;
        
        // Generate points along the ellipse perimeter (every 5 degrees for efficiency)
        for angle in (0..360).step_by(5) {
            let theta = angle as f64 * std::f64::consts::PI / 180.0;
            let x = (center_x as f64 + a * theta.cos()) as i32;
            let y = (center_y as f64 + b * theta.sin()) as i32;
            points.push((x, y));
        }
        
        // Ensure the ellipse is closed
        if let Some(&first) = points.first() {
            points.push(first);
        }
        
        points
    }

    fn calculate_ellipse_measurements(&self, width: i32, height: i32, image_data: &ImageData) -> ROIMeasurements {
        let area = std::f64::consts::PI * (width as f64 / 2.0) * (height as f64 / 2.0);
        let perimeter = 2.0 * std::f64::consts::PI * ((width as f64 + height as f64) / 4.0);
        
        let points = self.generate_ellipse_points(width, height);
        let intensity_stats = self.calculate_intensity_statistics(&points, image_data);
        let circularity = (4.0 * std::f64::consts::PI * area) / (perimeter * perimeter);
        let aspect_ratio = width as f64 / height as f64;

        ROIMeasurements {
            id: 0,
            shape_type: ROIShape::Ellipse { width, height },
            area: self.calibrate_area(area),
            perimeter: self.calibrate_length(perimeter),
            circularity,
            mean_intensity: intensity_stats.mean,
            min_intensity: intensity_stats.min,
            max_intensity: intensity_stats.max,
            integrated_density: intensity_stats.integrated,
            std_dev: intensity_stats.std_dev,
            aspect_ratio,
            roundness: 1.0 / aspect_ratio,
            solidity: 1.0,  // Ellipse is its own convex hull
            is_calibrated: self.calibration.is_some(),
            units: self.get_units(),
            notes: None,
        }
    }

    pub fn calculate_measurements(
        &self,
        shape: &ROIShape,
        image_data: &ImageData,
    ) -> ROIMeasurements {
        match shape {
            ROIShape::Polygon { points } => self.calculate_polygon_measurements(points, image_data),
            ROIShape::Ellipse { width, height } => {
                self.calculate_ellipse_measurements(*width, *height, image_data)
            }
            ROIShape::Rectangle { width, height } => {
                self.calculate_rectangle_measurements(*width, *height, image_data)
            }
            ROIShape::Line { points } => self.calculate_line_measurements(points, image_data),
        }
    }

    fn calculate_polygon_measurements(
        &self,
        points: &[(i32, i32)],
        image_data: &ImageData,
    ) -> ROIMeasurements {
        let area = self.calculate_polygon_area(points);
        let perimeter = self.calculate_polygon_perimeter(points);
        
        let intensity_stats = self.calculate_intensity_statistics(points, image_data);
        
        let circularity = if perimeter > 0.0 {
            (4.0 * std::f64::consts::PI * area) / (perimeter * perimeter)
        } else {
            0.0
        };
        let (convex_hull_area, solidity) = self.calculate_solidity(points, area);
        let (aspect_ratio, roundness) = self.calculate_shape_factors(points);

        ROIMeasurements {
            id: 0,
            shape_type: ROIShape::Polygon { points: points.to_vec() },
            area: self.calibrate_area(area),
            perimeter: self.calibrate_length(perimeter),
            circularity,
            mean_intensity: intensity_stats.mean,
            min_intensity: intensity_stats.min,
            max_intensity: intensity_stats.max,
            integrated_density: intensity_stats.integrated,
            std_dev: intensity_stats.std_dev,
            aspect_ratio,
            roundness,
            solidity,
            is_calibrated: self.calibration.is_some(),
            units: self.get_units(),
            notes: None,
        }
    }

    fn calculate_polygon_area(&self, points: &[(i32, i32)]) -> f64 {
        if points.len() < 3 {
            return 0.0;
        }

        let mut area = 0.0;
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            area += points[i].0 as f64 * points[j].1 as f64;
            area -= points[j].0 as f64 * points[i].1 as f64;
        }
        
        (area / 2.0).abs()
    }

    fn calculate_polygon_perimeter(&self, points: &[(i32, i32)]) -> f64 {
        if points.len() < 2 {
            return 0.0;
        }

        let mut perimeter = 0.0;
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            let dx = (points[j].0 - points[i].0) as f64;
            let dy = (points[j].1 - points[i].1) as f64;
            perimeter += (dx * dx + dy * dy).sqrt();
        }
        
        perimeter
    }

    fn calculate_centroid(&self, points: &[(i32, i32)]) -> (f64, f64) {
        if points.is_empty() {
            return (0.0, 0.0);
        }

        let mut cx = 0.0;
        let mut cy = 0.0;
        let len = points.len() as f64;

        for point in points {
            cx += point.0 as f64;
            cy += point.1 as f64;
        }

        (cx / len, cy / len)
    }

    fn calculate_solidity(&self, points: &[(i32, i32)], area: f64) -> (f64, f64) {
        // Placeholder for convex hull calculation
        // In practice, you'd implement Graham scan or another convex hull algorithm
        let convex_hull_area = area * 1.1; // Simplified approximation
        let solidity = area / convex_hull_area;
        (convex_hull_area, solidity)
    }

    fn calculate_shape_factors(&self, points: &[(i32, i32)]) -> (f64, f64) {
        // Calculate minimum bounding box
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;

        for point in points {
            min_x = min_x.min(point.0 as f64);
            min_y = min_y.min(point.1 as f64);
            max_x = max_x.max(point.0 as f64);
            max_y = max_y.max(point.1 as f64);
        }

        let width = max_x - min_x;
        let height = max_y - min_y;
        
        let aspect_ratio = if height != 0.0 { width / height } else { 1.0 };
        let roundness = if width != 0.0 { height / width } else { 1.0 };

        (aspect_ratio, roundness)
    }

    fn calculate_rectangle_measurements(&self, width: i32, height: i32, image_data: &ImageData) -> ROIMeasurements {
        let area = width as f64 * height as f64;
        let perimeter = 2.0 * (width as f64 + height as f64);
        let points = vec![
            (0, 0),
            (width, 0),
            (width, height),
            (0, height),
            (0, 0),
        ];
        let intensity_stats = self.calculate_intensity_statistics(&points, image_data);
        let aspect_ratio = width as f64 / height as f64;

        ROIMeasurements {
            id: 0,
            shape_type: ROIShape::Rectangle { width, height },
            area: self.calibrate_area(area),
            perimeter: self.calibrate_length(perimeter),
            circularity: std::f64::consts::PI,  // Rectangle has constant circularity
            mean_intensity: intensity_stats.mean,
            min_intensity: intensity_stats.min,
            max_intensity: intensity_stats.max,
            integrated_density: intensity_stats.integrated,
            std_dev: intensity_stats.std_dev,
            aspect_ratio,
            roundness: 1.0 / aspect_ratio,
            solidity: 1.0,
            is_calibrated: self.calibration.is_some(),
            units: self.get_units(),
            notes: None,
        }
    }

    fn calculate_line_measurements(&self, points: &[(i32, i32)], image_data: &ImageData) -> ROIMeasurements {
        let perimeter = if points.len() >= 2 {
            let dx = (points[1].0 - points[0].0) as f64;
            let dy = (points[1].1 - points[0].1) as f64;
            (dx * dx + dy * dy).sqrt()
        } else {
            0.0
        };

        let intensity_stats = self.calculate_intensity_statistics(points, image_data);

        ROIMeasurements {
            id: 0,
            shape_type: ROIShape::Line { points: points.to_vec() },
            area: 0.0,  // Lines have no area
            perimeter,
            circularity: 0.0,  // Not applicable for lines
            mean_intensity: intensity_stats.mean,
            min_intensity: intensity_stats.min,
            max_intensity: intensity_stats.max,
            integrated_density: intensity_stats.integrated,
            std_dev: intensity_stats.std_dev,
            aspect_ratio: 0.0,  // Not applicable for lines
            roundness: 0.0,  // Not applicable for lines
            solidity: 1.0,
            is_calibrated: self.calibration.is_some(),
            units: self.get_units(),
            notes: None,
        }
    }

    fn calculate_intensity_statistics(
        &self,
        points: &[(i32, i32)],
        image: &ImageData,
    ) -> IntensityStatistics {
        // Get bounding box
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for &(x, y) in points {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }

        // Calculate statistics
        let mut sum = 0.0;
        let mut sum_sq = 0.0;
        let mut count = 0;
        let mut min_val = f64::MAX;
        let mut max_val = f64::MIN;

        // Iterate through bounding box
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.point_in_polygon((x, y), points) {
                    let value = image.get_intensity(x, y) as f64;
                    sum += value;
                    sum_sq += value * value;
                    min_val = min_val.min(value);
                    max_val = max_val.max(value);
                    count += 1;
                }
            }
        }

        let mean = if count > 0 { sum / count as f64 } else { 0.0 };
        let variance = if count > 1 {
            (sum_sq - (sum * sum) / count as f64) / (count - 1) as f64
        } else {
            0.0
        };

        IntensityStatistics {
            mean,
            min: min_val,
            max: max_val,
            integrated: sum,
            std_dev: variance.sqrt(),
        }
    }

    fn point_in_polygon(&self, point: (i32, i32), vertices: &[(i32, i32)]) -> bool {
        if vertices.len() < 3 {
            return false;
        }

        let mut inside = false;
        let mut j = vertices.len() - 1;

        for i in 0..vertices.len() {
            if ((vertices[i].1 > point.1) != (vertices[j].1 > point.1)) &&
                (point.0 < (vertices[j].0 - vertices[i].0) * (point.1 - vertices[i].1) /
                    (vertices[j].1 - vertices[i].1) + vertices[i].0)
            {
                inside = !inside;
            }
            j = i;
        }

        inside
    }

    fn calibrate_area(&self, area: f64) -> f64 {
        if let Some(cal) = &self.calibration {
            area / (cal.pixels_per_unit as f64).powi(2)
        } else {
            area
        }
    }

    fn calibrate_length(&self, length: f64) -> f64 {
        if let Some(cal) = &self.calibration {
            length / cal.pixels_per_unit as f64
        } else {
            length
        }
    }

    fn get_units(&self) -> String {
        self.calibration
            .as_ref()
            .map(|c| c.unit.clone())
            .unwrap_or_else(|| "pixels".to_string())
    }
}
