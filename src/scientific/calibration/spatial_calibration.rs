//src/scientific/calibration/spatial_calibration.rs
use fltk::image::RgbImage;

#[derive(Clone, Debug)]
pub struct CalibrationPoint {
    pub pixel_coord: (i32, i32),
    pub real_coord: (f32, f32),
}

#[derive(Clone)]
pub struct SpatialCalibration {
    pub points: Vec<CalibrationPoint>,
    pub pixels_per_unit: f32,
    pub unit: String,
    pub transformation_matrix: Option<[[f32; 3]; 3]>,
}

impl SpatialCalibration {
    pub fn new(unit: String) -> Self {
        Self {
            points: Vec::new(),
            pixels_per_unit: 1.0,
            unit,
            transformation_matrix: None,
        }
    }

    pub fn add_point(&mut self, pixel_coord: (i32, i32), real_coord: (f32, f32)) {
        self.points.push(CalibrationPoint {
            pixel_coord,
            real_coord,
        });
        self.calculate_calibration();
    }

    pub fn calculate_calibration(&mut self) {
        if self.points.len() < 2 {
            return;
        }

        let mut total_ratio = 0.0;
        let mut count = 0;

        for i in 0..self.points.len() {
            for j in i+1..self.points.len() {
                let pixel_dist = ((self.points[i].pixel_coord.0 - self.points[j].pixel_coord.0).pow(2) +
                                (self.points[i].pixel_coord.1 - self.points[j].pixel_coord.1).pow(2)) as f32;
                let real_dist = ((self.points[i].real_coord.0 - self.points[j].real_coord.0).powi(2) +
                               (self.points[i].real_coord.1 - self.points[j].real_coord.1).powi(2)).sqrt();
                
                if real_dist > 0.0 {
                    total_ratio += pixel_dist.sqrt() / real_dist;
                    count += 1;
                }
            }
        }

        if count > 0 {
            self.pixels_per_unit = total_ratio / count as f32;
        }
    }

    pub fn pixel_to_real(&self, pixel_coord: (i32, i32)) -> (f32, f32) {
        if let Some(matrix) = self.transformation_matrix {
            let x = matrix[0][0] * pixel_coord.0 as f32 + matrix[0][1] * pixel_coord.1 as f32 + matrix[0][2];
            let y = matrix[1][0] * pixel_coord.0 as f32 + matrix[1][1] * pixel_coord.1 as f32 + matrix[1][2];
            (x, y)
        } else {
            (pixel_coord.0 as f32 / self.pixels_per_unit, 
             pixel_coord.1 as f32 / self.pixels_per_unit)
        }
    }

    pub fn real_to_pixel(&self, real_coord: (f32, f32)) -> (i32, i32) {
        if let Some(matrix) = self.transformation_matrix {
            let det = matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
            if det != 0.0 {
                let inv_det = 1.0 / det;
                let x = ((matrix[1][1] * (real_coord.0 - matrix[0][2]) -
                         matrix[0][1] * (real_coord.1 - matrix[1][2])) * inv_det) as i32;
                let y = ((matrix[0][0] * (real_coord.1 - matrix[1][2]) -
                         matrix[1][0] * (real_coord.0 - matrix[0][2])) * inv_det) as i32;
                (x, y)
            } else {
                ((real_coord.0 * self.pixels_per_unit) as i32,
                 (real_coord.1 * self.pixels_per_unit) as i32)
            }
        } else {
            ((real_coord.0 * self.pixels_per_unit) as i32,
             (real_coord.1 * self.pixels_per_unit) as i32)
        }
    }
}