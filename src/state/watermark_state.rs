use std::sync::{Arc, Mutex};
use fltk::image::RgbImage;
use fltk::prelude::ImageExt;
use image::{ImageBuffer, Rgba};
use crate::menu::edit::watermark::RemovalArea;
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

    pub fn remove_watermark_area(&self, image: &RgbImage, area: &RemovalArea) -> Result<Option<RgbImage>, WatermarkError> {
        let mut image_buffer = Self::fltk_to_image_buffer(image);
        
        // Ensure coordinates are within bounds
        let x = area.x.clamp(0, image_buffer.width() as i32 - 1);
        let y = area.y.clamp(0, image_buffer.height() as i32 - 1);
        let width = area.width.min((image_buffer.width() as i32 - x) as i32);
        let height = area.height.min((image_buffer.height() as i32 - y) as i32);
    
        if width <= 0 || height <= 0 {
            return Ok(Some(image.clone()));
        }
    
        // Increase sample size for better color matching
        let sample_size = 15; // Increased from 5
        let mut samples = Vec::new();
    
        // Sample from a larger area around the watermark
        for sample_y in (y - sample_size).max(0)..=(y + height + sample_size).min(image_buffer.height() as i32 - 1) {
            for sample_x in (x - sample_size).max(0)..=(x + width + sample_size).min(image_buffer.width() as i32 - 1) {
                // Only sample from outside the removal area
                if sample_x < x || sample_x >= x + width || sample_y < y || sample_y >= y + height {
                    samples.push((
                        sample_x,
                        sample_y,
                        image_buffer.get_pixel(sample_x as u32, sample_y as u32).clone()
                    ));
                }
            }
        }
    
        if samples.is_empty() {
            return Ok(Some(image.clone()));
        }
    
        // Process the selected area
        for cy in y..y + height {
            for cx in x..x + width {
                if cx >= 0 && cx < image_buffer.width() as i32 && 
                   cy >= 0 && cy < image_buffer.height() as i32 {
                    let mut total_weight = 0.0;
                    let mut new_color = [0.0f32; 3];
                    let current = image_buffer.get_pixel(cx as u32, cy as u32);
    
                    // Weight samples by both distance and color similarity
                    for (sample_x, sample_y, sample) in &samples {
                        // Calculate spatial distance
                        let dx = cx - sample_x;
                        let dy = cy - sample_y;
                        let dist = ((dx * dx + dy * dy) as f32).sqrt();
                        
                        // Calculate color similarity
                        let color_diff = (0..3)
                            .map(|i| (current[i] as f32 - sample[i] as f32).abs())
                            .sum::<f32>() / 3.0;
                        
                        // Combined weight based on distance and color similarity
                        let dist_weight = 1.0 / (1.0 + dist * 0.1); // Distance factor
                        let color_weight = 1.0 / (1.0 + color_diff * 0.1); // Color similarity factor
                        let weight = dist_weight * color_weight;
    
                        for i in 0..3 {
                            new_color[i] += sample[i] as f32 * weight;
                        }
                        total_weight += weight;
                    }
    
                    // Calculate the blend factor based on position within the removal area
                    let edge_dist_x = (cx - x).min(x + width - cx) as f32;
                    let edge_dist_y = (cy - y).min(y + height - cy) as f32;
                    let edge_dist = edge_dist_x.min(edge_dist_y) as f32;
                    let blend_factor = (edge_dist / 5.0).min(1.0); // Smoother transition at edges
    
                    // Normalize and apply the new color with edge blending
                    if total_weight > 0.0 {
                        let current_color = image_buffer.get_pixel(cx as u32, cy as u32);
                        let final_color = Rgba([
                            ((new_color[0] / total_weight) * blend_factor +
                             current_color[0] as f32 * (1.0 - blend_factor)) as u8,
                            ((new_color[1] / total_weight) * blend_factor +
                             current_color[1] as f32 * (1.0 - blend_factor)) as u8,
                            ((new_color[2] / total_weight) * blend_factor +
                             current_color[2] as f32 * (1.0 - blend_factor)) as u8,
                            255,
                        ]);
                        
                        image_buffer.put_pixel(cx as u32, cy as u32, final_color);
                    }
                }
            }
        }
    
        // Convert back to FLTK format
        Ok(Self::image_buffer_to_fltk(&image_buffer))
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

    pub fn clear_watermark(&mut self) {
        self.watermark_content = None;
        self.current_template = None;
        self.current_options = WatermarkOptions::default();
        self.is_preview_active = false;
    }

    pub fn get_current_watermark_type(&self) -> Option<&'static str> {
        self.watermark_content.as_ref().map(|content| {
            match content {
                WatermarkContent::Image(_) => "Image",
                WatermarkContent::Text(_) => "Text",
            }
        })
    }

    pub fn has_watermark(&self) -> bool {
        self.watermark_content.is_some()
    }

    pub fn update_watermark_options(&mut self, options: WatermarkOptions) -> Result<(), WatermarkError> {
        // Validate the options before updating
        if options.opacity < 0.0 || options.opacity > 1.0 {
            return Err(WatermarkError { 
                message: "Opacity must be between 0.0 and 1.0".to_string() 
            });
        }

        if let Some(rotation) = options.rotation {
            if rotation < 0.0 || rotation > 360.0 {
                return Err(WatermarkError { 
                    message: "Rotation must be between 0 and 360 degrees".to_string() 
                });
            }
        }

        self.current_options = options;
        Ok(())
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