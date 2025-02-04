// src/scientific/tools/interactive/roi/utils/validation.rs

use super::{ROIResult, ROIError};
use crate::scientific::types::{ROIShape, ROIProperties};

pub struct ROIValidator;

impl ROIValidator {
    pub fn validate_shape(shape: &ROIShape) -> ROIResult<()> {
        match shape {
            ROIShape::Polygon { points } => Self::validate_polygon(points),
            ROIShape::Rectangle { width, height } => Self::validate_rectangle(*width, *height),
            ROIShape::Ellipse { width, height } => Self::validate_ellipse(*width, *height),
            ROIShape::Line { points } => Self::validate_line(points),
        }
    }

    fn validate_polygon(points: &[(i32, i32)]) -> ROIResult<()> {
        if points.len() < 3 {
            return Err(ROIError::ValidationError(
                "Polygon must have at least 3 points".to_string()
            ));
        }

        // Check for self-intersection
        if Self::has_self_intersection(points) {
            return Err(ROIError::ValidationError(
                "Polygon cannot self-intersect".to_string()
            ));
        }

        Ok(())
    }

    fn validate_rectangle(width: i32, height: i32) -> ROIResult<()> {
        if width <= 0 || height <= 0 {
            return Err(ROIError::ValidationError(
                "Rectangle dimensions must be positive".to_string()
            ));
        }
        Ok(())
    }

    fn validate_ellipse(width: i32, height: i32) -> ROIResult<()> {
        if width <= 0 || height <= 0 {
            return Err(ROIError::ValidationError(
                "Ellipse dimensions must be positive".to_string()
            ));
        }
        Ok(())
    }

    fn validate_line(points: &[(i32, i32)]) -> ROIResult<()> {
        if points.len() < 2 {
            return Err(ROIError::ValidationError(
                "Line must have at least 2 points".to_string()
            ));
        }
        Ok(())
    }

    pub fn validate_properties(props: &ROIProperties) -> ROIResult<()> {
        if props.line_width <= 0 {
            return Err(ROIError::ValidationError(
                "Line width must be positive".to_string()
            ));
        }

        Ok(())
    }

    pub fn validate_bounds(point: (i32, i32), width: i32, height: i32) -> ROIResult<()> {
        if point.0 < 0 || point.0 >= width || point.1 < 0 || point.1 >= height {
            return Err(ROIError::OutOfBounds(
                format!("Point {:?} is outside image bounds {}x{}", point, width, height)
            ));
        }
        Ok(())
    }

    fn has_self_intersection(points: &[(i32, i32)]) -> bool {
        // Simple implementation of line segment intersection check
        if points.len() < 4 {
            return false;
        }

        for i in 0..points.len() - 1 {
            let p1 = points[i];
            let p2 = points[i + 1];
            
            for j in i + 2..points.len() - 1 {
                let p3 = points[j];
                let p4 = points[j + 1];
                
                if Self::line_segments_intersect(p1, p2, p3, p4) {
                    return true;
                }
            }
        }
        
        false
    }

    fn line_segments_intersect(
        p1: (i32, i32),
        p2: (i32, i32),
        p3: (i32, i32),
        p4: (i32, i32)
    ) -> bool {
        let o1 = Self::orientation(p1, p2, p3);
        let o2 = Self::orientation(p1, p2, p4);
        let o3 = Self::orientation(p3, p4, p1);
        let o4 = Self::orientation(p3, p4, p2);

        o1 != o2 && o3 != o4
    }

    fn orientation(p: (i32, i32), q: (i32, i32), r: (i32, i32)) -> bool {
        let val = (q.1 - p.1) * (r.0 - q.0) - (q.0 - p.0) * (r.1 - q.1);
        val > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polygon_validation() {
        let valid_polygon = vec![(0, 0), (0, 10), (10, 10), (10, 0)];
        assert!(ROIValidator::validate_polygon(&valid_polygon).is_ok());

        let invalid_polygon = vec![(0, 0), (0, 10)];
        assert!(ROIValidator::validate_polygon(&invalid_polygon).is_err());
    }

    #[test]
    fn test_rectangle_validation() {
        assert!(ROIValidator::validate_rectangle(10, 10).is_ok());
        assert!(ROIValidator::validate_rectangle(0, 10).is_err());
        assert!(ROIValidator::validate_rectangle(10, 0).is_err());
    }

    #[test]
    fn test_bounds_validation() {
        assert!(ROIValidator::validate_bounds((5, 5), 10, 10).is_ok());
        assert!(ROIValidator::validate_bounds((11, 5), 10, 10).is_err());
    }
}