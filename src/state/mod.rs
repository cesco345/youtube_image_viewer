// src/state/mod.rs

use fltk::image::RgbImage;
use std::path::PathBuf;
use crate::menu::edit::crop::CropSelection;
use crate::menu::edit::watermark::WatermarkOptions;

mod watermark_state;
pub use watermark_state::{WatermarkState, WatermarkError};

#[derive(Clone)]
pub struct ImageState {
    pub image: Option<RgbImage>,
    pub zoom: f64,
    pub path: Option<PathBuf>,
    pub crop_selection: Option<CropSelection>,
    pub watermark_state: WatermarkState,
}

impl ImageState {
    pub fn new() -> Self {
        Self {
            image: None,
            zoom: 1.0,
            path: None,
            crop_selection: None,
            watermark_state: WatermarkState::new(),
        }
    }

    pub fn reset_watermark(&mut self) {
        self.watermark_state = WatermarkState::new();
    }

    pub fn update_watermark_preview(&mut self) -> Result<(), WatermarkError> {
        if let Some(_template) = self.watermark_state.get_current_template() {
            self.watermark_state.toggle_preview();
        }
        Ok(())
    }

    pub fn get_watermark_options(&self) -> WatermarkOptions {
        self.watermark_state.get_current_options()
    }

    pub fn list_watermark_templates(&self) -> Result<Vec<String>, WatermarkError> {
        self.watermark_state.list_templates()
    }
}