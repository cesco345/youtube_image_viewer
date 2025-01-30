use fltk::{
    image::RgbImage,
};
use plotters::prelude::*;
use crate::scientific::analysis::CellMeasurement;

pub struct CellVisualizer {
    chart_width: u32,
    chart_height: u32,
}

impl CellVisualizer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            chart_width: width,
            chart_height: height,
        }
    }

    fn get_binned_data(values: &[f64], bin_count: usize) -> (Vec<f64>, Vec<usize>) {
        let min_val = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let bin_width = (max_val - min_val) / bin_count as f64;
        
        let mut bins = vec![0; bin_count];
        let bin_edges: Vec<f64> = (0..=bin_count).map(|i| min_val + i as f64 * bin_width).collect();

        for &value in values {
            let bin = ((value - min_val) / bin_width).floor() as usize;
            if bin < bin_count {
                bins[bin] += 1;
            }
        }

        (bin_edges, bins)
    }

    pub fn create_histogram(
        &self,
        measurements: &[CellMeasurement],
        metric: &str,
    ) -> Result<RgbImage, Box<dyn std::error::Error>> {
        let mut buffer = vec![0u8; (self.chart_width * self.chart_height * 3) as usize];
        {
            let root = BitMapBackend::with_buffer(&mut buffer, (self.chart_width, self.chart_height))
                .into_drawing_area();
            root.fill(&WHITE)?;

            let values: Vec<f64> = match metric {
                "area" => measurements.iter().map(|m| m.area).collect(),
                "perimeter" => measurements.iter().map(|m| m.perimeter).collect(),
                "circularity" => measurements.iter().map(|m| m.circularity).collect(),
                "intensity" => measurements.iter().map(|m| m.mean_intensity).collect(),
                _ => return Err("Invalid metric".into()),
            };

            let bin_count = 20;
            let (bin_edges, bins) = Self::get_binned_data(&values, bin_count);
            let max_count = *bins.iter().max().unwrap_or(&0);

            let mut chart = ChartBuilder::on(&root)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption(format!("{} Distribution", metric), ("sans-serif", 20))
                .build_cartesian_2d(
                    bin_edges[0]..bin_edges[bin_edges.len()-1],
                    0f64..(max_count as f64)
                )?;

            chart.configure_mesh().draw()?;

            let bars: Vec<(f64, f64)> = bins.iter().enumerate()
                .map(|(i, &count)| (bin_edges[i], count as f64))
                .collect();

            chart.draw_series(
                bars.iter().map(|&(x, y)| {
                    Rectangle::new(
                        [(x, 0.0), (x + (bin_edges[1] - bin_edges[0]), y)],
                        RGBColor(0, 0, 255).filled(),
                    )
                }),
            )?;

            root.present()?;
        }
        Ok(RgbImage::new(&buffer, self.chart_width as i32, self.chart_height as i32, fltk::enums::ColorDepth::Rgb8)?)
    }

    pub fn create_scatter_plot(
        &self,
        measurements: &[CellMeasurement],
        x_metric: &str,
        y_metric: &str,
    ) -> Result<RgbImage, Box<dyn std::error::Error>> {
        let mut buffer = vec![0u8; (self.chart_width * self.chart_height * 3) as usize];
        {
            let root = BitMapBackend::with_buffer(&mut buffer, (self.chart_width, self.chart_height))
                .into_drawing_area();
            root.fill(&WHITE)?;

            let get_values = |metric: &str| -> Vec<f64> {
                match metric {
                    "area" => measurements.iter().map(|m| m.area).collect(),
                    "perimeter" => measurements.iter().map(|m| m.perimeter).collect(),
                    "circularity" => measurements.iter().map(|m| m.circularity).collect(),
                    "intensity" => measurements.iter().map(|m| m.mean_intensity).collect(),
                    _ => vec![],
                }
            };

            let x_values = get_values(x_metric);
            let y_values = get_values(y_metric);

            let x_min = x_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let x_max = x_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            let y_min = y_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let y_max = y_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

            let mut chart = ChartBuilder::on(&root)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption(format!("{} vs {}", x_metric, y_metric), ("sans-serif", 20))
                .build_cartesian_2d(
                    x_min..x_max,
                    y_min..y_max,
                )?;

            chart.configure_mesh().draw()?;

            chart.draw_series(
                x_values.iter().zip(y_values.iter()).map(|(&x, &y)| {
                    Circle::new((x, y), 3, BLUE.filled())
                }),
            )?;

            if x_values.len() > 1 {
                let n = x_values.len() as f64;
                let sum_x: f64 = x_values.iter().sum();
                let sum_y: f64 = y_values.iter().sum();
                let sum_xy: f64 = x_values.iter().zip(y_values.iter()).map(|(x, y)| x * y).sum();
                let sum_xx: f64 = x_values.iter().map(|x| x * x).sum();

                let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_xx - sum_x * sum_x);
                let intercept = (sum_y - slope * sum_x) / n;

                chart.draw_series(LineSeries::new(
                    vec![(x_min, slope * x_min + intercept), (x_max, slope * x_max + intercept)],
                    RED.stroke_width(2),
                ))?;
            }

            drop(chart);
            root.present()?;
        }
        Ok(RgbImage::new(&buffer, self.chart_width as i32, self.chart_height as i32, fltk::enums::ColorDepth::Rgb8)?)
    }
}