use std::sync::{Arc, Mutex};
use fltk::image::RgbImage;
use fltk::prelude::ImageExt;
use image::{ImageBuffer, Rgba};
use crate::menu::edit::watermark::{
    image_watermark::ImageWatermark,
    text_watermark::TextWatermark,
    Watermark,
    WatermarkOptions,
    WatermarkError as WatermarkModuleError
};

#[derive(Clone, Debug)]
pub struct WatermarkError {
    pub message: String,
}

impl From<WatermarkModuleError> for WatermarkError {
    fn from(error: WatermarkModuleError) -> Self {
        WatermarkError {
            message: error.to_string()
        }
    }
}

#[derive(Clone)]
pub struct WatermarkTemplate {
    pub name: String,
    pub options: WatermarkOptions,
}

#[derive(Clone)]
pub struct TemplateManager {}

impl TemplateManager {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn list_templates(&self) -> Vec<String> {
        vec![]
    }
}

#[derive(Clone)]
enum WatermarkContent {
    Image(ImageWatermark),
    Text(TextWatermark),
}

#[derive(Clone)]
pub struct WatermarkState {
    template_manager: Arc<Mutex<TemplateManager>>,
    current_template: Option<WatermarkTemplate>,
    pub current_options: WatermarkOptions,
    is_preview_active: bool,
    watermark_content: Option<WatermarkContent>,
}

impl WatermarkState {
    pub fn new() -> Self {
        Self {
            template_manager: Arc::new(Mutex::new(TemplateManager::new())),
            current_template: None,
            current_options: WatermarkOptions::default(),
            is_preview_active: false,
            watermark_content: None,
        }
    }

    fn fltk_to_image_buffer(fltk_image: &RgbImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let data = fltk_image.to_rgb_data();
        let width = fltk_image.data_w() as u32;
        let height = fltk_image.data_h() as u32;

        let mut rgba_data = Vec::with_capacity((width * height * 4) as usize);
        
        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 3) as usize;
                if idx + 2 < data.len() {
                    rgba_data.push(data[idx]);     // R
                    rgba_data.push(data[idx + 1]); // G
                    rgba_data.push(data[idx + 2]); // B
                    rgba_data.push(255);           // A
                }
            }
        }

        ImageBuffer::from_raw(width, height, rgba_data)
            .unwrap_or_else(|| {
                println!("Failed to create image buffer, creating empty buffer");
                ImageBuffer::new(width, height)
            })
    }

    fn image_buffer_to_fltk(buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Option<RgbImage> {
        let width = buffer.width() as i32;
        let height = buffer.height() as i32;
        let mut rgb_data = Vec::with_capacity((width * height * 3) as usize);

        for pixel in buffer.pixels() {
            let alpha = pixel[3] as f32 / 255.0;
            rgb_data.push(((pixel[0] as f32 * alpha) + (255.0 * (1.0 - alpha))) as u8);
            rgb_data.push(((pixel[1] as f32 * alpha) + (255.0 * (1.0 - alpha))) as u8);
            rgb_data.push(((pixel[2] as f32 * alpha) + (255.0 * (1.0 - alpha))) as u8);
        }

        RgbImage::new(&rgb_data, width, height, fltk::enums::ColorDepth::Rgb8).ok()
    }

    pub fn set_watermark(&mut self, watermark: ImageWatermark) {
        self.watermark_content = Some(WatermarkContent::Image(watermark));
    }

    pub fn set_text_watermark(&mut self, watermark: TextWatermark) {
        self.watermark_content = Some(WatermarkContent::Text(watermark));
    }

    pub fn set_options(&mut self, options: WatermarkOptions) {
        self.current_options = options;
    }

    pub fn apply_watermark(&mut self, image: &RgbImage) -> Result<Option<RgbImage>, WatermarkError> {
        if let Some(content) = &self.watermark_content {
            let mut image_buffer = Self::fltk_to_image_buffer(image);
            
            match content {
                WatermarkContent::Image(watermark) => {
                    watermark.apply(&mut image_buffer, &self.current_options)?;
                },
                WatermarkContent::Text(watermark) => {
                    watermark.apply(&mut image_buffer, &self.current_options)?;
                }
            }
            
            Ok(Self::image_buffer_to_fltk(&image_buffer))
        } else {
            Ok(Some(image.clone()))
        }
    }

    pub fn get_current_template(&self) -> Option<WatermarkTemplate> {
        self.current_template.clone()
    }

    pub fn get_current_options(&self) -> WatermarkOptions {
        self.current_options.clone()
    }

    pub fn is_preview_active(&self) -> bool {
        self.is_preview_active
    }

    pub fn list_templates(&self) -> Result<Vec<String>, WatermarkError> {
        Ok(self.template_manager
            .lock()
            .map_err(|_| WatermarkError { message: "Failed to lock template manager".to_string() })?
            .list_templates())
    }

    pub fn toggle_preview(&mut self) {
        self.is_preview_active = !self.is_preview_active;
    }
}

impl Default for WatermarkState {
    fn default() -> Self {
        Self::new()
    }
}