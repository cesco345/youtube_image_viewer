// src/state/mod.rs
use fltk::image::RgbImage;
use std::path::PathBuf;
use crate::menu::edit::crop::CropSelection;
use crate::menu::edit::watermark::WatermarkOptions;

mod watermark_state;
mod filter_state;
mod layer_state;

pub use watermark_state::{WatermarkState, WatermarkError};
pub use filter_state::{FilterState, FilterError};
pub use layer_state::LayerState;

#[derive(Clone)]
pub struct ImageState {
    pub image: Option<RgbImage>,
    pub zoom: f64,  // Changed from f32 to f64
    pub path: Option<PathBuf>,
    pub crop_selection: Option<CropSelection>,
    pub watermark_state: WatermarkState,
    pub filter_state: FilterState,
    pub layer_state: LayerState,
}

impl ImageState {
    pub fn new() -> Self {
        Self {
            image: None,
            zoom: 1.0,
            path: None,
            crop_selection: None,
            watermark_state: WatermarkState::new(),
            filter_state: FilterState::new(),
            layer_state: LayerState::new(),
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

    pub fn update_filter_preview(&mut self) -> Result<(), FilterError> {
        self.filter_state.toggle_preview();
        Ok(())
    }

    pub fn reset_filter(&mut self) {
        self.filter_state = FilterState::new();
    }

    pub fn reset_layers(&mut self) {
        self.layer_state = LayerState::new();
    }
}