// menu/edit/watermark/fonts/mod.rs

use rusttype::Font;
use std::sync::Arc;
use std::collections::HashMap;
use super::WatermarkError;

const DEFAULT_FONT_BYTES: &[u8] = include_bytes!("assets/kenyan.otf");

struct StaticFontData {
    data: Vec<u8>,
    font: Arc<Font<'static>>,
}

impl StaticFontData {
    fn new(data: Vec<u8>) -> Result<Self, WatermarkError> {
        // Convert the data to a static slice
        let static_data = Box::leak(data.clone().into_boxed_slice());
        
        // Create a font from the static data
        let font = Font::try_from_bytes(static_data)
            .ok_or_else(|| WatermarkError::FontError("Invalid font data".to_string()))?;

        Ok(Self {
            data,
            font: Arc::new(font),
        })
    }
}

pub struct FontManager {
    fonts: HashMap<String, StaticFontData>,
    default_font: Arc<Font<'static>>,
}

impl FontManager {
    pub fn new() -> Result<Self, WatermarkError> {
        // Create the default font from the embedded bytes
        let default_font = Font::try_from_bytes(DEFAULT_FONT_BYTES)
            .ok_or_else(|| WatermarkError::FontError("Failed to load default font".to_string()))?;
        
        let default_font = Arc::new(default_font);
        let mut fonts = HashMap::new();
        
        // Store default font data
        let default_data = StaticFontData::new(DEFAULT_FONT_BYTES.to_vec())?;
        fonts.insert("default".to_string(), default_data);

        Ok(Self {
            fonts,
            default_font,
        })
    }

    pub fn add_font(&mut self, name: String, font_data: Vec<u8>) -> Result<(), WatermarkError> {
        let font_data = StaticFontData::new(font_data)?;
        self.fonts.insert(name, font_data);
        Ok(())
    }

    pub fn get_font(&self, name: &str) -> Option<Arc<Font<'static>>> {
        self.fonts.get(name).map(|data| data.font.clone())
    }

    pub fn get_default_font(&self) -> Result<Arc<Font<'static>>, WatermarkError> {
        Ok(self.default_font.clone())
    }

    pub fn list_fonts(&self) -> Vec<String> {
        self.fonts.keys().cloned().collect()
    }

    pub fn remove_font(&mut self, name: &str) -> Result<(), WatermarkError> {
        if name == "default" {
            return Err(WatermarkError::FontError(
                "Cannot remove default font".to_string(),
            ));
        }
        self.fonts.remove(name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_manager_creation() {
        let font_manager = FontManager::new();
        assert!(font_manager.is_ok());
    }

    #[test]
    fn test_default_font_available() {
        let font_manager = FontManager::new().unwrap();
        assert!(font_manager.get_default_font().is_ok());
    }

    #[test]
    fn test_list_fonts() {
        let font_manager = FontManager::new().unwrap();
        let fonts = font_manager.list_fonts();
        assert!(fonts.contains(&"default".to_string()));
    }

    #[test]
    fn test_add_font() {
        let mut font_manager = FontManager::new().unwrap();
        let test_font_data = DEFAULT_FONT_BYTES.to_vec();
        assert!(font_manager.add_font("test".to_string(), test_font_data).is_ok());
        assert!(font_manager.list_fonts().contains(&"test".to_string()));
    }
}