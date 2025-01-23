use fltk::image::RgbImage;
use std::path::PathBuf;
use crate::menu::edit::crop::crop_tool::CropSelection;
use crate::state::filter_state::FilterState;
use crate::state::watermark_state::WatermarkState;
pub use layer_state::{Layer, LayerGroup, LayerState};  // Add LayerGroup here
use crate::menu::edit::watermark::WatermarkOptions;

pub mod filter_state;
pub mod watermark_state;
mod layer_state;

#[derive(Debug)]
pub struct FilterError {
    pub message: String,
}

// Implement From for FilterError
impl From<FilterError> for filter_state::FilterError {
    fn from(error: FilterError) -> Self {
        filter_state::FilterError {
            message: error.message
        }
    }
}

pub struct ImageState {
    pub image: Option<RgbImage>,
    pub path: Option<PathBuf>,  // Changed from String to PathBuf
    pub crop_selection: Option<CropSelection>,
    pub filter_state: FilterState,
    pub watermark_state: WatermarkState,
    pub layer_state: LayerState,
    pub zoom: f32,  // Added zoom field
}

impl ImageState {
    pub fn new() -> Self {
        Self {
            image: None,
            path: None,
            crop_selection: None,
            filter_state: FilterState::new(),
            watermark_state: WatermarkState::new(),
            layer_state: LayerState::new(),
            zoom: 1.0,
        }
    }

    pub fn get_watermark_options(&self) -> WatermarkOptions {
        self.watermark_state.get_current_options()
    }
}