// src/utils/template_utils.rs

use std::path::{Path, PathBuf};
use std::fs;
use serde_json;

use crate::menu::edit::watermark::{
    WatermarkError,
    WatermarkTemplate,
    WatermarkType,
    WatermarkData,
    WatermarkOptions,
};

pub struct TemplateExport {
    pub template: WatermarkTemplate,
    pub associated_files: Vec<(PathBuf, Vec<u8>)>,
}

pub fn export_template(template: &WatermarkTemplate, export_dir: &Path) -> Result<(), WatermarkError> {
    // Create export directory if it doesn't exist
    fs::create_dir_all(export_dir).map_err(|e| {
        WatermarkError::TemplateError(format!("Failed to create export directory: {}", e))
    })?;

    // Export the template JSON
    let template_path = export_dir.join(format!("{}.json", &template.name));
    let template_json = serde_json::to_string_pretty(template).map_err(|e| {
        WatermarkError::TemplateError(format!("Failed to serialize template: {}", e))
    })?;

    fs::write(&template_path, template_json).map_err(|e| {
        WatermarkError::TemplateError(format!("Failed to write template file: {}", e))
    })?;

    // Export associated files (like images) if any
    if let WatermarkData::ImagePath(ref image_path) = template.data {
        if let Ok(image_data) = fs::read(image_path) {
            let target_path = export_dir.join(image_path.file_name().unwrap_or_default());
            fs::write(&target_path, image_data).map_err(|e| {
                WatermarkError::TemplateError(format!("Failed to write image file: {}", e))
            })?;
        }
    }

    Ok(())
}

pub fn import_template<P: AsRef<Path>>(template_path: P) -> Result<TemplateExport, WatermarkError> {
    let template_json = fs::read_to_string(&template_path).map_err(|e| {
        WatermarkError::TemplateError(format!("Failed to read template file: {}", e))
    })?;

    let template: WatermarkTemplate = serde_json::from_str(&template_json).map_err(|e| {
        WatermarkError::TemplateError(format!("Failed to parse template JSON: {}", e))
    })?;

    let mut associated_files = Vec::new();

    // Handle associated files
    if let WatermarkData::ImagePath(ref path) = template.data {
        let image_path = template_path.as_ref().parent().unwrap_or(Path::new("")).join(path);
        if image_path.exists() {
            let image_data = fs::read(&image_path).map_err(|e| {
                WatermarkError::TemplateError(format!("Failed to read image file: {}", e))
            })?;
            associated_files.push((image_path, image_data));
        }
    }

    Ok(TemplateExport {
        template,
        associated_files,
    })
}

pub fn validate_template(template: &WatermarkTemplate) -> Result<(), WatermarkError> {
    // Validate basic template properties
    if template.name.is_empty() {
        return Err(WatermarkError::TemplateError(
            "Template name cannot be empty".to_string(),
        ));
    }

    fn validate_options(options: &WatermarkOptions) -> Result<(), WatermarkError> {
        // Validate opacity
        if !(0.0..=1.0).contains(&options.opacity) {
            return Err(WatermarkError::TemplateError(
                "Opacity must be between 0.0 and 1.0".to_string(),
            ));
        }
    
        // Validate rotation
        if let Some(rotation) = options.rotation {
            if !(0.0..=360.0).contains(&rotation) {
                return Err(WatermarkError::TemplateError(
                    "Rotation must be between 0 and 360 degrees".to_string(),
                ));
            }
        }
    
        // Validate scale
        if let Some(scale) = options.scale {
            if scale <= 0.0 {
                return Err(WatermarkError::TemplateError(
                    "Scale must be greater than 0".to_string(),
                ));
            }
        }
    
        Ok(())
    }

    // Validate watermark type and data consistency
    match (&template.watermark_type, &template.data) {
        (WatermarkType::Text { .. }, WatermarkData::Text(_)) => Ok(()),
        (WatermarkType::Image { .. }, WatermarkData::ImagePath(_)) => Ok(()),
        _ => Err(WatermarkError::TemplateError(
            "Watermark type and data are inconsistent".to_string(),
        )),
    }
}