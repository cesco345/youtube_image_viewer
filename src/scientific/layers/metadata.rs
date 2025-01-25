// src/scientific/layers/metadata.rs
use chrono::{DateTime, Utc};

#[derive(Clone, Default)]
pub struct Metadata {
    pub acquisition_time: Option<DateTime<Utc>>,
    pub exposure_time: Option<f32>,
    pub gain: Option<f32>,
    pub objective: Option<String>,
    pub binning: Option<i32>,
    pub pixel_size: Option<f32>,
    pub comments: Option<String>,
    pub scale_calibration: Option<(f32, String)>,
}

#[derive(Clone)]
pub struct Calibration {
    pub objective: String,
    pub pixels_per_unit: f32,
    pub unit: String,
    pub pixel_distance: f64,
    pub real_distance: f64,
}