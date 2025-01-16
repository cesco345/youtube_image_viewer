use crate::state::ImageState;
use fltk::{
    dialog::{alert, choice2},
    frame::Frame,
    prelude::*,
    image::RgbImage,
    enums::{Color, ColorDepth, Event},
    app,
    draw,
};
use std::{cell::RefCell, rc::Rc};
use crate::menu::file::save::handle_save_as;
use crate::utils::display_image;

#[derive(Clone)]
pub struct CropSelection {
    pub start_x: i32,
    pub start_y: i32,
    pub end_x: i32,
    pub end_y: i32,
    pub is_selecting: bool,
    pub image_w: i32,
    pub image_h: i32,
    pub frame_w: i32,
    pub frame_h: i32,
}

impl CropSelection {
    pub fn new(image_w: i32, image_h: i32, frame_w: i32, frame_h: i32) -> Self {
        Self {
            start_x: 0,
            start_y: 0,
            end_x: 0,
            end_y: 0,
            is_selecting: false,
            image_w,
            image_h,
            frame_w,
            frame_h,
        }
    }

    pub fn get_dimensions(&self) -> (i32, i32, i32, i32) {
        let x = self.start_x.min(self.end_x);
        let y = self.start_y.min(self.end_y);
        let w = (self.start_x - self.end_x).abs();
        let h = (self.start_y - self.end_y).abs();
        (x, y, w, h)
    }

    pub fn get_image_dimensions(&self) -> (i32, i32, i32, i32) {
        let (x, y, w, h) = self.get_dimensions();
        
        // Calculate FLTK's display scale
        let scale_x = self.frame_w as f64 / self.image_w as f64;
        let scale_y = self.frame_h as f64 / self.image_h as f64;
        let scale = scale_x.min(scale_y);  // FLTK uses the smaller scale to preserve aspect ratio

        // Calculate display size
        let displayed_w = (self.image_w as f64 * scale) as i32;
        let displayed_h = (self.image_h as f64 * scale) as i32;

        // Calculate offsets for centered image
        let offset_x = (self.frame_w - displayed_w) / 2;
        let offset_y = (self.frame_h - displayed_h) / 2;

        // Convert screen coordinates back to image coordinates
        let image_x = ((x as f64 - offset_x as f64) / scale) as i32;
        let image_y = ((y as f64 - offset_y as f64) / scale) as i32;
        let image_w = (w as f64 / scale) as i32;
        let image_h = (h as f64 / scale) as i32;

        // Ensure coordinates are within bounds
        let image_x = image_x.max(0);
        let image_y = image_y.max(0);
        let image_w = image_w.min(self.image_w - image_x);
        let image_h = image_h.min(self.image_h - image_y);

        (image_x, image_y, image_w, image_h)
    }

    pub fn reset(&mut self) {
        self.start_x = 0;
        self.start_y = 0;
        self.end_x = 0;
        self.end_y = 0;
        self.is_selecting = false;
    }
}

fn handle_crop_with_selection(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) -> bool {
    let cropped_img = {
        let state_ref = state.borrow();
        if let (Some(current_image), Some(selection)) = (&state_ref.image, &state_ref.crop_selection) {
            let (image_x, image_y, image_w, image_h) = selection.get_image_dimensions();
            let img_w = current_image.data_w();
            let img_h = current_image.data_h();

            // Ensure coordinates are within bounds
            let image_x = image_x.max(0).min(img_w - 1);
            let image_y = image_y.max(0).min(img_h - 1);
            let image_w = image_w.min(img_w - image_x);
            let image_h = image_h.min(img_h - image_y);

            let src_data = current_image.to_rgb_data();
            let mut cropped_data = vec![0u8; (image_w * image_h * 3) as usize];

            for dy in 0..image_h {
                for dx in 0..image_w {
                    let src_pos = (((image_y + dy) * img_w + (image_x + dx)) * 3) as usize;
                    let dst_pos = ((dy * image_w + dx) * 3) as usize;
                    
                    if src_pos + 2 < src_data.len() && dst_pos + 2 < cropped_data.len() {
                        cropped_data[dst_pos] = src_data[src_pos];
                        cropped_data[dst_pos + 1] = src_data[src_pos + 1];
                        cropped_data[dst_pos + 2] = src_data[src_pos + 2];
                    }
                }
            }

            RgbImage::new(
                &cropped_data,
                image_w,
                image_h,
                ColorDepth::Rgb8,
            ).ok()
        } else {
            None
        }
    };

    if let Some(mut fltk_image) = cropped_img {
        let mut state_ref = state.borrow_mut();
        state_ref.image = Some(fltk_image.clone());
        state_ref.path = None;
        state_ref.crop_selection = None;
        drop(state_ref);
        
        
        display_image(frame, &fltk_image, 1.0);
        true
    } else {
        alert(300, 300, "Failed to create cropped image");
        false
    }
}

pub fn start_interactive_crop(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    let mut state_ref = state.borrow_mut();
    if state_ref.image.is_none() {
        alert(300, 300, "Please open an image first");
        return;
    }
    
    let original_image = state_ref.image.clone();
    
    // Initialize crop selection with image and frame dimensions
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
                    draw::set_draw_color(Color::White);
                    draw::set_line_style(draw::LineStyle::Solid, 2);
                    draw::draw_rect(x, y, w, h);
                    
                    draw::set_draw_color(Color::Black);
                    draw::set_line_style(draw::LineStyle::Dash, 1);
                    draw::draw_rect(x-1, y-1, w+2, h+2);
                }
            }
        }
    };

    frame.draw(draw_callback);

    let handle_callback = {
        let state_clone = state_clone.clone();
        let frame_clone = frame_clone.clone();
        move |f: &mut Frame, ev: Event| -> bool {
            match ev {
                Event::Push => {
                    let mut handled = false;
                    if let Ok(mut state) = state_clone.try_borrow_mut() {
                        if let Some(selection) = &mut state.crop_selection {
                            selection.reset();  // Reset previous selection state
                            selection.start_x = app::event_x();
                            selection.start_y = app::event_y();
                            selection.is_selecting = true;
                            handled = true;
                        }
                    }
                    if handled {
                        f.redraw();
                    }
                    handled
                },
                Event::Drag => {
                    let mut handled = false;
                    if let Ok(mut state) = state_clone.try_borrow_mut() {
                        if let Some(selection) = &mut state.crop_selection {
                            selection.end_x = app::event_x();
                            selection.end_y = app::event_y();
                            handled = true;
                        }
                    }
                    if handled {
                        f.redraw();
                    }
                    handled
                },
                Event::Released => {
                    // First reset the selection state
                    if let Ok(mut state) = state_clone.try_borrow_mut() {
                        if let Some(selection) = &mut state.crop_selection {
                            selection.is_selecting = false;
                            let end_x = app::event_x();
                            let end_y = app::event_y();
                            selection.end_x = end_x;
                            selection.end_y = end_y;
                            
                            let (_, _, w, h) = selection.get_dimensions();
                            
                            // Handle small selections immediately
                            if w <= 5 || h <= 5 {
                                selection.reset();
                                f.redraw();
                                return true;
                            }
                        }
                    }

                    // Handle cropping if selection was valid
                    if let Some(0) = choice2(300, 300, "Do you want to crop the selected area?", "Yes", "No", "") {
                        if handle_crop_with_selection(&frame_clone, &state_clone) {
                            match choice2(300, 300, "Would you like to save?", "Yes", "No", "") {
                                Some(0) => {
                                    handle_save_as(&frame_clone, &state_clone);
                                },
                                _ => {
                                    // User chose not to save, restore original image
                                    if let Ok(mut state) = state_clone.try_borrow_mut() {
                                        state.image = original_image.clone();
                                        state.crop_selection = None;
                                        
                                        // Display original image
                                        if let Some(img) = &original_image {
                                            
                                            display_image(&frame_clone, img, 1.0);
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // User chose not to crop, ensure cleanup
                        if let Ok(mut state) = state_clone.try_borrow_mut() {
                            if let Some(selection) = &mut state.crop_selection {
                                selection.reset();
                            }
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