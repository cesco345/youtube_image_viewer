//src/scientific/analysis/cell_statistics.rs
use crate::scientific::analysis::CellMeasurement;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CellStatistics {
    pub sample_size: usize,
    pub area_stats: MetricStatistics,
    pub perimeter_stats: MetricStatistics,
    pub circularity_stats: MetricStatistics,
    pub intensity_stats: MetricStatistics,
    pub correlations: CorrelationMatrix,
}

#[derive(Debug, Clone, Serialize)]
pub struct MetricStatistics {
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub skewness: f64,
    pub kurtosis: f64,
    pub coefficient_of_variation: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct CorrelationMatrix {
    pub area_perimeter: f64,
    pub area_circularity: f64,
    pub area_intensity: f64,
    pub perimeter_circularity: f64,
    pub perimeter_intensity: f64,
    pub circularity_intensity: f64,
}

impl CellStatistics {
    pub fn new(measurements: &[CellMeasurement]) -> Self {
        let sample_size = measurements.len();
        
        let area_values: Vec<f64> = measurements.iter().map(|m| m.area).collect();
        let perimeter_values: Vec<f64> = measurements.iter().map(|m| m.perimeter).collect();
        let circularity_values: Vec<f64> = measurements.iter().map(|m| m.circularity).collect();
        let intensity_values: Vec<f64> = measurements.iter().map(|m| m.mean_intensity).collect();

        let area_stats = MetricStatistics::calculate(&area_values);
        let perimeter_stats = MetricStatistics::calculate(&perimeter_values);
        let circularity_stats = MetricStatistics::calculate(&circularity_values);
        let intensity_stats = MetricStatistics::calculate(&intensity_values);

        let correlations = CorrelationMatrix::new(
            &area_values,
            &perimeter_values,
            &circularity_values,
            &intensity_values,
        );

        Self {
            sample_size,
            area_stats,
            perimeter_stats,
            circularity_stats,
            intensity_stats,
            correlations,
        }
    }

    fn create_metrics<'a>(&'a self, measurements: &'a [CellMeasurement]) -> Vec<(&str, &MetricStatistics, Box<dyn Iterator<Item = f64> + 'a>)> {
        vec![
            ("Area", &self.area_stats, Box::new(measurements.iter().map(|m| m.area))),
            ("Perimeter", &self.perimeter_stats, Box::new(measurements.iter().map(|m| m.perimeter))),
            ("Circularity", &self.circularity_stats, Box::new(measurements.iter().map(|m| m.circularity))),
            ("Mean Intensity", &self.intensity_stats, Box::new(measurements.iter().map(|m| m.mean_intensity)))
        ]
    }

    pub fn detect_outliers(&self, measurements: &[CellMeasurement]) -> Vec<(usize, String)> {
        let mut outliers = Vec::new();
        
        // Create tuples of metric info that we can iterate over
        let metrics = vec![
            ("Area", &self.area_stats, measurements.iter().map(|m| m.area).collect::<Vec<_>>()),
            ("Perimeter", &self.perimeter_stats, measurements.iter().map(|m| m.perimeter).collect::<Vec<_>>()),
            ("Circularity", &self.circularity_stats, measurements.iter().map(|m| m.circularity).collect::<Vec<_>>()),
            ("Mean Intensity", &self.intensity_stats, measurements.iter().map(|m| m.mean_intensity).collect::<Vec<_>>())
        ];
    
        for (idx, measurement) in measurements.iter().enumerate() {
            for (metric_name, stats, values) in &metrics {
                let value = match *metric_name {
                    "Area" => measurement.area,
                    "Perimeter" => measurement.perimeter,
                    "Circularity" => measurement.circularity,
                    "Mean Intensity" => measurement.mean_intensity,
                    _ => continue,
                };
    
                // Use Tukey's method (1.5 * IQR)
                let q1 = percentile(values.iter().copied(), 0.25);
                let q3 = percentile(values.iter().copied(), 0.75);
                let iqr = q3 - q1;
                let lower_bound = q1 - 1.5 * iqr;
                let upper_bound = q3 + 1.5 * iqr;
    
                if value < lower_bound || value > upper_bound {
                    outliers.push((idx, metric_name.to_string()));
                }
            }
        }
    
        outliers
    }
}

impl MetricStatistics {
    fn calculate(values: &[f64]) -> Self {
        let n = values.len() as f64;
        if n == 0.0 {
            return Self {
                mean: 0.0,
                median: 0.0,
                std_dev: 0.0,
                min: 0.0,
                max: 0.0,
                skewness: 0.0,
                kurtosis: 0.0,
                coefficient_of_variation: 0.0,
            };
        }

        let mean = values.iter().sum::<f64>() / n;
        
        let mut sorted_values = values.to_vec();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = if values.len() % 2 == 0 {
            (sorted_values[values.len() / 2 - 1] + sorted_values[values.len() / 2]) / 2.0
        } else {
            sorted_values[values.len() / 2]
        };

        let variance = values.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / (n - 1.0);
        let std_dev = variance.sqrt();

        let min = *values.iter().fold(&f64::INFINITY, |a, b| if b < a { b } else { a });
        let max = *values.iter().fold(&f64::NEG_INFINITY, |a, b| if b > a { b } else { a });

        // Calculate skewness
        let m3 = values.iter()
            .map(|&x| (x - mean).powi(3))
            .sum::<f64>() / n;
        let skewness = m3 / std_dev.powi(3);

        // Calculate kurtosis
        let m4 = values.iter()
            .map(|&x| (x - mean).powi(4))
            .sum::<f64>() / n;
        let kurtosis = (m4 / std_dev.powi(4)) - 3.0;  // Excess kurtosis

        let coefficient_of_variation = if mean != 0.0 { std_dev / mean } else { 0.0 };

        Self {
            mean,
            median,
            std_dev,
            min,
            max,
            skewness,
            kurtosis,
            coefficient_of_variation,
        }
    }
}

impl CorrelationMatrix {
    fn new(
        area_values: &[f64],
        perimeter_values: &[f64],
        circularity_values: &[f64],
        intensity_values: &[f64],
    ) -> Self {
        Self {
            area_perimeter: correlation(area_values, perimeter_values),
            area_circularity: correlation(area_values, circularity_values),
            area_intensity: correlation(area_values, intensity_values),
            perimeter_circularity: correlation(perimeter_values, circularity_values),
            perimeter_intensity: correlation(perimeter_values, intensity_values),
            circularity_intensity: correlation(circularity_values, intensity_values),
        }
    }
}

fn correlation(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len() as f64;
    if n == 0.0 {
        return 0.0;
    }
    
    let mean_x = x.iter().sum::<f64>() / n;
    let mean_y = y.iter().sum::<f64>() / n;
    
    let covariance = x.iter().zip(y.iter())
        .map(|(&xi, &yi)| (xi - mean_x) * (yi - mean_y))
        .sum::<f64>() / (n - 1.0);
    
    let std_x = (x.iter()
        .map(|&xi| (xi - mean_x).powi(2))
        .sum::<f64>() / (n - 1.0))
        .sqrt();
    
    let std_y = (y.iter()
        .map(|&yi| (yi - mean_y).powi(2))
        .sum::<f64>() / (n - 1.0))
        .sqrt();
    
    if std_x == 0.0 || std_y == 0.0 {
        0.0
    } else {
        covariance / (std_x * std_y)
    }
}

fn percentile(mut values: impl Iterator<Item = f64>, p: f64) -> f64 {
    let mut v: Vec<f64> = values.collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    if v.is_empty() {
        return 0.0;
    }
    
    let pos = (v.len() as f64 - 1.0) * p;
    let index = pos.floor() as usize;
    if index + 1 >= v.len() {
        v[index]
    } else {
        let fraction = pos - index as f64;
        v[index] * (1.0 - fraction) + v[index + 1] * fraction
    }
}

pub trait StatisticalAnalysis {
    fn analyze(&self) -> CellStatistics;
    fn detect_outliers(&self) -> Vec<(usize, String)>;
    fn calculate_shape_factors(&self) -> Vec<f64>;
}

impl StatisticalAnalysis for [CellMeasurement] {
    fn analyze(&self) -> CellStatistics {
        CellStatistics::new(self)
    }

    fn detect_outliers(&self) -> Vec<(usize, String)> {
        let stats = self.analyze();
        stats.detect_outliers(self)
    }

    fn calculate_shape_factors(&self) -> Vec<f64> {
        self.iter()
            .map(|m| {
                if m.perimeter == 0.0 {
                    0.0
                } else {
                    4.0 * std::f64::consts::PI * m.area / (m.perimeter * m.perimeter)
                }
            })
            .collect()
    }
}

pub fn export_statistics_report(stats: &CellStatistics, path: &std::path::Path) -> std::io::Result<()> {
    use std::io::Write;
    let mut file = std::fs::File::create(path)?;

    writeln!(file, "Cell Analysis Statistical Report")?;
    writeln!(file, "==============================")?;
    writeln!(file, "\nSample Size: {}\n", stats.sample_size)?;

    // Write basic statistics for each metric
    for (metric, stat) in [
        ("Area", &stats.area_stats),
        ("Perimeter", &stats.perimeter_stats),
        ("Circularity", &stats.circularity_stats),
        ("Intensity", &stats.intensity_stats),
    ] {
        writeln!(file, "{} Statistics:", metric)?;
        writeln!(file, "-----------------")?;
        writeln!(file, "Mean: {:.3}", stat.mean)?;
        writeln!(file, "Median: {:.3}", stat.median)?;
        writeln!(file, "Standard Deviation: {:.3}", stat.std_dev)?;
        writeln!(file, "Min: {:.3}", stat.min)?;
        writeln!(file, "Max: {:.3}", stat.max)?;
        writeln!(file, "Coefficient of Variation: {:.3}", stat.coefficient_of_variation)?;
        writeln!(file, "Skewness: {:.3}", stat.skewness)?;
        writeln!(file, "Kurtosis: {:.3}\n", stat.kurtosis)?;
    }

    // Write correlation matrix
    writeln!(file, "Correlation Matrix:")?;
    writeln!(file, "-----------------")?;
    writeln!(file, "Area-Perimeter: {:.3}", stats.correlations.area_perimeter)?;
    writeln!(file, "Area-Circularity: {:.3}", stats.correlations.area_circularity)?;
    writeln!(file, "Area-Intensity: {:.3}", stats.correlations.area_intensity)?;
    writeln!(file, "Perimeter-Circularity: {:.3}", stats.correlations.perimeter_circularity)?;
    writeln!(file, "Perimeter-Intensity: {:.3}", stats.correlations.perimeter_intensity)?;
    writeln!(file, "Circularity-Intensity: {:.3}", stats.correlations.circularity_intensity)?;

    Ok(())
}

