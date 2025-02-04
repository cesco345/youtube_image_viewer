// src/scientific/tools/interactive/roi/utils/mod.rs

mod conversion;
mod validation;

pub use conversion::*;
pub use validation::*;

#[derive(Debug, Clone)]
pub struct ScalingInfo {
    pub scale: f32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub frame_x: i32,
    pub frame_y: i32,
    pub img_w: i32,
    pub img_h: i32,
}

// Common error type for ROI utilities
#[derive(Debug)]
pub enum ROIError {
    InvalidCoordinates(String),
    OutOfBounds(String),
    ValidationError(String),
}

pub type ROIResult<T> = Result<T, ROIError>;