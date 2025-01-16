// menu/edit/watermark/templates/mod.rs

use crate::menu::edit::watermark::{
    WatermarkError, WatermarkOptions, WatermarkType, WatermarkData,
};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

mod defaults;

pub struct TemplateManager {
    templates: HashMap<String, (WatermarkType, WatermarkData, WatermarkOptions)>,
    template_dir: PathBuf,
}

impl TemplateManager {
    pub fn new() -> Self {
        let template_dir = Path::new("templates").to_path_buf();
        let mut manager = Self {
            templates: HashMap::new(),
            template_dir,
        };

        // Load default templates
        for (name, wtype, data, options) in defaults::DEFAULT_TEMPLATES.iter() {
            manager.templates.insert(
                name.clone(),
                (wtype.clone(), data.clone(), options.clone())
            );
        }

        // Create template directory if it doesn't exist
        if !manager.template_dir.exists() {
            if let Err(e) = fs::create_dir_all(&manager.template_dir) {
                eprintln!("Failed to create template directory: {:?}", e);
            }
        }

        // Load existing templates from disk
        if let Err(e) = manager.load_templates_from_disk() {
            eprintln!("Failed to load templates from disk: {:?}", e);
        }

        manager
    }

    pub fn save_template(
        &mut self,
        name: String,
        wtype: WatermarkType,
        data: WatermarkData,
        options: WatermarkOptions,
    ) -> Result<(), WatermarkError> {
        self.templates.insert(name.clone(), (wtype, data, options));
        self.save_template_to_disk(&name)
    }

    pub fn load_template(&self, name: &str) 
        -> Result<(WatermarkType, WatermarkData, WatermarkOptions), WatermarkError> 
    {
        self.templates
            .get(name)
            .cloned()
            .ok_or_else(|| WatermarkError::TemplateError(format!("Template '{}' not found", name)))
    }

    pub fn list_templates(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }

    fn load_templates_from_disk(&mut self) -> Result<(), WatermarkError> {
        if !self.template_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&self.template_dir)
            .map_err(|e| WatermarkError::TemplateError(format!("Failed to read template directory: {}", e)))? 
        {
            let entry = entry
                .map_err(|e| WatermarkError::TemplateError(format!("Failed to read entry: {}", e)))?;
            
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                let file = fs::File::open(entry.path())
                    .map_err(|e| WatermarkError::TemplateError(format!("Failed to open file: {}", e)))?;
                
                let template: (String, WatermarkType, WatermarkData, WatermarkOptions) = 
                    serde_json::from_reader(file)
                    .map_err(|e| WatermarkError::TemplateError(format!("Failed to parse template: {}", e)))?;
                
                self.templates.insert(
                    template.0.clone(),
                    (template.1, template.2, template.3)
                );
            }
        }

        Ok(())
    }

    fn save_template_to_disk(&self, name: &str) -> Result<(), WatermarkError> {
        if let Some(template) = self.templates.get(name) {
            let template_path = self.template_dir.join(format!("{}.json", name));
            
            let template_json = serde_json::to_string_pretty(&(
                name,
                &template.0,
                &template.1,
                &template.2
            ))
            .map_err(|e| WatermarkError::TemplateError(format!("Failed to serialize template: {}", e)))?;

            fs::write(&template_path, template_json)
                .map_err(|e| WatermarkError::TemplateError(format!("Failed to write template: {}", e)))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_manager_creation() {
        let manager = TemplateManager::new();
        assert!(!manager.templates.is_empty());
    }

    #[test]
    fn test_list_templates() {
        let manager = TemplateManager::new();
        let templates = manager.list_templates();
        assert!(!templates.is_empty());
        assert!(templates.contains(&"default_simple_text".to_string()));
    }
}