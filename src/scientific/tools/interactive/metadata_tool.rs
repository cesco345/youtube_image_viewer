use fltk::{
    enums::Event,
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
}

impl MetadataEditor {
    pub fn new() -> Self {
        let mut window = Window::default()
            .with_size(400, 300)
            .with_label("Metadata Editor");

        let mut pack = Pack::new(10, 10, 380, 280, "");
        pack.set_spacing(10);

        let exposure_input = FloatInput::new(120, 10, 100, 25, "Exposure (ms):");
        let gain_input = FloatInput::new(120, 45, 100, 25, "Gain:");
        let objective_input = Input::new(120, 80, 200, 25, "Objective:");
        let binning_input = Input::new(120, 115, 100, 25, "Binning:");
        let pixel_size_input = FloatInput::new(120, 150, 100, 25, "Pixel Size (μm):");
        let comments_input = Input::new(120, 185, 250, 25, "Comments:");

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
        }
    }

    pub fn show(&mut self, metadata: &Metadata) {
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
    
    editor.show(&current_metadata);

    let mut apply_btn = Button::new(10, 250, 70, 25, "Apply");
    let state_clone = state.clone();
    let frame_clone = frame.clone();
    
    apply_btn.set_callback(move |_| {
        if let Ok(mut state_ref) = state_clone.try_borrow_mut() {
            let new_metadata = editor.get_metadata();
            // Here you would apply the metadata to the current channel or image
            frame_clone.borrow_mut().redraw();
        }
    });
}