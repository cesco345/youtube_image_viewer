//src/scientific/layers/metadata.rs
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone, Default, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Calibration {
    pub objective: String,
    pub pixels_per_unit: f32,
    pub unit: String,
    pub pixel_distance: f64,
    pub real_distance: f64,
    pub timestamp: DateTime<Utc>,
    pub image_name: Option<String>,
    pub notes: Option<String>,
}

impl Calibration {
    pub fn to_report_string(&self) -> String {
        format!(
            "Date: {}\n\
             Image: {}\n\
             Objective: {}\n\
             Scale: {} pixels = {} {}\n\
             Scale Ratio: {:.2} pixels/{}\n\
             Notes: {}\n",
            self.timestamp.format("%Y-%m-%d %H:%M:%S"),
            self.image_name.as_deref().unwrap_or("Unknown"),
            self.objective,
            self.pixel_distance,
            self.real_distance,
            self.unit,
            self.pixels_per_unit,
            self.unit,
            self.notes.as_deref().unwrap_or("")
        )
    }
    pub fn new(
        objective: String,
        pixels_per_unit: f32,
        unit: String,
        pixel_distance: f64,
        real_distance: f64,
    ) -> Self {
        Self {
            objective,
            pixels_per_unit,
            unit,
            pixel_distance,
            real_distance,
            timestamp: Utc::now(),
            image_name: None,
            notes: None,
        }
    }
}