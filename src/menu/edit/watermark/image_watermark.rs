// menu/edit/watermark/image_watermark.rs

use super::{
    blend::WatermarkBlend,
    position::calculate_position,
    Watermark, WatermarkError, WatermarkOptions,
};
use image::{
    DynamicImage, GenericImageView, ImageBuffer, Rgba,
    imageops::{resize, FilterType},
};
use std::path::Path;

#[derive(Clone)]
pub struct ImageWatermark {
    watermark_image: DynamicImage,
    original_size: (u32, u32),
}

impl ImageWatermark {
    pub fn new(watermark_image: DynamicImage) -> Self {
        let original_size = watermark_image.dimensions();
        Self {
            watermark_image,
            original_size,
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, WatermarkError> {
        image::open(path)
            .map_err(|e| WatermarkError::ImageLoadError(e.to_string()))
            .map(Self::new)
    }

    fn resize_watermark(
        &self,
        target_width: u32,
        target_height: u32,
        scale: Option<f32>,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let (width, height) = if let Some(scale) = scale {
            (
                (self.original_size.0 as f32 * scale) as u32,
                (self.original_size.1 as f32 * scale) as u32,
            )
        } else {
            (target_width, target_height)
        };

        resize(
            &self.watermark_image.to_rgba8(),
            width,
            height,
            FilterType::Lanczos3,
        )
    }

    fn apply_rotation(
        &self,
        image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
        angle: f32,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        use std::f32::consts::PI;
        
        // Convert angle to radians
        let angle_rad = angle * PI / 180.0;
        
        // Calculate the dimensions of the rotated image
        let (width, height) = image.dimensions();
        let cos_angle = angle_rad.cos().abs();
        let sin_angle = angle_rad.sin().abs();
        
        // New dimensions after rotation
        let new_width = (width as f32 * cos_angle + height as f32 * sin_angle).ceil() as u32;
        let new_height = (width as f32 * sin_angle + height as f32 * cos_angle).ceil() as u32;
        
        // Create a new buffer for the rotated image
        let mut rotated = ImageBuffer::new(new_width, new_height);
        
        // Calculate center points
        let center_x = width as f32 / 2.0;
        let center_y = height as f32 / 2.0;
        let new_center_x = new_width as f32 / 2.0;
        let new_center_y = new_height as f32 / 2.0;
        
        // Rotation matrix
        let cos = angle_rad.cos();
        let sin = angle_rad.sin();
        
        for y in 0..new_height {
            for x in 0..new_width {
                // Translate to origin
                let dx = x as f32 - new_center_x;
                let dy = y as f32 - new_center_y;
                
                // Apply rotation matrix
                let orig_x = (dx * cos - dy * sin + center_x).round();
                let orig_y = (dx * sin + dy * cos + center_y).round();
                
                // Check if the original pixel is within bounds
                if orig_x >= 0.0 && orig_x < width as f32 && 
                   orig_y >= 0.0 && orig_y < height as f32 
                {
                    let orig_x = orig_x as u32;
                    let orig_y = orig_y as u32;
                    
                    // Copy pixel from original image
                    let pixel = image.get_pixel(orig_x, orig_y);
                    rotated.put_pixel(x, y, *pixel);
                } else {
                    // Set transparent pixel for out-of-bounds areas
                    rotated.put_pixel(x, y, Rgba([0, 0, 0, 0]));
                }
            }
        }
        
        rotated
    }
}



impl Watermark for ImageWatermark {
    fn apply(
        &self,
        image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        options: &WatermarkOptions,
    ) -> Result<(), WatermarkError> {
        // Calculate dimensions for the watermark
        let watermark_size = if let Some(scale) = options.scale {
            (
                (self.original_size.0 as f32 * scale) as u32,
                (self.original_size.1 as f32 * scale) as u32,
            )
        } else {
            // Default to a reasonable size relative to the main image
            let max_width = image.width() / 4;
            let scale = max_width as f32 / self.original_size.0 as f32;
            (
                max_width,
                (self.original_size.1 as f32 * scale) as u32,
            )
        };

        // Resize the watermark image
        let mut watermark = self.resize_watermark(
            watermark_size.0,
            watermark_size.1,
            options.scale,
        );

        // Apply rotation if specified
        if let Some(rotation) = options.rotation {
            watermark = self.apply_rotation(&watermark, rotation);
        }

        // Calculate position
        let (x, y) = calculate_position(
            image.width(),
            image.height(),
            watermark.width(),
            watermark.height(),
            &options.position,
            options.padding.unwrap_or(0),
        );

        // Apply the watermark with blending
        if options.repeat {
            // Apply repeating pattern
            for offset_y in (0..image.height()).step_by(watermark.height() as usize + options.padding.unwrap_or(0) as usize) {
                for offset_x in (0..image.width()).step_by(watermark.width() as usize + options.padding.unwrap_or(0) as usize) {
                    self.apply_single_watermark(image, &watermark, offset_x, offset_y, options)?;
                }
            }
        } else {
            // Apply single watermark
            self.apply_single_watermark(image, &watermark, x, y, options)?;
        }

        Ok(())
    }

    fn remove(
        &self,
        image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        watermark_area: &super::WatermarkPosition,
    ) -> Result<(), WatermarkError> {
        // For now, implement a simple removal by filling with average color
        // TODO: Implement more sophisticated removal (e.g., content-aware fill)
        let (x, y, width, height) = match watermark_area {
            // Calculate the area to clear based on position
            super::WatermarkPosition::Custom(pos) => (pos.x, pos.y, 100, 100), // Default size
            _ => (0, 0, 100, 100), // Default fallback
        };

        // Calculate average color of surrounding pixels
        let mut avg_r = 0u32;
        let mut avg_g = 0u32;
        let mut avg_b = 0u32;
        let mut count = 0u32;

        // Sample pixels around the watermark area
        for sample_y in y.saturating_sub(5)..=y + height + 5 {
            for sample_x in x.saturating_sub(5)..=x + width + 5 {
                if sample_x < image.width() && sample_y < image.height() {
                    let pixel = image.get_pixel(sample_x, sample_y);
                    avg_r += pixel[0] as u32;
                    avg_g += pixel[1] as u32;
                    avg_b += pixel[2] as u32;
                    count += 1;
                }
            }
        }

        if count > 0 {
            let avg_color = Rgba([
                (avg_r / count) as u8,
                (avg_g / count) as u8,
                (avg_b / count) as u8,
                255,
            ]);

            // Fill the area with average color
            for img_y in y..y + height {
                for img_x in x..x + width {
                    if img_x < image.width() && img_y < image.height() {
                        image.put_pixel(img_x, img_y, avg_color);
                    }
                }
            }
        }

        Ok(())
    }
}

impl ImageWatermark {
    fn apply_single_watermark(
        &self,
        image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        watermark: &ImageBuffer<Rgba<u8>, Vec<u8>>,
        x: u32,
        y: u32,
        options: &WatermarkOptions,
    ) -> Result<(), WatermarkError> {
        for (wx, wy, watermark_pixel) in watermark.enumerate_pixels() {
            let image_x = x + wx;
            let image_y = y + wy;

            if image_x < image.width() && image_y < image.height() && watermark_pixel[3] > 0 {
                let base_pixel = *image.get_pixel(image_x, image_y);
                let blended_pixel = <Rgba<u8> as WatermarkBlend>::blend_pixel(
                    base_pixel,
                    *watermark_pixel,
                    options.opacity,
                    options.blend_mode,
                );
                image.put_pixel(image_x, image_y, blended_pixel);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{RgbaImage};

    #[test]
    fn test_create_image_watermark() {
        let test_image = RgbaImage::new(100, 100);
        let watermark = ImageWatermark::new(DynamicImage::ImageRgba8(test_image));
        assert_eq!(watermark.original_size, (100, 100));
    }

    #[test]
    fn test_resize_watermark() {
        let test_image = RgbaImage::new(100, 100);
        let watermark = ImageWatermark::new(DynamicImage::ImageRgba8(test_image));
        let resized = watermark.resize_watermark(50, 50, None);
        assert_eq!(resized.dimensions(), (50, 50));
    }
}