// src/state/mod.rs
use fltk::image::RgbImage;
use std::path::PathBuf;
use crate::menu::edit::crop::CropSelection;  // Updated import path

#[derive(Clone)]
pub struct ImageState {
    pub image: Option<RgbImage>,
    pub zoom: f64,
    pub path: Option<PathBuf>,
    pub crop_selection: Option<CropSelection>,
}

impl ImageState {
    pub fn new() -> Self {
        Self {
            image: None,
            zoom: 1.0,
            path: None,
            crop_selection: None,
        }
    }
}
