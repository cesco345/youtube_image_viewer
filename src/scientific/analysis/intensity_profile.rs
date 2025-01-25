use fltk::image::RgbImage;
use fltk::prelude::*;  
use crate::scientific::layers::Channel;
pub struct IntensityProfile {
    pub x_values: Vec<f32>,
    pub intensities: Vec<Vec<f32>>,
    pub channels: Vec<usize>,
}

impl IntensityProfile {
    pub fn new(line_points: &[(i32, i32)], channels: &[Channel]) -> Self {
        let mut profile = Self {
            x_values: Vec::new(),
            intensities: vec![Vec::new(); channels.len()],
            channels: (0..channels.len()).collect(),
        };
        
        profile.calculate_profile(line_points, channels);
        profile
    }

    fn calculate_profile(&mut self, line_points: &[(i32, i32)], channels: &[Channel]) {
        if line_points.len() < 2 {
            return;
        }

        let (x1, y1) = line_points[0];
        let (x2, y2) = line_points[1];
        let distance = ((x2 - x1).pow(2) as f32 + (y2 - y1).pow(2) as f32).sqrt();
        let steps = distance as usize;

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let x = x1 as f32 + (x2 - x1) as f32 * t;
            let y = y1 as f32 + (y2 - y1) as f32 * t;
            
            self.x_values.push(i as f32);
            
            for (idx, channel) in channels.iter().enumerate() {
                let intensity = self.sample_intensity(&channel.image, x as i32, y as i32);
                self.intensities[idx].push(intensity);
            }
        }
    }

    fn sample_intensity(&self, image: &RgbImage, x: i32, y: i32) -> f32 {
        if x < 0 || y < 0 || x >= image.data_w() as i32 || y >= image.data_h() as i32 {
            return 0.0;
        }

        let idx = (y * image.data_w() as i32 + x) as usize * 3;
        let data = image.to_rgb_data();
        
        if idx + 2 < data.len() {
            (data[idx] as f32 + data[idx + 1] as f32 + data[idx + 2] as f32) / (3.0 * 255.0)
        } else {
            0.0
        }
    }
}