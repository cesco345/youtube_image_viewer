use fltk::{
    window::Window,
    button::Button,
    frame::Frame,
    group::{Pack, Tabs, Group},
    prelude::*,
    enums::FrameType,
};
use std::{rc::Rc, cell::RefCell};
use crate::scientific::{
    analysis::{
        CellMeasurement,
        cell_statistics::{CellStatistics, StatisticalAnalysis},
    },
    visualization::cell_plots::CellVisualizer
};
use super::{
    table::create_data_table,
    WINDOW_PADDING, BUTTON_HEIGHT, BUTTON_WIDTH
};

pub fn show_statistics_dialog(measurements: &[CellMeasurement]) {
    let mut wind = Window::default()
        .with_size(800, 600)
        .with_label("Measurement Statistics");
    wind.make_modal(true);

    // Get statistics using the StatisticalAnalysis trait
    let stats = measurements.analyze();

    // Create tabs for different views
    let mut tabs = Tabs::new(WINDOW_PADDING, WINDOW_PADDING, 780, 540, "");
    
    // Summary tab
    let mut summary_group = Group::new(WINDOW_PADDING, 35, 780, 515, "Summary");
    
    // Create summary data table
    let data = create_summary_data(&stats);
    create_data_table(
        WINDOW_PADDING * 2,
        45,
        760,
        200,
        data
    );

    // Add visualizer for histograms
    let visualizer = CellVisualizer::new(370, 250);
    
    // Add area histogram
    if let Ok(mut area_hist) = visualizer.create_histogram(measurements, "area") {
        let mut area_frame = Frame::new(WINDOW_PADDING * 2, 255, 370, 250, "Area Distribution");
        area_frame.set_frame(FrameType::FlatBox);
        area_hist.draw(area_frame.x(), area_frame.y(), area_frame.width(), area_frame.height());
    }

    // Add circularity histogram
    if let Ok(mut circ_hist) = visualizer.create_histogram(measurements, "circularity") {
        let mut circ_frame = Frame::new(WINDOW_PADDING * 2 + 380, 255, 370, 250, "Circularity Distribution");
        circ_frame.set_frame(FrameType::FlatBox);
        circ_hist.draw(circ_frame.x(), circ_frame.y(), circ_frame.width(), circ_frame.height());
    }

    summary_group.end();

    // Correlations tab
    let mut correlations_group = Group::new(WINDOW_PADDING, 35, 780, 515, "Correlations");
    
    if let Ok(mut area_vs_circ) = visualizer.create_scatter_plot(measurements, "area", "circularity") {
        let mut scatter_frame = Frame::new(WINDOW_PADDING * 2, 45, 370, 370, "Area vs Circularity");
        scatter_frame.set_frame(FrameType::FlatBox);
        area_vs_circ.draw(scatter_frame.x(), scatter_frame.y(), scatter_frame.width(), scatter_frame.height());
    }

    // Create correlation matrix table
    let correlation_data = create_correlation_data(&stats);
    create_data_table(
        WINDOW_PADDING * 2 + 380,
        45,
        370,
        370,
        correlation_data
    );

    correlations_group.end();
    
    tabs.end();

    // Add close button
    let mut close_btn = Button::new(
        800/2 - BUTTON_WIDTH/2,
        600 - WINDOW_PADDING - BUTTON_HEIGHT,
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
        "Close"
    );

    let wind_rc = Rc::new(RefCell::new(wind));
    let wind_rc_clone = wind_rc.clone();
    close_btn.set_callback(move |_| {
        wind_rc_clone.borrow_mut().hide();
    });

    wind_rc.borrow_mut().end();
    wind_rc.borrow_mut().show();

    while wind_rc.borrow().shown() {
        fltk::app::wait();
    }
}

fn create_summary_data(stats: &CellStatistics) -> Vec<Vec<String>> {
    vec![
        // Headers
        vec!["Metric".to_string(), "Area".to_string(), "Perimeter".to_string(), 
             "Circularity".to_string(), "Intensity".to_string()],
        // Mean row
        vec!["Mean".to_string(), 
             format!("{:.2}", stats.area_stats.mean),
             format!("{:.2}", stats.perimeter_stats.mean),
             format!("{:.3}", stats.circularity_stats.mean),
             format!("{:.1}", stats.intensity_stats.mean)],
        // Standard Deviation row
        vec!["Std Dev".to_string(),
             format!("{:.2}", stats.area_stats.std_dev),
             format!("{:.2}", stats.perimeter_stats.std_dev),
             format!("{:.3}", stats.circularity_stats.std_dev),
             format!("{:.1}", stats.intensity_stats.std_dev)],
        // CV row
        vec!["CV (%)".to_string(),
             format!("{:.1}%", stats.area_stats.coefficient_of_variation * 100.0),
             format!("{:.1}%", stats.perimeter_stats.coefficient_of_variation * 100.0),
             format!("{:.1}%", stats.circularity_stats.coefficient_of_variation * 100.0),
             format!("{:.1}%", stats.intensity_stats.coefficient_of_variation * 100.0)],
        // Min row
        vec!["Min".to_string(),
             format!("{:.2}", stats.area_stats.min),
             format!("{:.2}", stats.perimeter_stats.min),
             format!("{:.3}", stats.circularity_stats.min),
             format!("{:.1}", stats.intensity_stats.min)],
        // Max row
        vec!["Max".to_string(),
             format!("{:.2}", stats.area_stats.max),
             format!("{:.2}", stats.perimeter_stats.max),
             format!("{:.3}", stats.circularity_stats.max),
             format!("{:.1}", stats.intensity_stats.max)],
        // Skewness row
        vec!["Skewness".to_string(),
             format!("{:.3}", stats.area_stats.skewness),
             format!("{:.3}", stats.perimeter_stats.skewness),
             format!("{:.3}", stats.circularity_stats.skewness),
             format!("{:.3}", stats.intensity_stats.skewness)],
        // Kurtosis row
        vec!["Kurtosis".to_string(),
             format!("{:.3}", stats.area_stats.kurtosis),
             format!("{:.3}", stats.perimeter_stats.kurtosis),
             format!("{:.3}", stats.circularity_stats.kurtosis),
             format!("{:.3}", stats.intensity_stats.kurtosis)]
    ]
}

fn create_correlation_data(stats: &CellStatistics) -> Vec<Vec<String>> {
    vec![
        vec!["Parameter".to_string(), "Area".to_string(), "Perimeter".to_string(), 
             "Circularity".to_string(), "Intensity".to_string()],
        vec!["Area".to_string(), 
             "1.000".to_string(),
             format!("{:.3}", stats.correlations.area_perimeter),
             format!("{:.3}", stats.correlations.area_circularity),
             format!("{:.3}", stats.correlations.area_intensity)],
        vec!["Perimeter".to_string(),
             format!("{:.3}", stats.correlations.area_perimeter),
             "1.000".to_string(),
             format!("{:.3}", stats.correlations.perimeter_circularity),
             format!("{:.3}", stats.correlations.perimeter_intensity)],
        vec!["Circularity".to_string(),
             format!("{:.3}", stats.correlations.area_circularity),
             format!("{:.3}", stats.correlations.perimeter_circularity),
             "1.000".to_string(),
             format!("{:.3}", stats.correlations.circularity_intensity)],
        vec!["Intensity".to_string(),
             format!("{:.3}", stats.correlations.area_intensity),
             format!("{:.3}", stats.correlations.perimeter_intensity),
             format!("{:.3}", stats.correlations.circularity_intensity),
             "1.000".to_string()]
    ]
}