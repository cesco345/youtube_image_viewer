// convolution_tool.rs
use fltk::{
    dialog::{alert, choice2},
    frame::Frame,
    prelude::*,
    enums::{Color, Event},
    app,
    draw,
};
use std::{cell::RefCell, rc::Rc};
use crate::state::ImageState;
use crate::menu::edit::crop::crop_tool::CropSelection;
use super::advanced::ConvolutionType; 
use super::advanced::ConvolutionFilter;

pub fn start_interactive_convolution(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    conv_type: ConvolutionType
) {
    let mut state_ref = state.borrow_mut();
    if state_ref.image.is_none() {
        alert(300, 300, "Please open an image first");
        return;
    }
    
    let original_image = state_ref.image.clone();
    
    // initialize the crop selection functionality for convolution area
    if let Some(img) = &original_image {
        let frame_ref = frame.borrow();
        state_ref.crop_selection = Some(CropSelection::new(
            img.data_w(),
            img.data_h(),
            frame_ref.w(),
            frame_ref.h()
        ));
    }
    drop(state_ref);

    let frame_clone = frame.clone();
    let state_clone = state.clone();
    let mut frame = frame.borrow_mut();

    // get the filter name based on the convolution type
    let filter_name = match conv_type {
        ConvolutionType::GaussianBlur { .. } => "Gaussian Blur",
        ConvolutionType::BoxBlur { .. } => "Box Blur",
        ConvolutionType::Sharpen { .. } => "Sharpen",
    };

    let draw_callback = {
        let state_clone = state_clone.clone();
        let original_image = original_image.clone();
        move |f: &mut Frame| {
            if let Some(img) = &original_image {
                f.set_image(Some(img.clone()));

                let dimensions = state_clone
                    .try_borrow()
                    .ok()
                    .and_then(|state_ref| state_ref.crop_selection.as_ref().map(|s| (s.is_selecting, s.get_dimensions())));
                
                if let Some((true, (x, y, w, h))) = dimensions {
                    // draw selection rectangle
                    draw::set_draw_color(Color::Yellow);
                    draw::set_line_style(draw::LineStyle::Solid, 2);
                    draw::draw_rect(x, y, w, h);
                    
                    // draw pattern inside selection
                    draw::set_line_style(draw::LineStyle::Dot, 1);
                    let pattern_size = 10;
                    
                    // draw crosshatch pattern
                    for i in (x..x + w).step_by(pattern_size) {
                        draw::draw_line(i, y, i, y + h);
                    }
                    for i in (y..y + h).step_by(pattern_size) {
                        draw::draw_line(x, i, x + w, i);
                    }
                }
            }
        }
    };

    frame.draw(draw_callback);

    let handle_callback = {
        let state_clone = state_clone.clone();
        let frame_clone = frame_clone.clone();
        let conv_type = conv_type;
        let filter_name = filter_name.to_string();
        
        move |f: &mut Frame, ev: Event| -> bool {
            match ev {
                Event::Push => {
                    if let Ok(mut state) = state_clone.try_borrow_mut() {
                        if let Some(selection) = &mut state.crop_selection {
                            selection.reset();
                            selection.start_x = app::event_x();
                            selection.start_y = app::event_y();
                            selection.is_selecting = true;
                            f.redraw();
                        }
                    }
                    true
                },
                Event::Drag => {
                    if let Ok(mut state) = state_clone.try_borrow_mut() {
                        if let Some(selection) = &mut state.crop_selection {
                            selection.end_x = app::event_x();
                            selection.end_y = app::event_y();
                            f.redraw();
                        }
                    }
                    true
                },
                Event::Released => {
                    let should_apply = {
                        let mut dimensions = None;
                        if let Ok(mut state) = state_clone.try_borrow_mut() {
                            if let Some(selection) = &mut state.crop_selection {
                                selection.is_selecting = false;
                                selection.end_x = app::event_x();
                                selection.end_y = app::event_y();
                                dimensions = Some(selection.get_dimensions());
                            }
                        }
                        
                        if let Some((_, _, w, h)) = dimensions {
                            w > 5 && h > 5 && choice2(300, 300, &format!("Apply {} to selected area?", filter_name), "Yes", "No", "") == Some(0)
                        } else {
                            false
                        }
                    };

                    if should_apply {
                        if let Ok(mut state) = state_clone.try_borrow_mut() {
                            if let (Some(selection), Some(current_image)) = (state.crop_selection.as_ref(), &state.image) {
                                let filter = match conv_type {
                                    ConvolutionType::GaussianBlur { radius, sigma } => {
                                        ConvolutionFilter::new_gaussian_blur(radius, sigma)
                                    },
                                    ConvolutionType::BoxBlur { radius } => {
                                        ConvolutionFilter::new_box_blur(radius)
                                    },
                                    ConvolutionType::Sharpen { intensity } => {
                                        ConvolutionFilter::new_sharpen(intensity)
                                    },
                                }.with_selection(selection.clone())
                                 .with_feather(5)
                                 .with_intensity(1.0);
                                
                                if let Ok(Some(new_image)) = state.filter_state.apply_filter(current_image, &filter) {
                                    state.image = Some(new_image.clone());
                                    frame_clone.borrow_mut().set_image(state.image.clone());
                                }
                            }
                            state.crop_selection = None;
                        }
                    } else {
                        if let Ok(mut state) = state_clone.try_borrow_mut() {
                            state.crop_selection = None;
                        }
                    }
                    
                    f.redraw();
                    true
                },
                _ => false,
            }
        }
    };

    frame.handle(handle_callback);
}