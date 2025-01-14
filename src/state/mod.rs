use fltk::image::RgbImage;
use std::path::PathBuf;

#[derive(Clone)]
pub struct ImageState {
    pub image: Option<RgbImage>,
    pub zoom: f64,
    pub path: Option<PathBuf>,
}

impl ImageState {
    pub fn new() -> Self {
        Self {
            image: None,
            zoom: 1.0,
            path: None,
        }
    }
}