// utils/blend_utils.rs

use image::Rgba;
use crate::menu::edit::watermark::BlendMode;

pub fn ensure_valid_opacity(opacity: f32) -> f32 {
    opacity.clamp(0.0, 1.0)
}

pub fn calculate_alpha(base_alpha: u8, overlay_alpha: u8, opacity: f32) -> u8 {
    let base = base_alpha as f32 / 255.0;
    let overlay = overlay_alpha as f32 / 255.0;
    let result = base + (overlay * opacity) * (1.0 - base);
    (result * 255.0).clamp(0.0, 255.0) as u8
}

pub fn get_recommended_blend_mode(background_brightness: f32) -> BlendMode {
    match background_brightness {
        b if b < 0.3 => BlendMode::Screen,    // Light blend for dark backgrounds
        b if b > 0.7 => BlendMode::Multiply,  // Dark blend for light backgrounds
        _ => BlendMode::Overlay,              // Balanced blend for medium backgrounds
    }
}

pub fn calculate_brightness(color: Rgba<u8>) -> f32 {
    // Using perceived brightness formula
    (0.299 * color[0] as f32 + 0.587 * color[1] as f32 + 0.114 * color[2] as f32) / 255.0
}

pub fn sample_background_color(
    image: &image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    x: u32,
    y: u32,
    sample_size: u32,
) -> Rgba<u8> {
    let mut r = 0u32;
    let mut g = 0u32;
    let mut b = 0u32;
    let mut a = 0u32;
    let mut count = 0u32;

    let start_x = x.saturating_sub(sample_size / 2);
    let start_y = y.saturating_sub(sample_size / 2);
    let end_x = (x + sample_size / 2).min(image.width());
    let end_y = (y + sample_size / 2).min(image.height());

    for sample_y in start_y..end_y {
        for sample_x in start_x..end_x {
            let pixel = image.get_pixel(sample_x, sample_y);
            r += pixel[0] as u32;
            g += pixel[1] as u32;
            b += pixel[2] as u32;
            a += pixel[3] as u32;
            count += 1;
        }
    }

    if count > 0 {
        Rgba([
            (r / count) as u8,
            (g / count) as u8,
            (b / count) as u8,
            (a / count) as u8,
        ])
    } else {
        Rgba([0, 0, 0, 0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opacity_clamping() {
        assert_eq!(ensure_valid_opacity(1.5), 1.0);
        assert_eq!(ensure_valid_opacity(-0.5), 0.0);
        assert_eq!(ensure_valid_opacity(0.5), 0.5);
    }

    #[test]
    fn test_alpha_calculation() {
        assert_eq!(calculate_alpha(255, 255, 1.0), 255);
        assert_eq!(calculate_alpha(0, 255, 0.5), 127);
    }

    #[test]
    fn test_brightness_calculation() {
        let white = Rgba([255, 255, 255, 255]);
        let black = Rgba([0, 0, 0, 255]);
        assert!((calculate_brightness(white) - 1.0).abs() < 0.001);
        assert!((calculate_brightness(black) - 0.0).abs() < 0.001);
    }
}