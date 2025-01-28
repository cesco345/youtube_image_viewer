// src/scientific/layers/channel.rs
use fltk::image::RgbImage;
use crate::scientific::layers::metadata::Metadata;

#[derive(Clone)]
pub struct Channel {
    pub name: String,
    pub image: RgbImage,
    pub wavelength: f32,
    pub pseudo_color: (u8, u8, u8),
    pub opacity: f32,
    pub visible: bool,
    pub metadata: Metadata,
}

impl Channel {
    pub fn new(name: String, image: RgbImage, wavelength: f32, pseudo_color: (u8, u8, u8)) -> Self {
        Self {
            name,
            image,
            wavelength,
            pseudo_color,
            opacity: 1.0,
            visible: true,
            metadata: Metadata::default(),
        }
    }
}