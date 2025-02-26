// src/menu/edit/filters/handlers.rs

// Standard library imports
use std::{rc::Rc, cell::RefCell};

// FLTK imports
use fltk::{
    frame::Frame, 
    prelude::*,
};

// Internal state imports
use crate::state::ImageState;

// Basic filters
use super::basic::{
    GrayscaleFilter, 
    SepiaFilter,
    BrightnessFilter,
    ContrastFilter,
    SaturationFilter,
    ThresholdFilter,
    HueFilter,
};

// Advanced filters
use super::advanced::{
    EdgeDetectionFilter,
    EdgeDetectionMethod,
    NoiseFilter,
    VignetteFilter,
    PosterizeFilter,
    PixelateFilter,
    MotionBlurFilter,
    ConvolutionType,
};

// Interactive tools
use super::{
    start_interactive_pixelate,
    start_interactive_convolution,
    start_interactive_edge_detection,
    start_interactive_noise,

};


pub fn handle_apply_grayscale(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    intensity: f32
) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        println!("Starting grayscale filter application");
        
        let current_image = if let Some(img) = &state_ref.image {
            println!("Found current image in state");
            img.clone()
        } else {
            println!("No image found in state");
            return;
        };

        let filter = GrayscaleFilter::new(intensity);
        
        if let Ok(Some(new_image)) = state_ref.filter_state.apply_filter(&current_image, &filter) {
            println!("Successfully applied grayscale filter");
            frame.borrow_mut().set_image(Some(new_image.clone()));
            frame.borrow_mut().redraw();
            state_ref.image = Some(new_image);
        }
    }
}

pub fn handle_apply_sepia(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    intensity: f32
) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        let current_image = if let Some(img) = &state_ref.image {
            img.clone()
        } else {
            return;
        };

        let filter = SepiaFilter::new(intensity);
        
        if let Ok(Some(new_image)) = state_ref.filter_state.apply_filter(&current_image, &filter) {
            frame.borrow_mut().set_image(Some(new_image.clone()));
            frame.borrow_mut().redraw();
            state_ref.image = Some(new_image);
        }
    }
}

pub fn handle_apply_brightness(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    intensity: f32
) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        let current_image = if let Some(img) = &state_ref.image {
            img.clone()
        } else {
            return;
        };

        let filter = BrightnessFilter::new(intensity);
        
        if let Ok(Some(new_image)) = state_ref.filter_state.apply_filter(&current_image, &filter) {
            frame.borrow_mut().set_image(Some(new_image.clone()));
            frame.borrow_mut().redraw();
            state_ref.image = Some(new_image);
        }
    }
}

pub fn handle_apply_contrast(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    intensity: f32
) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        let current_image = if let Some(img) = &state_ref.image {
            img.clone()
        } else {
            return;
        };

        let filter = ContrastFilter::new(intensity);
        
        if let Ok(Some(new_image)) = state_ref.filter_state.apply_filter(&current_image, &filter) {
            frame.borrow_mut().set_image(Some(new_image.clone()));
            frame.borrow_mut().redraw();
            state_ref.image = Some(new_image);
        }
    }
}

pub fn handle_apply_saturation(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    intensity: f32
) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        let current_image = if let Some(img) = &state_ref.image {
            img.clone()
        } else {
            return;
        };

        let filter = SaturationFilter::new(intensity);
        
        if let Ok(Some(new_image)) = state_ref.filter_state.apply_filter(&current_image, &filter) {
            frame.borrow_mut().set_image(Some(new_image.clone()));
            frame.borrow_mut().redraw();
            state_ref.image = Some(new_image);
        }
    }
}



pub fn handle_apply_threshold(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    threshold: f32
) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        let current_image = if let Some(img) = &state_ref.image {
            img.clone()
        } else {
            return;
        };

        let filter = ThresholdFilter::new(threshold);
        
        if let Ok(Some(new_image)) = state_ref.filter_state.apply_filter(&current_image, &filter) {
            frame.borrow_mut().set_image(Some(new_image.clone()));
            frame.borrow_mut().redraw();
            state_ref.image = Some(new_image);
        }
    }
}

pub fn handle_apply_hue(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    angle: f32
) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        let current_image = if let Some(img) = &state_ref.image {
            img.clone()
        } else {
            return;
        };

        let filter = HueFilter::new(angle);
        
        if let Ok(Some(new_image)) = state_ref.filter_state.apply_filter(&current_image, &filter) {
            frame.borrow_mut().set_image(Some(new_image.clone()));
            frame.borrow_mut().redraw();
            state_ref.image = Some(new_image);
        }
    }
}


pub fn handle_apply_gaussian_blur(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    radius: f32
) {
    start_interactive_convolution(frame, state, ConvolutionType::GaussianBlur {
        radius,
        sigma: radius / 2.0,
    });
}

pub fn handle_apply_box_blur(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    radius: f32
) {
    start_interactive_convolution(frame, state, ConvolutionType::BoxBlur {
        radius,
    });
}

pub fn handle_apply_sharpen(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    intensity: f32
) {
    start_interactive_convolution(frame, state, ConvolutionType::Sharpen {
        intensity,
    });
}

pub fn handle_apply_edge_detection(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    threshold: f32
) {
    start_interactive_edge_detection(frame, state, threshold, EdgeDetectionMethod::Sobel);
}

// Add a new function for Canny edge detection
pub fn handle_apply_canny_edge_detection(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    threshold: f32
) {
    start_interactive_edge_detection(frame, state, threshold, EdgeDetectionMethod::Canny);
}

pub fn handle_apply_noise(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    amount: f32
) {
    start_interactive_noise(frame, state, amount);
}

pub fn handle_apply_vignette(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    intensity: f32
) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        let current_image = if let Some(img) = &state_ref.image {
            img.clone()
        } else {
            return;
        };

        let filter = VignetteFilter::new(intensity);
        
        if let Ok(Some(new_image)) = state_ref.filter_state.apply_filter(&current_image, &filter) {
            frame.borrow_mut().set_image(Some(new_image.clone()));
            frame.borrow_mut().redraw();
            state_ref.image = Some(new_image);
        }
    }
}

pub fn handle_apply_posterize(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    levels: u8
) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        let current_image = if let Some(img) = &state_ref.image {
            img.clone()
        } else {
            return;
        };

        let filter = PosterizeFilter::new(levels);
        
        if let Ok(Some(new_image)) = state_ref.filter_state.apply_filter(&current_image, &filter) {
            frame.borrow_mut().set_image(Some(new_image.clone()));
            frame.borrow_mut().redraw();
            state_ref.image = Some(new_image);
        }
    }
}

pub fn handle_apply_pixelate(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    block_size: u32
) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        let current_image = if let Some(img) = &state_ref.image {
            img.clone()
        } else {
            return;
        };

        let filter = PixelateFilter::new(block_size);
        
        if let Ok(Some(new_image)) = state_ref.filter_state.apply_filter(&current_image, &filter) {
            frame.borrow_mut().set_image(Some(new_image.clone()));
            frame.borrow_mut().redraw();
            state_ref.image = Some(new_image);
        }
    }
}



pub fn handle_apply_motion_blur(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    angle: f32
) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        let current_image = if let Some(img) = &state_ref.image {
            img.clone()
        } else {
            return;
        };

        let filter = MotionBlurFilter::new(angle);
        
        if let Ok(Some(new_image)) = state_ref.filter_state.apply_filter(&current_image, &filter) {
            frame.borrow_mut().set_image(Some(new_image.clone()));
            frame.borrow_mut().redraw();
            state_ref.image = Some(new_image);
        }
    }
}

pub fn handle_toggle_preview(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    if let Ok(mut state_ref) = state.try_borrow_mut() {
        let preview_active = state_ref.filter_state.is_preview_active();
        let current_zoom = state_ref.zoom;
        state_ref.filter_state.toggle_preview();
        
        if let Some(ref image) = state_ref.image.clone() {
            if preview_active {
                // Reload original image when disabling preview
                if let Some(path) = &state_ref.path {
                    if let Ok(img) = image::open(path) {
                        let fltk_image = fltk::image::RgbImage::new(
                            &img.to_rgb8().into_raw(),
                            img.width() as i32,
                            img.height() as i32,
                            fltk::enums::ColorDepth::Rgb8
                        ).unwrap();
                        
                        crate::utils::image::display_image(frame, &fltk_image, current_zoom.into());
                        state_ref.image = Some(fltk_image);
                        return;
                    }
                }
                crate::utils::image::display_image(frame, image, current_zoom.into());
            }
        }
    }
}