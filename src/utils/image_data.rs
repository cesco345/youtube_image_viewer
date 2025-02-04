// src/utils/image_data.rs

use fltk::{image::RgbImage, prelude::ImageExt};

pub struct ImageData {
    image: RgbImage,
}

impl ImageData {
    pub fn new(image: RgbImage) -> Self {
        Self { image }
    }

    pub fn get_intensity(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 || x >= self.image.data_w() || y >= self.image.data_h() {
            return 0;
        }
        
        let data = self.image.to_rgb_data();
        let idx = (y * self.image.data_w() + x) as usize * 3;
        
        if idx + 2 >= data.len() {
            return 0;
        }
        
        // Calculate intensity as average of RGB
        ((data[idx] as u16 + data[idx + 1] as u16 + data[idx + 2] as u16) / 3) as u8
    }

    pub fn get_image(&self) -> &RgbImage {
        &self.image
    }

    pub fn width(&self) -> i32 {
        self.image.data_w()
    }

    pub fn height(&self) -> i32 {
        self.image.data_h()
    }
}