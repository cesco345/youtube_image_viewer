// src/state/layer_state.rs
use fltk::{prelude::*, image::RgbImage, enums::ColorDepth};
use image::{ImageBuffer, Rgba};

#[derive(Clone)]
pub struct Layer {
    pub name: String,
    pub image: RgbImage,
    pub opacity: f32,
    pub visible: bool,
    pub tint: (f32, f32, f32), // RGB tint values
}

#[derive(Clone)]
pub struct LayerState {
    layers: Vec<Layer>,
    active_layer: Option<usize>,
}

impl LayerState {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            active_layer: None,
        }
    }

    pub fn add_layer(&mut self, image: RgbImage) -> usize {
        let id = self.layers.len();
        let name = format!("Layer {}", id + 1);
        
        // Create different tints for different layers
        let tint = match id % 5 {
            0 => (1.0, 0.8, 0.8),  // Reddish
            1 => (0.8, 1.0, 0.8),  // Greenish
            2 => (0.8, 0.8, 1.0),  // Bluish
            3 => (1.0, 1.0, 0.8),  // Yellowish
            4 => (1.0, 0.8, 1.0),  // Purplish
            _ => (1.0, 1.0, 1.0),  // No tint
        };

        // Apply tint to the image
        let tinted_image = self.apply_tint(&image, tint);
        
        let layer = Layer {
            name,
            image: tinted_image,
            opacity: 1.0,
            visible: true,
            tint,
        };
        
        self.layers.push(layer);
        self.active_layer = Some(id);
        id
    }

    fn apply_tint(&self, image: &RgbImage, tint: (f32, f32, f32)) -> RgbImage {
        let width = image.data_w() as u32;
        let height = image.data_h() as u32;
        let data = image.to_rgb_data();

        let mut new_data = Vec::with_capacity(data.len());
        
        for chunk in data.chunks(3) {
            if chunk.len() == 3 {
                let r = (chunk[0] as f32 * tint.0).min(255.0) as u8;
                let g = (chunk[1] as f32 * tint.1).min(255.0) as u8;
                let b = (chunk[2] as f32 * tint.2).min(255.0) as u8;
                new_data.extend_from_slice(&[r, g, b]);
            }
        }

        RgbImage::new(&new_data, image.data_w(), image.data_h(), ColorDepth::Rgb8).unwrap()
    }

    pub fn get_composite_image(&self) -> Option<RgbImage> {
        if self.layers.is_empty() {
            return None;
        }

        // Start with the bottom layer
        let mut composite_data = self.layers[0].image.to_rgb_data();
        let width = self.layers[0].image.data_w();
        let height = self.layers[0].image.data_h();

        // Blend each subsequent visible layer
        for layer in self.layers.iter().skip(1) {
            if !layer.visible {
                continue;
            }

            let layer_data = layer.image.to_rgb_data();
            
            // Blend pixels
            for (i, chunk) in composite_data.chunks_mut(3).enumerate() {
                if chunk.len() == 3 && i * 3 + 2 < layer_data.len() {
                    let opacity = layer.opacity;
                    for c in 0..3 {
                        let idx = i * 3 + c;
                        chunk[c] = ((1.0 - opacity) * chunk[c] as f32 + 
                                  opacity * layer_data[idx] as f32) as u8;
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

    pub fn remove_layer(&mut self, index: usize) -> bool {
        if index >= self.layers.len() {
            return false;
        }

        self.layers.remove(index);

        // Update layer names
        for (i, layer) in self.layers.iter_mut().enumerate() {
            layer.name = format!("Layer {}", i + 1);
        }

        // Update active layer
        match self.active_layer {
            Some(active) if active == index => {
                // If we removed the active layer, select the previous one
                self.active_layer = if index > 0 {
                    Some(index - 1)
                } else if !self.layers.is_empty() {
                    Some(0)
                } else {
                    None
                };
            }
            Some(active) if active > index => {
                // If active layer was after the removed one, decrement its index
                self.active_layer = Some(active - 1);
            }
            _ => {} // Other cases don't need adjustment
        }

        true
    }
}

impl Default for LayerState {
    fn default() -> Self {
        Self::new()
    }
}