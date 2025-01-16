// menu/edit/watermark/templates/defaults.rs

use crate::menu::edit::watermark::{
    WatermarkOptions,
    WatermarkType,
    WatermarkData,
    position::{Position, WatermarkPosition},
    blend::BlendMode,
};
use lazy_static::lazy_static;
use std::path::PathBuf;

lazy_static! {
    pub static ref DEFAULT_TEMPLATES: Vec<(String, WatermarkType, WatermarkData, WatermarkOptions)> = {
        let now = chrono::Utc::now();
        vec![
            (
                "default_simple_text".to_string(),
                WatermarkType::Text {
                    font_name: "default".to_string(),
                    font_size: 24.0,
                    color: [0, 0, 0, 255],
                },
                WatermarkData::Text("© Copyright".to_string()),
                WatermarkOptions {
                    position: WatermarkPosition::BottomRight(Position::new(20, 20)),
                    opacity: 0.8,
                    blend_mode: BlendMode::Normal,
                    rotation: None,
                    scale: None,
                    padding: Some(10),
                    repeat: false,
                }
            ),
            (
                "default_diagonal".to_string(),
                WatermarkType::Text {
                    font_name: "default".to_string(),
                    font_size: 36.0,
                    color: [128, 128, 128, 255],
                },
                WatermarkData::Text("CONFIDENTIAL".to_string()),
                WatermarkOptions {
                    position: WatermarkPosition::Center(Position::new(0, 0)),
                    opacity: 0.3,
                    blend_mode: BlendMode::Normal,
                    rotation: Some(45.0),
                    scale: None,
                    padding: None,
                    repeat: true,
                }
            ),
            (
                "default_image".to_string(),
                WatermarkType::Image {
                    preserve_aspect_ratio: true,
                    scale: Some(0.2),
                },
                WatermarkData::ImagePath(PathBuf::from("assets/default_logo.png")),
                WatermarkOptions {
                    position: WatermarkPosition::BottomRight(Position::new(20, 20)),
                    opacity: 0.8,
                    blend_mode: BlendMode::Normal,
                    rotation: None,
                    scale: Some(0.2),
                    padding: Some(10),
                    repeat: false,
                }
            ),
        ]
    };
}