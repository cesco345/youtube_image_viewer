use fltk::{
    window::{Window, DoubleWindow},
    group::{Pack, Scroll},
    button::Button,
    frame::Frame,
    enums::{Color},
    prelude::*,
};
use std::{rc::Rc, cell::RefCell};
use crate::scientific::{
    types::ROIMeasurements,
    tools::interactive::roi::measurements::MeasurementCalculator,
};
use super::components::measurement_table::MeasurementTable;

pub struct MeasurementDialog {
    window: DoubleWindow,
    measurements: ROIMeasurements,
    table: MeasurementTable,
    export_btn: Button,
    close_btn: Button,
}

impl MeasurementDialog {
    pub fn new(
        parent: Rc<RefCell<Window>>,
        measurements: ROIMeasurements,
    ) -> Self {
        let mut window = DoubleWindow::new(
            0, 0, 400, 500,
            "ROI Measurements"
        );

        // Center dialog
        let parent = parent.borrow();
        let x = parent.x() + (parent.width() - window.width()) / 2;
        let y = parent.y() + (parent.height() - window.height()) / 2;
        window.set_pos(x, y);

        let mut pack = Pack::new(10, 10, 380, 480, "");
        pack.set_spacing(10);

        // Header with basic info
        let mut header = Frame::new(0, 0, 380, 30, "Measurement Results");
        header.set_label_size(16);
        header.set_label_color(Color::Dark3);

        // Create scrollable measurement table
        let mut scroll = Scroll::new(0, 0, 380, 380, "");
        let table = MeasurementTable::new(
            0, 0, 380, 380,
            vec![measurements.clone()]
        );
        scroll.end();

        // Statistics summary
        let mut stats = Frame::new(0, 0, 380, 25, "");
        stats.set_label(&format!(
            "Area: {:.2} {}², Mean Intensity: {:.2}",
            measurements.area,
            measurements.units,
            measurements.mean_intensity
        ));

        // Create buttons
        let mut export_btn = Button::new(0, 0, 185, 25, "Export Data");
        let mut close_btn = Button::new(195, 0, 185, 25, "Close");

        pack.end();
        window.end();
        window.make_modal(true);

        let mut dialog = Self {
            window,
            measurements,
            table,
            export_btn,
            close_btn,
        };

        dialog.setup_callbacks();
        dialog
    }

    fn setup_callbacks(&mut self) {
        let measurements = self.measurements.clone();
        let mut window = self.window.clone();

        self.export_btn.set_callback(move |_| {
            // Implement export functionality
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("CSV", &["csv"])
                .save_file() {
                    Self::export_measurements(&measurements, &path);
            }
        });

        self.close_btn.set_callback(move |_| {
            window.hide();
        });
    }

    fn export_measurements(measurements: &ROIMeasurements, path: &std::path::Path) {
        use std::io::Write;
        if let Ok(mut file) = std::fs::File::create(path) {
            writeln!(file, "Measurement,Value,Units").unwrap();
            writeln!(file, "Area,{:.2},{}", measurements.area, measurements.units).unwrap();
            writeln!(file, "Perimeter,{:.2},{}", measurements.perimeter, measurements.units).unwrap();
            writeln!(file, "Mean Intensity,{:.2},", measurements.mean_intensity).unwrap();
            writeln!(file, "Min Intensity,{:.2},", measurements.min_intensity).unwrap();
            writeln!(file, "Max Intensity,{:.2},", measurements.max_intensity).unwrap();
            writeln!(file, "Integrated Density,{:.2},", measurements.integrated_density).unwrap();
            writeln!(file, "Circularity,{:.2},", measurements.circularity).unwrap();
            writeln!(file, "Aspect Ratio,{:.2},", measurements.aspect_ratio).unwrap();
        }
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