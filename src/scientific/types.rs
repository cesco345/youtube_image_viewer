// src/scientific/types.rs

#[derive(Clone, Debug)]
pub enum ROIShape {
    Rectangle { width: i32, height: i32 },
    Ellipse { width: i32, height: i32 },
    Polygon { points: Vec<(i32, i32)> },
    Line { points: Vec<(i32, i32)> },
}

pub struct ROITool {
    pub shape: ROIShape,
    pub color: (u8, u8, u8),
    pub line_width: i32,
}

impl ROITool {
    pub fn new(shape: ROIShape, color: (u8, u8, u8), line_width: i32) -> Self {
        Self { shape, color, line_width }
    }
}

pub struct MeasurementTool;