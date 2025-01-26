//src/state/mod.rs

use fltk::image::RgbImage;
use std::path::PathBuf;
use crate::menu::edit::crop::crop_tool::CropSelection;
use crate::state::filter_state::FilterState;
use crate::state::watermark_state::WatermarkState;

pub use layer_state::{Layer, LayerGroup, LayerState};
use crate::menu::edit::watermark::WatermarkOptions;
use crate::scientific::state::scientific_state::ScientificState;

pub mod filter_state;
pub mod watermark_state;
mod layer_state;

#[derive(Debug)]
pub struct FilterError {
   pub message: String,
}

impl From<FilterError> for filter_state::FilterError {
   fn from(error: FilterError) -> Self {
       filter_state::FilterError {
           message: error.message
       }
   }
}

pub struct ImageState {
   pub image: Option<RgbImage>,
   pub path: Option<PathBuf>,
   pub crop_selection: Option<CropSelection>,
   pub filter_state: FilterState,
   pub watermark_state: WatermarkState,
   pub layer_state: LayerState,
   pub scientific_state: ScientificState,
   pub zoom: f32,
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
           scientific_state: ScientificState::new(),
           zoom: 1.0,
       }
   }

   pub fn get_watermark_options(&self) -> WatermarkOptions {
       self.watermark_state.get_current_options()
   }
   
   pub fn get_scientific_state(&self) -> &ScientificState {
       &self.scientific_state
   }
   
   pub fn get_scientific_state_mut(&mut self) -> &mut ScientificState {
       &mut self.scientific_state
   }
}