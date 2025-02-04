use fltk::{
    window::{Window, DoubleWindow}, 
    group::{Pack, Group,PackType},
    button::{Button, CheckButton, RadioRoundButton},
    frame::Frame,
    input::Input,
    enums::{Align, Color, FrameType},
    prelude::*,
};
use std::{rc::Rc, cell::RefCell, path::PathBuf};
use crate::scientific::{
    types::{ROIMeasurements, ROIShape},
    tools::interactive::roi::measurements::MeasurementCalculator,
};

#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    CSV,
    JSON,
    Excel,
    ImageROI,  // Exports ROI overlay on image
}

#[derive(Debug, Clone)]
pub struct ExportOptions {
    pub format: ExportFormat,
    pub include_measurements: bool,
    pub include_statistics: bool,
    pub include_images: bool,
    pub include_metadata: bool,
    pub batch_export: bool,
    pub export_path: PathBuf,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            format: ExportFormat::CSV,
            include_measurements: true,
            include_statistics: true,
            include_images: false,
            include_metadata: true,
            batch_export: false,
            export_path: PathBuf::new(),
        }
    }
}

pub struct ExportDialog {
    window: DoubleWindow,
    options: ExportOptions,
    path_input: Input,
    format_group: Group,
    content_group: Group,
    browse_btn: Button,
    export_btn: Button,
    cancel_btn: Button,
    csv_radio: RadioRoundButton,
    json_radio: RadioRoundButton,
    excel_radio: RadioRoundButton,
    image_radio: RadioRoundButton,
    measurements_check: CheckButton,
    statistics_check: CheckButton,
    images_check: CheckButton,
    metadata_check: CheckButton,
    batch_check: CheckButton,
}

impl ExportDialog {
    pub fn new(parent: Rc<RefCell<Window>>) -> Self {
        let mut window = DoubleWindow::new(
            0, 0, 400, 500,
            "Export ROI Data"
        );

        // Center dialog
        let parent = parent.borrow();
        let x = parent.x() + (parent.width() - window.width()) / 2;
        let y = parent.y() + (parent.height() - window.height()) / 2;
        window.set_pos(x, y);

        let mut pack = Pack::new(10, 10, 380, 480, None);
        pack.set_spacing(10);

        // Export path selection
        let mut path_frame = Frame::new(0, 0, 380, 25, Some("Export Path"));
        path_frame.set_align(Align::Left | Align::Inside);
        
        let mut path_group = Pack::new(0, 0, 380, 30, None);
        path_group.set_type(PackType::Horizontal);
        path_group.set_spacing(5);
        
        let mut path_input = Input::new(0, 0, 300, 30, None);
        let mut browse_btn = Button::new(0, 0, 75, 30, Some("Browse"));
        path_group.end();

        // Format selection
        let mut format_frame = Frame::new(0, 0, 380, 25, Some("Export Format"));
        format_frame.set_align(Align::Left | Align::Inside);
        
        let mut format_group = Group::new(0, 0, 380, 120, None);
        format_group.set_frame(FrameType::EngravedBox);
        
        let mut csv_radio = RadioRoundButton::new(20, 10, 160, 25, Some("CSV (.csv)"));
        let mut json_radio = RadioRoundButton::new(20, 35, 160, 25, Some("JSON (.json)"));
        let mut excel_radio = RadioRoundButton::new(20, 60, 160, 25, Some("Excel (.xlsx)"));
        let mut image_radio = RadioRoundButton::new(20, 85, 160, 25, Some("Image with ROI (.png)"));
        image_radio.deactivate(); // Initially deactivated
        
        csv_radio.set_value(true);
        format_group.end();

        // Content options
        let mut content_frame = Frame::new(0, 0, 380, 25, Some("Export Content"));
        content_frame.set_align(Align::Left | Align::Inside);
        
        let mut content_group = Group::new(0, 0, 380, 150, None);
        content_group.set_frame(FrameType::EngravedBox);
        
        let mut measurements_check = CheckButton::new(20, 10, 340, 25, Some("Include Measurements"));
        let mut statistics_check = CheckButton::new(20, 35, 340, 25, Some("Include Statistics"));
        let mut images_check = CheckButton::new(20, 60, 340, 25, Some("Include Images"));
        let mut metadata_check = CheckButton::new(20, 85, 340, 25, Some("Include Metadata"));
        let mut batch_check = CheckButton::new(20, 110, 340, 25, Some("Batch Export"));
        
        measurements_check.set_checked(true);
        statistics_check.set_checked(true);
        metadata_check.set_checked(true);
        content_group.end();

        // Action buttons
        let mut button_group = Pack::new(0, 0, 380, 30, None);
        button_group.set_type(PackType::Horizontal);
        button_group.set_spacing(5);
        
        let mut export_btn = Button::new(0, 0, 185, 30, Some("Export"));
        let mut cancel_btn = Button::new(0, 0, 185, 30, Some("Cancel"));
        button_group.end();

        pack.end();
        window.end();
        window.make_modal(true);

        let mut dialog = Self {
            window,
            options: ExportOptions::default(),
            path_input,
            format_group,
            content_group,
            browse_btn,
            export_btn,
            cancel_btn,
            csv_radio,
            json_radio,
            excel_radio,
            image_radio,
            measurements_check,
            statistics_check,
            images_check,
            metadata_check,
            batch_check,
        };

        dialog.setup_callbacks();
        dialog
    }

    fn setup_callbacks(&mut self) {
        let mut path_input = self.path_input.clone();
        let mut window = self.window.clone();
        let image_radio = self.image_radio.clone();
        
        // Update image radio button based on images checkbox
        let mut images_check = self.images_check.clone();
        let mut image_radio = self.image_radio.clone();
        images_check.set_callback(move |check| {
            if check.value() {
                image_radio.activate();
            } else {
                image_radio.deactivate();
                if image_radio.value() {
                    image_radio.set_value(false);
                }
            }
        });

        self.browse_btn.set_callback(move |_| {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("CSV", &["csv"])
                .add_filter("JSON", &["json"])
                .add_filter("Excel", &["xlsx"])
                .add_filter("PNG", &["png"])
                .save_file() {
                    path_input.set_value(&path.to_string_lossy());
            }
        });

        self.export_btn.set_callback({
            let mut dialog = self.clone();
            move |_| {
                dialog.update_options();
                if dialog.validate_export() {
                    dialog.perform_export();
                    dialog.window.hide();
                }
            }
        });

        self.cancel_btn.set_callback(move |_| {
            window.hide();
        });
    }

    fn update_options(&mut self) {
        self.options.export_path = PathBuf::from(self.path_input.value());
        
        // Update format based on radio selection
        self.options.format = if self.csv_radio.value() {
            ExportFormat::CSV
        } else if self.json_radio.value() {
            ExportFormat::JSON
        } else if self.excel_radio.value() {
            ExportFormat::Excel
        } else if self.image_radio.value() {
            ExportFormat::ImageROI
        } else {
            ExportFormat::CSV // Default
        };

        // Update content options
        self.options.include_measurements = self.measurements_check.value();
        self.options.include_statistics = self.statistics_check.value();
        self.options.include_images = self.images_check.value();
        self.options.include_metadata = self.metadata_check.value();
        self.options.batch_export = self.batch_check.value();
    }

    fn validate_export(&self) -> bool {
        if self.path_input.value().is_empty() {
            fltk::dialog::alert_default("Please select an export path");
            return false;
        }
        true
    }

    fn perform_export(&self) {
        match self.options.format {
            ExportFormat::CSV => self.export_csv(),
            ExportFormat::JSON => self.export_json(),
            ExportFormat::Excel => self.export_excel(),
            ExportFormat::ImageROI => self.export_image_roi(),
        }
    }

    fn export_csv(&self) {
        // TODO: Implement CSV export
    }

    fn export_json(&self) {
        // TODO: Implement JSON export
    }

    fn export_excel(&self) {
        // TODO: Implement Excel export
    }

    fn export_image_roi(&self) {
        // TODO: Implement image with ROI overlay export
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

impl Clone for ExportDialog {
    fn clone(&self) -> Self {
        Self {
            window: self.window.clone(),
            options: self.options.clone(),
            path_input: self.path_input.clone(),
            format_group: self.format_group.clone(),
            content_group: self.content_group.clone(),
            browse_btn: self.browse_btn.clone(),
            export_btn: self.export_btn.clone(),
            cancel_btn: self.cancel_btn.clone(),
            csv_radio: self.csv_radio.clone(),
            json_radio: self.json_radio.clone(),
            excel_radio: self.excel_radio.clone(),
            image_radio: self.image_radio.clone(),
            measurements_check: self.measurements_check.clone(),
            statistics_check: self.statistics_check.clone(),
            images_check: self.images_check.clone(),
            metadata_check: self.metadata_check.clone(),
            batch_check: self.batch_check.clone(),
        }
    }
}