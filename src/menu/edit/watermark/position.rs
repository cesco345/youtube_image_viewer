// menu/edit/watermark/position.rs

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WatermarkPosition {
    TopLeft(Position),
    TopRight(Position),
    BottomLeft(Position),
    BottomRight(Position),
    Center(Position),
    Custom(Position),
}

pub fn calculate_position(
    image_width: u32,
    image_height: u32,
    watermark_width: u32,
    watermark_height: u32,
    position: &WatermarkPosition,
    padding: u32,
) -> (u32, u32) {
    match position {
        WatermarkPosition::TopLeft(pos) => (
            pos.x + padding,
            pos.y + padding,
        ),
        WatermarkPosition::TopRight(pos) => (
            image_width.saturating_sub(watermark_width + pos.x + padding),
            pos.y + padding,
        ),
        WatermarkPosition::BottomLeft(pos) => (
            pos.x + padding,
            image_height.saturating_sub(watermark_height + pos.y + padding),
        ),
        WatermarkPosition::BottomRight(pos) => (
            image_width.saturating_sub(watermark_width + pos.x + padding),
            image_height.saturating_sub(watermark_height + pos.y + padding),
        ),
        WatermarkPosition::Center(_) => (
            (image_width.saturating_sub(watermark_width)) / 2,
            (image_height.saturating_sub(watermark_height)) / 2,
        ),
        WatermarkPosition::Custom(pos) => (
            pos.x.min(image_width.saturating_sub(watermark_width)),
            pos.y.min(image_height.saturating_sub(watermark_height)),
        ),
    }
}

/// Helper function to check if a position is within image bounds
pub fn is_position_valid(
    image_width: u32,
    image_height: u32,
    watermark_width: u32,
    watermark_height: u32,
    position: &WatermarkPosition,
    padding: u32,
) -> bool {
    let (x, y) = calculate_position(
        image_width,
        image_height,
        watermark_width,
        watermark_height,
        position,
        padding,
    );

    x + watermark_width <= image_width && y + watermark_height <= image_height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_position_top_left() {
        let pos = calculate_position(
            1000,
            1000,
            100,
            100,
            &WatermarkPosition::TopLeft(Position::new(10, 10)),
            5,
        );
        assert_eq!(pos, (15, 15));
    }

    #[test]
    fn test_calculate_position_bottom_right() {
        let pos = calculate_position(
            1000,
            1000,
            100,
            100,
            &WatermarkPosition::BottomRight(Position::new(10, 10)),
            5,
        );
        assert_eq!(pos, (885, 885));
    }

    #[test]
    fn test_calculate_position_center() {
        let pos = calculate_position(
            1000,
            1000,
            100,
            100,
            &WatermarkPosition::Center(Position::new(0, 0)),
            0,
        );
        assert_eq!(pos, (450, 450));
    }
}