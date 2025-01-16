// menu/edit/watermark/mod.rs

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use chrono::{DateTime, Utc};

mod blend;
pub mod dialog;
mod fonts;
mod handlers;
pub mod image_watermark;
mod position;
mod templates;
pub mod text_watermark;

// Re-export everything needed externally
pub use blend::{BlendMode};
pub use position::{Position, WatermarkPosition};
pub use handlers::*;


#[derive(Debug, Clone)]
pub enum WatermarkError {
    InvalidPosition(String),
    InvalidOpacity(String),
    TextRenderingError(String),
    ImageLoadError(String),
    BlendingError(String),
    FontError(String),
    TemplateError(String),
}
// First, add this enum to watermark/mod.rs or a new types.rs file
#[derive(Clone, Debug)]
pub enum WatermarkSource {
    File(PathBuf),
    Synthetic {
        text: Option<String>,
        width: u32,
        height: u32,
    }
}
impl std::fmt::Display for WatermarkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WatermarkError::InvalidPosition(msg) => write!(f, "Invalid position: {}", msg),
            WatermarkError::InvalidOpacity(msg) => write!(f, "Invalid opacity: {}", msg),
            WatermarkError::TextRenderingError(msg) => write!(f, "Text rendering error: {}", msg),
            WatermarkError::ImageLoadError(msg) => write!(f, "Image load error: {}", msg),
            WatermarkError::BlendingError(msg) => write!(f, "Blending error: {}", msg),
            WatermarkError::FontError(msg) => write!(f, "Font error: {}", msg),
            WatermarkError::TemplateError(msg) => write!(f, "Template error: {}", msg),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WatermarkOptions {
    pub position: WatermarkPosition,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub rotation: Option<f32>,
    pub scale: Option<f32>,
    pub padding: Option<u32>,
    pub repeat: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WatermarkTemplate {
    pub name: String,
    pub watermark_type: WatermarkType,
    pub data: WatermarkData,
    pub options: WatermarkOptions,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WatermarkType {
    Text {
        font_name: String,
        font_size: f32,
        color: [u8; 4],
    },
    Image {
        preserve_aspect_ratio: bool,
        scale: Option<f32>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WatermarkData {
    Text(String),
    ImagePath(PathBuf),
}

impl Default for WatermarkOptions {
    fn default() -> Self {
        Self {
            position: WatermarkPosition::BottomRight(Position::new(20, 20)),
            opacity: 0.8,
            blend_mode: BlendMode::Normal,
            rotation: None,
            scale: None,
            padding: Some(10),
            repeat: false,
        }
    }
}

// Base trait for watermark functionality
pub trait Watermark {
    fn apply(&self, image: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, options: &WatermarkOptions) 
        -> Result<(), WatermarkError>;
    
    fn remove(&self, image: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, area: &WatermarkPosition) 
        -> Result<(), WatermarkError>;
}