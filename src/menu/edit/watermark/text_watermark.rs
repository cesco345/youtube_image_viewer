// menu/edit/watermark/text_watermark.rs

use super::{
    Watermark, WatermarkError, WatermarkOptions,
    blend::WatermarkBlend,
    fonts::FontManager,
    position::calculate_position,
};
use image::{ImageBuffer, Rgba};
use rusttype::{Scale, point};
use std::sync::Arc;

#[derive(Clone)]
pub struct TextWatermark {
    text: String,
    color: Rgba<u8>,
    font_size: f32,
    font_manager: Arc<FontManager>,
}

impl TextWatermark {
    pub fn new(
        text: String,
        color: Rgba<u8>,
        font_size: f32,
    ) -> Result<Self, WatermarkError> {
        Ok(Self {
            text,
            color,
            font_size,
            font_manager: Arc::new(FontManager::new()?),
        })
    }

    fn render_text(
        &self,
        _width: u32,
        _height: u32,
        options: &WatermarkOptions,
    ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, WatermarkError> {
        let font = self.font_manager.get_default_font()?;
        let scale = Scale::uniform(self.font_size);

        // Calculate text dimensions
        let v_metrics = font.v_metrics(scale);
        let glyphs: Vec<_> = font
            .layout(&self.text, scale, point(0.0, v_metrics.ascent))
            .collect();

        // Calculate text bounds
        let text_width = glyphs
            .iter()
            .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0) as u32;
        let text_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;

        // Create image buffer for text
        let mut text_image = ImageBuffer::new(text_width, text_height);

        // Draw text onto the image buffer
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let x = x as i32 + bounding_box.min.x;
                    let y = y as i32 + bounding_box.min.y;

                    if x >= 0 && x < text_width as i32 && y >= 0 && y < text_height as i32 {
                        let alpha = (v * 255.0) as u8;
                        let pixel = Rgba([
                            self.color[0],
                            self.color[1],
                            self.color[2],
                            ((alpha as f32 * options.opacity) as u8).min(255),
                        ]);
                        text_image.put_pixel(x as u32, y as u32, pixel);
                    }
                });
            }
        }

        Ok(text_image)
    }

    fn apply_rotation(
        &self,
        image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
        _angle: f32,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        // TODO: Implement proper rotation
        image.clone()
    }
}

impl Watermark for TextWatermark {
    fn apply(
        &self,
        image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        options: &WatermarkOptions,
    ) -> Result<(), WatermarkError> {
        let text_image = self.render_text(image.width(), image.height(), options)?;
        let text_image = if let Some(rotation) = options.rotation {
            self.apply_rotation(&text_image, rotation)
        } else {
            text_image
        };

        let (x, y) = calculate_position(
            image.width(),
            image.height(),
            text_image.width(),
            text_image.height(),
            &options.position,
            options.padding.unwrap_or(0),
        );

        // Apply the watermark with blending
        for (text_x, text_y, text_pixel) in text_image.enumerate_pixels() {
            let image_x = x + text_x;
            let image_y = y + text_y;

            if image_x < image.width() && image_y < image.height() && text_pixel[3] > 0 {
                let base_pixel = *image.get_pixel(image_x, image_y);
                let blended_pixel = <Rgba<u8> as WatermarkBlend>::blend_pixel(
                    base_pixel,
                    *text_pixel,
                    options.opacity,
                    options.blend_mode,
                );
                image.put_pixel(image_x, image_y, blended_pixel);
            }
        }

        Ok(())
    }

    fn remove(
        &self,
        image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
        area: &super::position::WatermarkPosition,
    ) -> Result<(), WatermarkError> {
        // For text watermarks, we'll simply clear the area
        let (x, y, width, height) = match area {
            // Default clear area
            _ => (0, 0, 100, 30),
        };

        for img_y in y..y + height {
            for img_x in x..x + width {
                if img_x < image.width() && img_y < image.height() {
                    image.put_pixel(img_x, img_y, Rgba([255, 255, 255, 255]));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_watermark_creation() {
        let watermark = TextWatermark::new(
            "Test".to_string(),
            Rgba([0, 0, 0, 255]),
            24.0,
        );
        assert!(watermark.is_ok());
    }
}