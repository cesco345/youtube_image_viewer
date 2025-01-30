use fltk::{prelude::*, frame::Frame}; 
use std::{rc::Rc, cell::RefCell}; 
use crate::{
    state::ImageState,
    scientific::{
        ui::cell_analysis::{
            dialog::show_cell_analysis_dialog,
            statistics::show_statistics_dialog,
            export::export_batch_measurements,
        },
        types::CellMeasurementMode,
        tools::interactive::cell_analysis_tool::CellAnalysisState,
    }
};

pub fn handle_cell_analysis(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        if !state_ref.image.is_some() {
            return;
        }

        let calibration_scale = state_ref.scientific_state.calibration.pixels_per_unit as f64;
        let unit = state_ref.scientific_state.calibration.unit.clone();
        
        state_ref.scientific_state.init_cell_analysis(calibration_scale, unit);
        state_ref.scientific_state.start_cell_analysis(CellMeasurementMode::Single);
    }
    frame.borrow_mut().redraw();
}

pub fn handle_batch_analysis(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        if !state_ref.image.is_some() {
            return;
        }

        let calibration_scale = state_ref.scientific_state.calibration.pixels_per_unit as f64;
        let unit = state_ref.scientific_state.calibration.unit.clone();
        
        state_ref.scientific_state.init_cell_analysis(calibration_scale, unit);
        state_ref.scientific_state.start_cell_analysis(CellMeasurementMode::Batch);
    }
    frame.borrow_mut().redraw();
}

pub fn handle_show_statistics(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    if let Ok(state_ref) = state.try_borrow() {
        if let Some(measurements) = state_ref.scientific_state.get_measurements() {
            // Convert measurements to Vec before passing
            let measurements_vec = measurements.to_vec();
            show_statistics_dialog(&measurements_vec);
        }
    }
}

pub fn handle_export_analysis(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    if let Ok(state_ref) = state.try_borrow() {
        if let Some(measurements) = state_ref.scientific_state.get_measurements() {
            // Convert measurements to Vec before passing
            let measurements_vec = measurements.to_vec();
            export_batch_measurements(&state_ref, &measurements_vec);
        }
    }
}

pub fn handle_stop_analysis(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        state_ref.scientific_state.stop_cell_analysis();
    }
    frame.borrow_mut().redraw();
}

