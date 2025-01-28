use fltk::{
    enums::{Event, Color, Align},
    frame::Frame,
    window::Window,
    group::Pack,
    input::{Input, FloatInput},
    button::Button,
    prelude::*,
};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;
use crate::scientific::layers::Metadata;
use chrono::Utc;

pub struct MetadataEditor {
    window: Window,
    exposure_input: FloatInput,
    gain_input: FloatInput,
    objective_input: Input,
    binning_input: Input,
    pixel_size_input: FloatInput,
    comments_input: Input,
    scale_info_frame: Frame,
}

impl MetadataEditor {
    pub fn new() -> Self {
        let mut window = Window::default()
            .with_size(400, 350)
            .with_label("Metadata Editor");

        let mut pack = Pack::new(10, 10, 380, 330, "");
        pack.set_spacing(10);

        let mut exposure_label = Frame::new(10, 10, 110, 25, "Exposure (ms):");
        exposure_label.set_label_color(Color::White);
        let mut exposure_input = FloatInput::new(120, 10, 100, 25, "");
        exposure_input.set_color(Color::Dark3);
        exposure_input.set_text_color(Color::White);

        let mut gain_label = Frame::new(10, 45, 110, 25, "Gain:");
        gain_label.set_label_color(Color::White);
        let mut gain_input = FloatInput::new(120, 45, 100, 25, "");
        gain_input.set_color(Color::Dark3);
        gain_input.set_text_color(Color::White);

        let mut objective_label = Frame::new(10, 80, 110, 25, "Objective:");
        objective_label.set_label_color(Color::White);
        let mut objective_input = Input::new(120, 80, 200, 25, "");
        objective_input.set_color(Color::Dark3);
        objective_input.set_text_color(Color::White);

        let mut binning_label = Frame::new(10, 115, 110, 25, "Binning:");
        binning_label.set_label_color(Color::White);
        let mut binning_input = Input::new(120, 115, 100, 25, "");
        binning_input.set_color(Color::Dark3);
        binning_input.set_text_color(Color::White);

        let mut pixel_size_label = Frame::new(10, 150, 110, 25, "Pixel Size (μm):");
        pixel_size_label.set_label_color(Color::White);
        let mut pixel_size_input = FloatInput::new(120, 150, 100, 25, "");
        pixel_size_input.set_color(Color::Dark3);
        pixel_size_input.set_text_color(Color::White);

        let mut comments_label = Frame::new(10, 185, 110, 25, "Comments:");
        comments_label.set_label_color(Color::White);
        let mut comments_input = Input::new(120, 185, 250, 25, "");
        comments_input.set_color(Color::Dark3);
        comments_input.set_text_color(Color::White);

        let mut scale_info_frame = Frame::new(10, 220, 380, 60, "");
        scale_info_frame.set_label_color(Color::White);
        scale_info_frame.set_align(Align::Left | Align::Inside);

        pack.end();
        window.end();

        Self {
            window,
            exposure_input,
            gain_input,
            objective_input,
            binning_input,
            pixel_size_input,
            comments_input,
            scale_info_frame,
        }
    }

    pub fn show(&mut self, metadata: &Metadata, state: &ImageState) {
        if let Some(exposure) = metadata.exposure_time {
            self.exposure_input.set_value(&exposure.to_string());
        }
        if let Some(gain) = metadata.gain {
            self.gain_input.set_value(&gain.to_string());
        }
        if let Some(ref obj) = metadata.objective {
            self.objective_input.set_value(obj);
        }
        if let Some(binning) = metadata.binning {
            self.binning_input.set_value(&binning.to_string());
        }
        if let Some(pixel_size) = metadata.pixel_size {
            self.pixel_size_input.set_value(&pixel_size.to_string());
        }
        if let Some(ref comments) = metadata.comments {
            self.comments_input.set_value(comments);
        }

        // Update scale information
        let scale_info = format!(
            "Scale: {:.2} pixels/{}\nCalibration: {}",
            state.scientific_state.calibration.pixels_per_unit,
            state.scientific_state.calibration.unit,
            metadata.scale_calibration.as_ref()
                .map(|(ppu, unit)| format!("{:.2} pixels/{}", ppu, unit))
                .unwrap_or_else(|| "Not calibrated".to_string())
        );
        self.scale_info_frame.set_label(&scale_info);

        self.window.show();
    }

    pub fn get_metadata(&self) -> Metadata {
        Metadata {
            acquisition_time: Some(Utc::now()),
            exposure_time: self.exposure_input.value().parse().ok(),
            gain: self.gain_input.value().parse().ok(),
            objective: Some(self.objective_input.value()),
            binning: self.binning_input.value().parse().ok(),
            pixel_size: self.pixel_size_input.value().parse().ok(),
            comments: Some(self.comments_input.value()),
            scale_calibration: None,
        }
    }
}

pub fn start_metadata_editor(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    let mut editor = MetadataEditor::new();
    let state_ref = state.borrow();
    let current_metadata = Metadata::default();
    
    editor.show(&current_metadata, &state_ref);

    let mut apply_btn = Button::new(10, 290, 70, 25, "Apply");
    apply_btn.set_color(Color::Dark3);
    apply_btn.set_label_color(Color::White);
    
    let state_clone = state.clone();
    let frame_clone = frame.clone();
    
    apply_btn.set_callback(move |_| {
        if let Ok(mut state_ref) = state_clone.try_borrow_mut() {
            let new_metadata = editor.get_metadata();
            frame_clone.borrow_mut().redraw();
        }
    });
}