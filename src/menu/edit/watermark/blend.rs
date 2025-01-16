// src/menu/edit/watermark/blend.rs

use image::Rgba;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    SoftLight,
    HardLight,
    Difference,
}

pub trait WatermarkBlend {
    fn blend_pixel(base: Rgba<u8>, overlay: Rgba<u8>, opacity: f32, mode: BlendMode) -> Rgba<u8>;
}

impl WatermarkBlend for Rgba<u8> {
    fn blend_pixel(base: Rgba<u8>, overlay: Rgba<u8>, opacity: f32, mode: BlendMode) -> Rgba<u8> {
        let opacity = opacity.clamp(0.0, 1.0);
        let overlay_opacity = (overlay[3] as f32 / 255.0) * opacity;

        let blended = match mode {
            BlendMode::Normal => overlay,
            BlendMode::Multiply => multiply_blend(base, overlay),
            BlendMode::Screen => screen_blend(base, overlay),
            BlendMode::Overlay => overlay_blend(base, overlay),
            BlendMode::SoftLight => soft_light_blend(base, overlay),
            BlendMode::HardLight => hard_light_blend(base, overlay),
            BlendMode::Difference => difference_blend(base, overlay),
        };

        blend_with_opacity(base, blended, overlay_opacity)
    }
}

fn multiply_blend(base: Rgba<u8>, overlay: Rgba<u8>) -> Rgba<u8> {
    Rgba([
        ((base[0] as f32 * overlay[0] as f32) / 255.0) as u8,
        ((base[1] as f32 * overlay[1] as f32) / 255.0) as u8,
        ((base[2] as f32 * overlay[2] as f32) / 255.0) as u8,
        overlay[3],
    ])
}

fn screen_blend(base: Rgba<u8>, overlay: Rgba<u8>) -> Rgba<u8> {
    Rgba([
        255 - ((255 - base[0] as u32) * (255 - overlay[0] as u32) / 255) as u8,
        255 - ((255 - base[1] as u32) * (255 - overlay[1] as u32) / 255) as u8,
        255 - ((255 - base[2] as u32) * (255 - overlay[2] as u32) / 255) as u8,
        overlay[3],
    ])
}

fn overlay_blend(base: Rgba<u8>, overlay: Rgba<u8>) -> Rgba<u8> {
    Rgba([
        blend_overlay_channel(base[0], overlay[0]),
        blend_overlay_channel(base[1], overlay[1]),
        blend_overlay_channel(base[2], overlay[2]),
        overlay[3],
    ])
}

fn soft_light_blend(base: Rgba<u8>, overlay: Rgba<u8>) -> Rgba<u8> {
    Rgba([
        blend_soft_light_channel(base[0], overlay[0]),
        blend_soft_light_channel(base[1], overlay[1]),
        blend_soft_light_channel(base[2], overlay[2]),
        overlay[3],
    ])
}

fn hard_light_blend(base: Rgba<u8>, overlay: Rgba<u8>) -> Rgba<u8> {
    Rgba([
        blend_hard_light_channel(base[0], overlay[0]),
        blend_hard_light_channel(base[1], overlay[1]),
        blend_hard_light_channel(base[2], overlay[2]),
        overlay[3],
    ])
}

fn difference_blend(base: Rgba<u8>, overlay: Rgba<u8>) -> Rgba<u8> {
    Rgba([
        (base[0] as i16 - overlay[0] as i16).abs() as u8,
        (base[1] as i16 - overlay[1] as i16).abs() as u8,
        (base[2] as i16 - overlay[2] as i16).abs() as u8,
        overlay[3],
    ])
}

fn blend_with_opacity(base: Rgba<u8>, blended: Rgba<u8>, opacity: f32) -> Rgba<u8> {
    Rgba([
        lerp(base[0], blended[0], opacity) as u8,
        lerp(base[1], blended[1], opacity) as u8,
        lerp(base[2], blended[2], opacity) as u8,
        lerp(base[3], blended[3], opacity) as u8,
    ])
}

fn lerp(a: u8, b: u8, t: f32) -> f32 {
    a as f32 * (1.0 - t) + b as f32 * t
}

fn blend_overlay_channel(base: u8, overlay: u8) -> u8 {
    let base_f = base as f32 / 255.0;
    let overlay_f = overlay as f32 / 255.0;

    let result = if base_f <= 0.5 {
        2.0 * base_f * overlay_f
    } else {
        1.0 - 2.0 * (1.0 - base_f) * (1.0 - overlay_f)
    };

    (result * 255.0).clamp(0.0, 255.0) as u8
}

fn blend_soft_light_channel(base: u8, overlay: u8) -> u8 {
    let base_f = base as f32 / 255.0;
    let overlay_f = overlay as f32 / 255.0;

    let result = if overlay_f <= 0.5 {
        base_f - (1.0 - 2.0 * overlay_f) * base_f * (1.0 - base_f)
    } else {
        base_f + (2.0 * overlay_f - 1.0) * (if base_f <= 0.25 {
            ((16.0 * base_f - 12.0) * base_f + 4.0) * base_f
        } else {
            base_f.sqrt()
        } - base_f)
    };

    (result * 255.0).clamp(0.0, 255.0) as u8
}

fn blend_hard_light_channel(base: u8, overlay: u8) -> u8 {
    blend_overlay_channel(overlay, base)
}