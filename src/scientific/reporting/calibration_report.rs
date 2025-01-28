use std::path::Path;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::scientific::layers::metadata::Calibration;

#[derive(Serialize, Deserialize)]
pub struct CalibrationReport {
    calibrations: Vec<Calibration>,
    export_date: DateTime<Utc>,
}

impl CalibrationReport {
    pub fn new(calibrations: Vec<Calibration>) -> Self {
        Self {
            calibrations,
            export_date: Utc::now(),
        }
    }

    pub fn export_csv<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut wtr = csv::Writer::from_path(path)?;
        
        // Write headers
        wtr.write_record(&[
            "Date",
            "Image",
            "Objective",
            "Pixel Distance",
            "Real Distance",
            "Unit",
            "Scale Ratio",
            "Notes"
        ])?;

        // Write data
        for cal in &self.calibrations {
            wtr.write_record(&[
                cal.timestamp.to_rfc3339(),
                cal.image_name.as_deref().unwrap_or("Unknown").to_string(),
                cal.objective.clone(),
                cal.pixel_distance.to_string(),
                cal.real_distance.to_string(),
                cal.unit.clone(),
                cal.pixels_per_unit.to_string(),
                cal.notes.as_deref().unwrap_or("").to_string(),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }

    pub fn export_markdown<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut content = String::new();
        content.push_str("# Calibration Report\n\n");
        content.push_str(&format!("Generated: {}\n\n", self.export_date.format("%Y-%m-%d %H:%M:%S")));

        for cal in &self.calibrations {
            content.push_str("## Calibration Entry\n\n");
            content.push_str(&cal.to_report_string());
            content.push_str("\n---\n\n");
        }

        std::fs::write(path, content)
    }
}