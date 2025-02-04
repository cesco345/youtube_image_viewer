// src/scientific/ui/roi/batch_dialog.rs

use fltk::{
    window::{Window, DoubleWindow},
    group::{Pack, Scroll, Group, PackType},
    button::{Button, CheckButton, RadioRoundButton},    
    frame::Frame,
    table::TableRow,
    enums::{Color, Font, FrameType, Align},
    prelude::*,
};
use std::{rc::Rc, cell::RefCell, collections::HashMap};
use crate::scientific::{
    types::{ROIMeasurements, CellMeasurementMode},
    tools::interactive::roi::measurements::MeasurementCalculator,
};
use crate::scientific::analysis::CellMeasurement;
use super::components::measurement_table::MeasurementTable;
use crate::ImageState;

#[derive(Debug, Clone)]
pub struct BatchStatistics {
    count: usize,
    mean_area: f64,
    std_dev_area: f64,
    mean_intensity: f64,
    std_dev_intensity: f64,
    min_area: f64,
    max_area: f64,
    min_intensity: f64,
    max_intensity: f64,
    units: String,
}

#[derive(Clone)]
pub struct BatchDialog {
    window: DoubleWindow,
    measurements_table: MeasurementTable,
    statistics_table: TableRow,
    stats_frames: Vec<Frame>,
    mode_buttons: Vec<RadioRoundButton>,
    current_mode: CellMeasurementMode,
    measurement_sets: Vec<ROIMeasurements>,
    batch_stats: Option<BatchStatistics>,
    auto_update: bool,
    update_button: Button,
    filter_button: Button,
    export_button: Button,
    close_button: Button,
}

impl BatchDialog {
    pub fn new(
        parent: Rc<RefCell<Window>>,
        initial_measurements: Vec<ROIMeasurements>,
        mode: CellMeasurementMode,
    ) -> Self {
        let mut window = DoubleWindow::new(
            0, 0, 800, 600,
            "Batch Analysis"
        );

        // Center dialog
        let parent = parent.borrow();
        let x = parent.x() + (parent.width() - window.width()) / 2;
        let y = parent.y() + (parent.height() - window.height()) / 2;
        window.set_pos(x, y);

        let mut pack = Pack::new(10, 10, 780, 580, "");
        pack.set_spacing(10);

        // Header with mode selection
        let mut header_group = Pack::new(0, 0, 780, 60, "");
        header_group.set_spacing(5);
        
        let mut title = Frame::new(0, 0, 780, 25, "ROI Batch Analysis");
        title.set_label_size(16);
        title.set_label_color(Color::Dark3);

        let mut mode_pack = Pack::new(0, 0, 780, 30, "");
        mode_pack.set_type(PackType::Horizontal);
        mode_pack.set_spacing(10);

        let mut single_mode = RadioRoundButton::new(0, 0, 120, 30, "Single");
        let mut batch_mode = RadioRoundButton::new(0, 0, 120, 30, "Batch");
        let mut auto_mode = RadioRoundButton::new(0, 0, 120, 30, "Auto Detect");
        
        match mode {
            CellMeasurementMode::Single => single_mode.set_value(true),
            CellMeasurementMode::Batch => batch_mode.set_value(true),
            CellMeasurementMode::AutoDetect => auto_mode.set_value(true),
        }
        
        mode_pack.end();
        header_group.end();

        // Main content area
        let mut content_pack = Pack::new(0, 0, 780, 450, "");
        content_pack.set_spacing(10);

        // Measurements table
        let mut table_group = Group::new(0, 0, 780, 300, "Measurements");
        table_group.set_frame(FrameType::ThinUpBox);
        
        let mut scroll = Scroll::new(5, 25, 770, 270, "");
        let table = MeasurementTable::new(
            0, 0, 770, 270,
            initial_measurements.clone()
        );
        scroll.end();
        table_group.end();

        // Statistics section
        let mut stats_group = Group::new(0, 0, 780, 140, "Statistics");
        stats_group.set_frame(FrameType::ThinUpBox);
        
        let mut stats_pack = Pack::new(5, 25, 770, 110, "");
        stats_pack.set_spacing(5);
        
        let mut stats_frames = Vec::new();
        for label in ["Count", "Mean Area", "Std Dev Area", "Mean Intensity", "Min/Max"] {
            let mut frame = Frame::new(0, 0, 770, 20, label);
            frame.set_align(Align::Left | Align::Inside);
            stats_frames.push(frame);
        }
        
        stats_pack.end();
        stats_group.end();
        
        content_pack.end();

        // Control buttons
        let mut control_pack = Pack::new(0, 0, 780, 30, "");
        control_pack.set_type(PackType::Horizontal);
        control_pack.set_spacing(10);
        
        let mut auto_update = CheckButton::new(0, 0, 150, 30, "Auto Update");
        auto_update.set_checked(true);
        
        let update_button = Button::new(0, 0, 150, 30, "Update Statistics");
        let filter_button = Button::new(0, 0, 150, 30, "Filter Data");
        let export_button = Button::new(0, 0, 150, 30, "Export Results");
        let close_button = Button::new(0, 0, 150, 30, "Close");
        
        control_pack.end();

        pack.end();
        window.end();
        window.make_modal(true);

        let mut dialog = Self {
            window,
            measurements_table: table,
            statistics_table: TableRow::default(),
            stats_frames,
            mode_buttons: vec![single_mode, batch_mode, auto_mode],
            current_mode: mode,
            measurement_sets: initial_measurements,
            batch_stats: None,
            auto_update: true,
            update_button,
            filter_button,
            export_button,
            close_button,
        };

        dialog.setup_callbacks();
        dialog.calculate_statistics();
        dialog.update_display();
        dialog
    }

    fn setup_callbacks(&mut self) {
        // Mode selection callbacks
        let mode_buttons = self.mode_buttons.clone();
        let this = Rc::new(RefCell::new(self.clone()));
        
        for mut button in mode_buttons {
            let mode = match button.label().as_ref() {
                "Single" => CellMeasurementMode::Single,
                "Batch" => CellMeasurementMode::Batch,
                "Auto Detect" => CellMeasurementMode::AutoDetect,
                _ => continue,
            };
            
            let this = this.clone();
            button.set_callback(move |b| {
                if b.is_set() {
                    if let Ok(mut dialog) = this.try_borrow_mut() {
                        dialog.current_mode = mode;
                        dialog.update_display();
                    }
                }
            });
        }

        // Control button callbacks
        let this = Rc::new(RefCell::new(self.clone()));
        
        let this_update = this.clone();
        self.update_button.set_callback(move |_| {
            if let Ok(mut dialog) = this_update.try_borrow_mut() {
                dialog.calculate_statistics();
                dialog.update_display();
            }
        });

        let this_filter = this.clone();
        self.filter_button.set_callback(move |_| {
            if let Ok(mut dialog) = this_filter.try_borrow_mut() {
                dialog.show_filter_dialog();
            }
        });

        let this_export = this.clone();
        self.export_button.set_callback(move |_| {
            if let Ok(dialog) = this_export.try_borrow() {
                dialog.export_results();
            }
        });

        let mut window = self.window.clone();
        self.close_button.set_callback(move |_| {
            window.hide();
        });
    }

    fn calculate_statistics(&mut self) {
        if self.measurement_sets.is_empty() {
            self.batch_stats = None;
            return;
        }

        let count = self.measurement_sets.len();
        let units = self.measurement_sets[0].units.clone();

        // Calculate statistics
        let areas: Vec<f64> = self.measurement_sets.iter()
            .map(|m| m.area)
            .collect();

        let intensities: Vec<f64> = self.measurement_sets.iter()
            .map(|m| m.mean_intensity)
            .collect();

        self.batch_stats = Some(BatchStatistics {
            count,
            mean_area: Self::calculate_mean(&areas),
            std_dev_area: Self::calculate_std_dev(&areas),
            mean_intensity: Self::calculate_mean(&intensities),
            std_dev_intensity: Self::calculate_std_dev(&intensities),
            min_area: areas.iter().copied().fold(f64::INFINITY, f64::min),
            max_area: areas.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            min_intensity: intensities.iter().copied().fold(f64::INFINITY, f64::min),
            max_intensity: intensities.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            units,
        });
    }

    fn calculate_mean(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        values.iter().sum::<f64>() / values.len() as f64
    }

    fn calculate_std_dev(values: &[f64]) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }
        let mean = Self::calculate_mean(values);
        let variance = values.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / (values.len() - 1) as f64;
        variance.sqrt()
    }

    fn update_display(&mut self) {
        if let Some(stats) = &self.batch_stats {
            // Update statistics frames
            for frame in &mut self.stats_frames {
                match frame.label().as_ref() {
                    "Count" => frame.set_label(&format!("Count: {}", stats.count)),
                    "Mean Area" => frame.set_label(&format!(
                        "Mean Area: {:.2} {} (±{:.2})", 
                        stats.mean_area, 
                        stats.units,
                        stats.std_dev_area
                    )),
                    "Mean Intensity" => frame.set_label(&format!(
                        "Mean Intensity: {:.2} (±{:.2})",
                        stats.mean_intensity,
                        stats.std_dev_intensity
                    )),
                    "Min/Max" => frame.set_label(&format!(
                        "Area Range: {:.2} - {:.2} {}, Intensity Range: {:.2} - {:.2}",
                        stats.min_area,
                        stats.max_area,
                        stats.units,
                        stats.min_intensity,
                        stats.max_intensity
                    )),
                    _ => {}
                }
            }
        }

        // Update table display
        self.measurements_table.update_data(&self.measurement_sets);
    }

    pub fn add_measurement(&mut self, measurement: ROIMeasurements) {
        self.measurement_sets.push(measurement);
        if self.auto_update {
            self.calculate_statistics();
            self.update_display();
        }
    }

    fn show_filter_dialog(&mut self) {
        // TODO: Implement filtering dialog
    }

    fn export_results(&self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("CSV", &["csv"])
            .add_filter("Excel", &["xlsx"])
            .save_file() {
                self.export_to_file(&path);
        }
    }

    fn export_to_file(&self, path: &std::path::Path) {
        // TODO: Implement export functionality
    }

    pub fn show(&mut self) {
        self.window.show();
    }

    pub fn hide(&mut self) {
        self.window.hide();
    }

    pub fn is_visible(&self) -> bool {
        self.window.visible()
    }
}
pub fn show_batch_analysis_dialog(
    frame: &Rc<RefCell<Frame>>,
    state: &Rc<RefCell<ImageState>>,
    measurements: &Vec<CellMeasurement>
) {
    // Create a new window as parent
    let parent = Rc::new(RefCell::new(Window::default()));
    
    // Convert CellMeasurement to ROIMeasurements if needed
    let roi_measurements: Vec<ROIMeasurements> = Vec::new(); // Empty for now as conversion needs domain knowledge
    
    let mut dialog = BatchDialog::new(
        parent,
        roi_measurements,
        CellMeasurementMode::Single
    );
    dialog.show();
}