// src/scientific/layers/annotation.rs
use fltk::image::RgbImage;

#[derive(Clone, PartialEq)]
pub enum AnnotationType {
    ROI {
        color: (u8, u8, u8),
        line_width: i32,
    },
    Scale {
        pixels_per_unit: f32,
        unit: String,
    },
    Measurement {
        length: f32,
        unit: String,
    },
    Text {
        content: String,
        font_size: i32,
    },
}

#[derive(Clone)]
pub struct Annotation {
    pub name: String,
    pub image: RgbImage,
    pub annotation_type: AnnotationType,
    pub visible: bool,
    pub coordinates: Vec<(i32, i32)>,
}