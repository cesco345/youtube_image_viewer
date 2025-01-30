use fltk::{
    prelude::*,
    menu::{MenuBar, MenuFlag},
    enums::{Shortcut, Event},
    frame::Frame,
    app,
};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;
use crate::scientific::{
    types::{ CellMeasurementMode, ROITool },
    tools::interactive,
    ui::cell_analysis::{
        dialog::show_cell_analysis_dialog,
        statistics::show_statistics_dialog,
        export::export_batch_measurements,
    }
};
// Add this import
use crate::scientific::tools::interactive::cell_analysis_tool::CellAnalysisState;

pub fn setup_cell_analysis_menu(menu: &mut MenuBar, frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    let frame_start = frame.clone();
    let state_start = state.clone();
    menu.add(
        "&Scientific/&Analysis/Cell Analysis/Start Analysis",
        Shortcut::None,
        MenuFlag::Normal,
        move |_| {
            println!("Starting cell analysis...");
            
            // Clone the image first, outside the mutable borrow
            let image_clone = if let Ok(state_ref) = state_start.try_borrow() {
                println!("Got state reference");
                state_ref.image.clone()
            } else {
                println!("Failed to borrow state");
                None
            };
        
            // Now do the mutable operations with the cloned image
            if let Some(img) = image_clone {
                println!("Got image, initializing analysis");
                if let Ok(mut state_ref) = state_start.try_borrow_mut() {
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
                interactive::start_interactive_roi(&frame_start, &state_start);
            } else {
                println!("No image available");
            }
            
            frame_start.borrow_mut().redraw();
        }
    );

    // Batch analysis menu item
    let frame_batch = frame.clone();
    let state_batch = state.clone();
    menu.add(
        "&Scientific/&Analysis/Cell Analysis/Start Batch Analysis",
        Shortcut::None,
        MenuFlag::Normal,
        move |_| {
            println!("Starting batch analysis...");
            if let Ok(mut state_ref) = state_batch.try_borrow_mut() {
                if let Some(img) = state_ref.image.clone() {
                    let calibration_scale = state_ref.scientific_state.calibration.pixels_per_unit as f64;
                    let unit = state_ref.scientific_state.calibration.unit.clone();
                    
                    state_ref.scientific_state.store_base_image(img);
                    state_ref.scientific_state.init_cell_analysis(calibration_scale, unit);
                    state_ref.scientific_state.start_cell_analysis(CellMeasurementMode::Batch);
                }
            }
            interactive::start_interactive_roi(&frame_batch, &state_batch);
            frame_batch.borrow_mut().redraw();
        }
    );

    // Statistics menu item
    let state_stats = state.clone();
    menu.add(
        "&Scientific/&Analysis/Cell Analysis/Show Statistics",
        Shortcut::None,
        MenuFlag::Normal,
        move |_| {
            println!("Showing statistics...");
            if let Ok(state_ref) = state_stats.try_borrow() {
                if let Some(measurements) = state_ref.scientific_state.get_measurements() {
                    let measurements_vec = measurements.to_vec();
                    show_statistics_dialog(&measurements_vec);
                }
            }
        }
    );

    // Export menu item
    let state_export = state.clone();
    menu.add(
        "&Scientific/&Analysis/Cell Analysis/Export Analysis",
        Shortcut::None,
        MenuFlag::Normal,
        move |_| {
            println!("Exporting analysis...");
            if let Ok(state_ref) = state_export.try_borrow() {
                if let Some(measurements) = state_ref.scientific_state.get_measurements() {
                    let measurements_vec = measurements.to_vec();
                    export_batch_measurements(&state_ref, &measurements_vec);
                }
            }
        }
    );

    // Stop analysis menu item
    let frame_stop = frame.clone();
    let state_stop = state.clone();
    menu.add(
        "&Scientific/&Analysis/Cell Analysis/Stop Analysis",
        Shortcut::None,
        MenuFlag::Normal,
        move |_| {
            println!("Stopping analysis...");
            if let Ok(mut state_ref) = state_stop.try_borrow_mut() {
                state_ref.scientific_state.stop_cell_analysis();
            }
            frame_stop.borrow_mut().redraw();
        }
    );

    // Event handling setup
    let state_events = state.clone();
    let frame_events = frame.clone();
    frame.borrow_mut().handle(move |_, event| {
        if let Ok(mut state_ref) = state_events.try_borrow_mut() {
            if !state_ref.scientific_state.is_analyzing_cells() {
                return false;
            }

            match event {
                Event::Push | Event::Drag | Event::Released => {
                    let coords = app::event_coords();
                    
                    // Store initial state values we need
                    let is_analyzing = state_ref.scientific_state.is_analyzing_cells();
                    let (width, height) = if let Some(ref img) = state_ref.image {
                        (img.data_w(), img.data_h())
                    } else {
                        (1, 1)
                    };

                    let (event_handled, roi_shape_opt, points) = {
                        if let Some(cell_tool) = &mut state_ref.scientific_state.cell_analysis_tool {
                            cell_tool.handle_event(event, coords)
                        } else {
                            (false, None, Vec::new())
                        }
                    };

                    // Store ROI updates if available
                    if let Some(roi_shape) = roi_shape_opt {
                        let roi_tool = ROITool::new(
                            roi_shape,
                            (0, 255, 0),
                            2
                        );
                        state_ref.scientific_state.set_roi_tool(roi_tool);
                    }

                    // Process measurement on release
                    if event == Event::Released && !points.is_empty() {
                        println!("=== ROI Released Event Processing ===");
                        println!("Points collected: {}", points.len());
                        
                        // Get intensity profile
                        let profile = state_ref.scientific_state.get_roi_intensity_profile(&points);
                        println!("Intensity profile generated: {}", profile.is_some());
                        
                        if let Some(prof) = profile {
                            println!("Processing intensity profile");
                            
                            // Drop state_ref temporarily to avoid borrow checker issues
                            drop(state_ref);
                            
                            // Re-borrow to process the measurement
                            if let Ok(mut state_ref) = state_events.try_borrow_mut() {
                                if let Some(cell_tool) = &mut state_ref.scientific_state.cell_analysis_tool {
                                    println!("Cell tool found, processing measurement");
                                    if let Some((measurement, annotation)) = cell_tool.process_intensity_profile(
                                        prof, 
                                        &points,
                                        width,
                                        height
                                    ) {
                                        println!("Measurement generated:");
                                        println!("  Area: {}", measurement.area);
                                        println!("  Perimeter: {}", measurement.perimeter);
                                        println!("  Circularity: {}", measurement.circularity);
                                        
                                        let mode = state_ref.scientific_state.get_measurement_mode();
                                        println!("Current measurement mode: {:?}", mode);
                                        
                                        // Drop borrow before showing dialog
                                        let measurement_clone = measurement.clone();
                                        drop(state_ref);
                                        
                                        if mode == CellMeasurementMode::Single {
                                            println!("Showing single measurement dialog");
                                            show_cell_analysis_dialog(&frame_events, &state_events, &measurement_clone);
                                            println!("Dialog should be visible now");
                                        } else {
                                            println!("Batch mode - not showing dialog");
                                        }
                                        
                                        // Re-borrow to update state
                                        if let Ok(mut state_ref) = state_events.try_borrow_mut() {
                                            println!("Adding measurement and annotation to state");
                                            state_ref.scientific_state.add_cell_measurement(measurement);
                                            state_ref.scientific_state.add_annotation(annotation);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    frame_events.borrow_mut().redraw();
                    event_handled
                }
                _ => false
            }
        } else {
            false
        }
    });
}
