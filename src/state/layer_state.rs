use fltk::{prelude::*, image::RgbImage, enums::ColorDepth};
use crate::menu::edit::crop::crop_tool::CropSelection;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Layer {
    pub name: String,
    pub image: RgbImage,
    pub opacity: f32,
    pub visible: bool,
    pub color: (u8, u8, u8),
    pub region: Option<CropSelection>,
    pub group_id: Option<usize>,
}

#[derive(Clone)]
pub struct LayerGroup {
    pub name: String,
    pub color: (u8, u8, u8),
    pub visible: bool,
    pub layer_indices: Vec<usize>,
}

#[derive(Clone)]
pub struct LayerState {
    layers: Vec<Layer>,
    groups: Vec<LayerGroup>,
    active_layer: Option<usize>,
    original_image: Option<RgbImage>,
    is_preview_active: bool,
}

impl LayerState {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            groups: Vec::new(),
            active_layer: None,
            original_image: None,
            is_preview_active: false,
        }
    }

    pub fn toggle_preview(&mut self) {
        self.is_preview_active = !self.is_preview_active;
        println!("Preview toggled: {}", self.is_preview_active);
    }

    pub fn is_preview_active(&self) -> bool {
        self.is_preview_active
    }

    pub fn set_original_image(&mut self, image: RgbImage) {
        println!("Setting original image");
        self.original_image = Some(image);
    }

    pub fn add_layer(&mut self, color: (u8, u8, u8), region: CropSelection) -> usize {
        let id = self.layers.len();
        println!("Adding new layer {} with color RGB({}, {}, {})", id, color.0, color.1, color.2);
        
        let (x, y, w, h) = region.get_image_dimensions();
        println!("Creating layer with region: x={}, y={}, w={}, h={}", x, y, w, h);
        
        // Skip validation to maintain original coordinates
        let layer = Layer {
            name: format!("Layer {}", id + 1),
            image: self.original_image.as_ref().expect("Original image must be set").clone(),
            opacity: 0.8,
            visible: true,
            color,
            region: Some(region),
            group_id: None,
        };
        
        self.layers.push(layer);
        self.active_layer = Some(id);
        self.update_groups();
        println!("Layer {} added successfully", id);
        id
    }

    pub fn update_groups(&mut self) {
        self.groups.clear();
        let mut color_map: HashMap<(u8, u8, u8), Vec<usize>> = HashMap::new();
        
        // Group layers by color
        for (idx, layer) in self.layers.iter().enumerate() {
            color_map.entry(layer.color)
                .or_insert_with(Vec::new)
                .push(idx);
        }

        // Create groups
        for (color, indices) in color_map {
            let group_id = self.groups.len();
            let group = LayerGroup {
                name: format!("Color Group {}", group_id + 1),
                color,
                visible: true,
                layer_indices: indices.clone(),
            };
            self.groups.push(group);

            // Update layer group references
            for &idx in &indices {
                if let Some(layer) = self.layers.get_mut(idx) {
                    layer.group_id = Some(group_id);
                }
            }
        }
    }

    pub fn set_group_visibility(&mut self, group_id: usize, visible: bool) -> bool {
        if let Some(group) = self.groups.get_mut(group_id) {
            group.visible = visible;
            // Update all layers in group
            for &layer_idx in &group.layer_indices {
                if let Some(layer) = self.layers.get_mut(layer_idx) {
                    layer.visible = visible;
                }
            }
            true
        } else {
            false
        }
    }

    pub fn get_groups(&self) -> &[LayerGroup] {
        &self.groups
    }

    pub fn set_layer_visibility(&mut self, index: usize, visible: bool) -> bool {
        if let Some(layer) = self.layers.get_mut(index) {
            println!("Setting layer {} visibility to {}", index, visible);
            layer.visible = visible;
            true
        } else {
            false
        }
    }

    pub fn get_composite_image(&self) -> Option<RgbImage> {
        let base_image = self.original_image.as_ref()?.clone();
        let mut composite_data = base_image.to_rgb_data();
        let width = base_image.data_w();
        let height = base_image.data_h();
    
        println!("Composite image dimensions: {}x{}", width, height);
        println!("Number of layers to process: {}", 
            self.layers.iter().filter(|l| l.visible).count());
    
        for (i, layer) in self.layers.iter().filter(|l| l.visible).enumerate() {
            println!("Processing layer {} with opacity {}", i, layer.opacity);
            println!("Layer {} details:", i);
            println!("  Color: RGB({}, {}, {})", layer.color.0, layer.color.1, layer.color.2);
            println!("  Opacity: {}", layer.opacity);
            
            if let Some(region) = &layer.region {
                let (sel_x, sel_y, sel_w, sel_h) = region.get_image_dimensions();
                println!("Layer region: x={}, y={}, w={}, h={}", sel_x, sel_y, sel_w, sel_h);
                
                if sel_w <= 0 || sel_h <= 0 {
                    println!("Warning: Skipping layer {} due to invalid region dimensions", i);
                    continue;
                }
                
                for y in sel_y..sel_y + sel_h {
                    for x in sel_x..sel_x + sel_w {
                        if x >= 0 && y >= 0 && x < width && y < height {
                            let pixel_idx = (y * width + x) as usize * 3;
                            if pixel_idx + 2 < composite_data.len() {
                                for c in 0..3 {
                                    let color_val = match c {
                                        0 => layer.color.0,
                                        1 => layer.color.1,
                                        2 => layer.color.2,
                                        _ => unreachable!(),
                                    };
                                    
                                    if x == sel_x && y == sel_y && c == 0 {
                                        println!("First pixel blend: original={}, color={}, opacity={}", 
                                            composite_data[pixel_idx + c], color_val, layer.opacity);
                                    }
                                    
                                    composite_data[pixel_idx + c] = ((1.0 - layer.opacity) * 
                                        composite_data[pixel_idx + c] as f32 + 
                                        layer.opacity * color_val as f32) as u8;
                                }
                            }
                        }
                    }
                }
            }
        }
    
        Some(RgbImage::new(&composite_data, width, height, ColorDepth::Rgb8).unwrap())
    }

    pub fn get_layer_count(&self) -> usize {
        self.layers.len()
    }

    pub fn get_layer(&self, index: usize) -> Option<&Layer> {
        self.layers.get(index)
    }

    pub fn get_layer_mut(&mut self, index: usize) -> Option<&mut Layer> {
        self.layers.get_mut(index)
    }

    pub fn get_original_image(&self) -> Option<&RgbImage> {
        self.original_image.as_ref()
    }

    pub fn remove_layer(&mut self, index: usize) -> bool {
        if index >= self.layers.len() {
            return false;
        }

        self.layers.remove(index);

        for (i, layer) in self.layers.iter_mut().enumerate() {
            layer.name = format!("Layer {}", i + 1);
        }

        self.active_layer = if self.layers.is_empty() {
            None
        } else if index >= self.layers.len() {
            Some(self.layers.len() - 1)
        } else {
            Some(index)
        };

        if self.layers.is_empty() {
            self.original_image = None;
            println!("All layers removed, original image cleared");
        }

        self.update_groups();
        true
    }

    pub fn get_active_layer(&self) -> Option<usize> {
        self.active_layer
    }

    pub fn set_active_layer(&mut self, index: usize) -> bool {
        if index < self.layers.len() {
            self.active_layer = Some(index);
            true
        } else {
            false
        }
    }
}

impl Default for LayerState {
    fn default() -> Self {
        Self::new()
    }
}
    