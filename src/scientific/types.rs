#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellMeasurementMode {
    Single,
    Batch,
    AutoDetect,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ROIShape {
    Polygon {
        points: Vec<(i32, i32)>
    },
    Ellipse {
        width: i32,
        height: i32,
    },
    Rectangle {
        width: i32,
        height: i32,
    },
    Line {
        points: Vec<(i32, i32)>
    },
}


#[derive(Clone, Debug, PartialEq)]
pub struct ROITool {
    pub shape: ROIShape,
    pub color: (u8, u8, u8),
    pub line_width: i32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum LegendPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl ROITool {
    pub fn new(shape: ROIShape, color: (u8, u8, u8), line_width: i32) -> Self {
        Self {
            shape,
            color,
            line_width,
        }
    }
}

pub struct MeasurementTool {
    pub active: bool,
    pub unit: String,
    pub color: (u8, u8, u8),
    pub line_width: i32,
}

impl MeasurementTool {
    pub fn new(unit: String) -> Self {
        Self {
            active: false,
            unit,
            color: (255, 255, 0),
            line_width: 2,
        }
    }
}
