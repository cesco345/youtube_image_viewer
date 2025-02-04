//src/menu/scientific/analysis/handlers.rs
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
        tools::interactive::{
            cell_analysis_tool::CellAnalysisState,
            roi_tool::start_interactive_roi
        },
    }
};


pub fn handle_cell_analysis(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    println!("Starting cell analysis...");
    
    // Clone the image first, outside the mutable borrow
    let image_clone = if let Ok(state_ref) = state.try_borrow() {
        println!("Got state reference");
        state_ref.image.clone()
    } else {
        println!("Failed to borrow state");
        None
    };

    // Now do the mutable operations with the cloned image
    if let Some(img) = image_clone {
        println!("Got image, initializing analysis");
        if let Ok(mut state_ref) = state.try_borrow_mut() {
            let calibration_scale = state_ref.scientific_state.calibration.pixels_per_unit as f64;
            let unit = state_ref.scientific_state.calibration.unit.clone();
            
            println!("Storing base image and creating channel");
            // Store base image - this will also create the channel
            state_ref.scientific_state.store_base_image(img);
            
            println!("Initializing cell analysis");
            // Initialize cell analysis using the trait method
            state_ref.scientific_state.init_cell_analysis(calibration_scale, unit);
            state_ref.scientific_state.start_cell_analysis(CellMeasurementMode::Single);
        }
        
        // Start ROI tool
        start_interactive_roi(frame, state);
    } else {
        println!("No image available");
    }
    
    frame.borrow_mut().redraw();
}

pub fn handle_batch_analysis(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    println!("Starting batch analysis...");
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        if let Some(img) = state_ref.image.clone() {
            let calibration_scale = state_ref.scientific_state.calibration.pixels_per_unit as f64;
            let unit = state_ref.scientific_state.calibration.unit.clone();
            
            state_ref.scientific_state.store_base_image(img);
            state_ref.scientific_state.init_cell_analysis(calibration_scale, unit);
            state_ref.scientific_state.start_cell_analysis(CellMeasurementMode::Batch);

            // Start ROI tool after configuration
            drop(state_ref); // Drop mutable borrow before starting ROI tool
            start_interactive_roi(frame, state);
        }
    }
    frame.borrow_mut().redraw();
}

pub fn handle_show_statistics(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    println!("Showing statistics...");
    if let Ok(state_ref) = state.try_borrow() {
        if let Some(measurements) = state_ref.scientific_state.get_measurements() {
            let measurements_vec = measurements.to_vec();
            if !measurements_vec.is_empty() {
                show_statistics_dialog(&measurements_vec);
            } else {
                println!("No measurements available");
            }
        }
    }
}

pub fn handle_export_analysis(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    println!("Exporting analysis...");
    if let Ok(state_ref) = state.try_borrow() {
        if let Some(measurements) = state_ref.scientific_state.get_measurements() {
            let measurements_vec = measurements.to_vec();
            if !measurements_vec.is_empty() {
                export_batch_measurements(&state_ref, &measurements_vec);
            } else {
                println!("No measurements to export");
            }
        }
    }
}

pub fn handle_stop_analysis(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    println!("Stopping cell analysis...");
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        state_ref.scientific_state.stop_cell_analysis();
    }
    frame.borrow_mut().redraw();
}

