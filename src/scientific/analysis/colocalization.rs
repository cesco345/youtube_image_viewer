use fltk::image::RgbImage;
use fltk::prelude::*;  
use crate::scientific::layers::Channel;
use std::collections::HashMap;


pub struct ColocalizationAnalysis {
    pub pearson_coefficient: f32,
    pub overlap_coefficient: f32,
    pub manders_coefficients: (f32, f32),
    pub intensity_correlation_quotient: f32,
}

impl ColocalizationAnalysis {
    pub fn analyze(channel1: &Channel, channel2: &Channel) -> Self {
        let (intensities1, intensities2) = Self::get_intensity_vectors(channel1, channel2);
        
        Self {
            pearson_coefficient: Self::calculate_pearson(&intensities1, &intensities2),
            overlap_coefficient: Self::calculate_overlap(&intensities1, &intensities2),
            manders_coefficients: Self::calculate_manders(&intensities1, &intensities2),
            intensity_correlation_quotient: Self::calculate_icq(&intensities1, &intensities2),
        }
    }

    fn get_intensity_vectors(channel1: &Channel, channel2: &Channel) -> (Vec<f32>, Vec<f32>) {
        let data1 = channel1.image.to_rgb_data();
        let data2 = channel2.image.to_rgb_data();
        let mut intensities1 = Vec::new();
        let mut intensities2 = Vec::new();

        for (chunk1, chunk2) in data1.chunks(3).zip(data2.chunks(3)) {
            let intensity1 = (chunk1[0] as f32 + chunk1[1] as f32 + chunk1[2] as f32) / (3.0 * 255.0);
            let intensity2 = (chunk2[0] as f32 + chunk2[1] as f32 + chunk2[2] as f32) / (3.0 * 255.0);
            intensities1.push(intensity1);
            intensities2.push(intensity2);
        }

        (intensities1, intensities2)
    }

    fn calculate_pearson(x: &[f32], y: &[f32]) -> f32 {
        let n = x.len() as f32;
        let mean_x = x.iter().sum::<f32>() / n;
        let mean_y = y.iter().sum::<f32>() / n;
        
        let numerator: f32 = x.iter().zip(y.iter())
            .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
            .sum();
            
        let denominator = (x.iter().map(|xi| (xi - mean_x).powi(2)).sum::<f32>() *
            y.iter().map(|yi| (yi - mean_y).powi(2)).sum::<f32>()).sqrt();
            
        if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
        }
    }

    fn calculate_overlap(x: &[f32], y: &[f32]) -> f32 {
        let numerator: f32 = x.iter().zip(y.iter())
            .map(|(xi, yi)| xi * yi)
            .sum();
            
        let denominator = (x.iter().map(|xi| xi.powi(2)).sum::<f32>() *
            y.iter().map(|yi| yi.powi(2)).sum::<f32>()).sqrt();
            
        if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
        }
    }

    fn calculate_manders(x: &[f32], y: &[f32]) -> (f32, f32) {
        let sum_xi_yi: f32 = x.iter().zip(y.iter())
            .map(|(xi, yi)| if *yi > 0.0 { *xi } else { 0.0 })
            .sum();
        let sum_yi_xi: f32 = y.iter().zip(x.iter())
            .map(|(yi, xi)| if *xi > 0.0 { *yi } else { 0.0 })
            .sum();
            
        let sum_x: f32 = x.iter().sum();
        let sum_y: f32 = y.iter().sum();
        
        let m1 = if sum_x == 0.0 { 0.0 } else { sum_xi_yi / sum_x };
        let m2 = if sum_y == 0.0 { 0.0 } else { sum_yi_xi / sum_y };
        
        (m1, m2)
    }

    fn calculate_icq(x: &[f32], y: &[f32]) -> f32 {
        let mean_x = x.iter().sum::<f32>() / x.len() as f32;
        let mean_y = y.iter().sum::<f32>() / y.len() as f32;
        
        let positive_counts = x.iter().zip(y.iter())
            .filter(|(&xi, &yi)| (xi - mean_x) * (yi - mean_y) > 0.0)
            .count();
            
        (2.0 * positive_counts as f32 / x.len() as f32) - 1.0
    }
}