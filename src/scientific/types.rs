//src/scientific/types.rs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellMeasurementMode {
    Single,
    Batch,
    AutoDetect,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineStyle {
    Solid,
    Dash,
    Dot,
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
impl std::fmt::Display for ROIShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ROIShape::Rectangle { width: _, height: _ } => write!(f, "Rectangle"),
            ROIShape::Ellipse { width: _, height: _ } => write!(f, "Ellipse"),
            ROIShape::Line { points: _ } => write!(f, "Line"),
            ROIShape::Polygon { points: _ } => write!(f, "Polygon"),
        }
    }
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

#[derive(Debug, Clone)]
pub struct ROIMeasurements {
    pub id: i32,                  // Added to match usage
    pub area: f64,              
    pub perimeter: f64,         
    pub circularity: f64,
    pub mean_intensity: f64,
    pub min_intensity: f64,
    pub max_intensity: f64,
    pub integrated_density: f64,  // Added to match usage
    pub std_dev: f64,
    pub aspect_ratio: f64,      
    pub roundness: f64,        
    pub solidity: f64,         
    pub shape_type: ROIShape,     // Added to match usage
    pub is_calibrated: bool,
    pub units: String,
    pub notes: Option<String>,    // Added to match usage
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
