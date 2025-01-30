// src/scientific/ui/cell_analysis/dialog.rs
use fltk::{
    window::Window,
    button::Button,
    frame::Frame,
    group::{Group, Pack, Tabs},
    enums::{Color, FrameType},
    prelude::*,
};
use std::{rc::Rc, cell::RefCell};
use crate::{
    state::ImageState,
    scientific::{
        analysis::{
            CellMeasurement,
            cell_statistics::{CellStatistics, StatisticalAnalysis}
        },
        visualization::cell_plots::CellVisualizer,
        types::CellMeasurementMode,
        tools::interactive::cell_analysis_tool::CellAnalysisState
    }
};

use super::{
    export::{export_measurement_data, export_batch_measurements},
    table::create_data_table,
    WINDOW_PADDING, BUTTON_HEIGHT, BUTTON_WIDTH
};

pub fn show_cell_analysis_dialog(
    frame: &Rc<RefCell<Frame>>,
    state: &Rc<RefCell<ImageState>>,
    measurement: &CellMeasurement
) {
    let mut wind = Window::default()
        .with_size(800, 600)
        .with_label("Cell Analysis Results");
    wind.make_modal(true);

    let tabs = Tabs::new(WINDOW_PADDING, WINDOW_PADDING, 780, 540, "");
    
    let measurements_group = Group::new(WINDOW_PADDING, 35, 780, 515, "Measurements");
    
    let (mean_int, min_int, max_int) = measurement.format_intensities();
    let data = vec![
        vec!["Metric", "Value", "Unit"],
        vec!["Area", &measurement.format_area(), &measurement.calibration_unit],
        vec!["Perimeter", &measurement.format_perimeter(), &measurement.calibration_unit],
        vec!["Circularity", &measurement.format_circularity(), "-"],
        vec!["Mean Intensity", &mean_int, "A.U."],
        vec!["Min Intensity", &min_int, "A.U."],
        vec!["Max Intensity", &max_int, "A.U."],
    ].iter().map(|row| row.iter().map(|&s| s.to_string()).collect()).collect();

    create_data_table(
        WINDOW_PADDING * 2,
        45,
        760,
        200,
        data
    );

    let visualizer = CellVisualizer::new(700, 250);
    if let Ok(mut histogram) = visualizer.create_histogram(&[measurement.clone()], "intensity") {
        let mut profile_frame = Frame::new(WINDOW_PADDING * 2, 255, 760, 250, "Intensity Distribution");
        profile_frame.set_frame(FrameType::FlatBox);
        histogram.draw(profile_frame.x(), profile_frame.y(), profile_frame.width(), profile_frame.height());
    }

    measurements_group.end();
    tabs.end();

    let mut button_pack = Pack::new(WINDOW_PADDING, 560, 780, BUTTON_HEIGHT, "");
    button_pack.set_type(fltk::group::PackType::Horizontal);
    button_pack.set_spacing(10);

    let mut export_btn = Button::new(0, 0, BUTTON_WIDTH + 30, BUTTON_HEIGHT, "Export");
    let state_clone = state.clone();
    let measurement = measurement.clone();
    export_btn.set_callback(move |_| {
        if let Ok(state_ref) = state_clone.try_borrow() {
            export_measurement_data(&state_ref, &measurement);
        }
    });

    let mut update_btn = Button::new(0, 0, BUTTON_WIDTH + 30, BUTTON_HEIGHT, "Update ROI");
    let frame_clone = frame.clone();
    let state_clone = state.clone();
    update_btn.set_callback(move |_| {
        if let Ok(mut state_ref) = state_clone.try_borrow_mut() {
            state_ref.scientific_state.start_cell_analysis(CellMeasurementMode::Single);
        }
        frame_clone.borrow_mut().redraw();
    });

    let mut close_btn = Button::new(0, 0, BUTTON_WIDTH, BUTTON_HEIGHT, "Close");
    let wind_rc = Rc::new(RefCell::new(wind));
    let wind_rc_clone = wind_rc.clone();
    close_btn.set_callback(move |_| {
        wind_rc_clone.borrow_mut().hide();
    });

    button_pack.end();
    wind_rc.borrow_mut().end();
    wind_rc.borrow_mut().show();

    while wind_rc.borrow().shown() {
        fltk::app::wait();
    }
}

pub fn show_batch_analysis_dialog(
    frame: &Rc<RefCell<Frame>>,
    state: &Rc<RefCell<ImageState>>,
    measurements: &[CellMeasurement]
) {
    let mut wind = Window::default()
        .with_size(600, 500)
        .with_label("Batch Cell Analysis Results");
    wind.make_modal(true);

    let mut data = vec![vec![
        "ID",
        "Area",
        "Perimeter",
        "Circularity",
        "Mean Int",
        "Min Int",
        "Max Int",
    ].iter().map(|s| s.to_string()).collect()];

    for (i, measurement) in measurements.iter().enumerate() {
        let (mean_int, min_int, max_int) = measurement.format_intensities();
        data.push(vec![
            format!("#{}", i + 1),
            measurement.format_area(),
            measurement.format_perimeter(),
            measurement.format_circularity(),
            mean_int,
            min_int,
            max_int,
        ]);
    }

    create_data_table(
        WINDOW_PADDING,
        WINDOW_PADDING,
        600 - 2 * WINDOW_PADDING,
        500 - 2 * WINDOW_PADDING - BUTTON_HEIGHT - 10,
        data
    );

    let button_y = 500 - WINDOW_PADDING - BUTTON_HEIGHT;
    let measurements_for_export = measurements.to_vec();
    
    let mut export_btn = Button::new(
        WINDOW_PADDING,
        button_y,
        BUTTON_WIDTH + 30,
        BUTTON_HEIGHT,
        "Export CSV"
    );
    let state_clone = state.clone();
    let measurements_clone = measurements_for_export.clone();
    export_btn.set_callback(move |_| {
        if let Ok(state_ref) = state_clone.try_borrow() {
            export_batch_measurements(&state_ref, &measurements_clone);
        }
    });

    let mut stats_btn = Button::new(
        WINDOW_PADDING * 2 + BUTTON_WIDTH + 30,
        button_y,
        BUTTON_WIDTH + 30,
        BUTTON_HEIGHT,
        "Statistics"
    );
    let measurements_clone = measurements.to_vec();
    stats_btn.set_callback(move |_| {
        super::statistics::show_statistics_dialog(&measurements_clone);
    });

    let mut close_btn = Button::new(
        600 - WINDOW_PADDING - BUTTON_WIDTH,
        button_y,
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