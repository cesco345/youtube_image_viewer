use fltk::{frame::Frame, image::RgbImage, prelude::*};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;
use crate::scientific::rendering::frame_renderer::FrameRenderer;

pub const MENU_HEIGHT: i32 = 25;

pub fn display_image_with_zoom(frame: &Rc<RefCell<Frame>>, image: &mut RgbImage, zoom: f32, state: &Rc<RefCell<ImageState>>) {
    let frame_ref = frame.borrow();
    let frame_w = frame_ref.w();
    let frame_h = frame_ref.h();
                
    let (new_w, new_h) = scale_image_dimensions(
        image.data_w(),
        image.data_h(),
        frame_w,
        frame_h - MENU_HEIGHT,
        zoom as f64
    );

    image.scale(new_w, new_h, true, true);
    drop(frame_ref);
    
    // Only set up the frame draw, don't set_image directly
    FrameRenderer::setup_frame_draw(frame, state);
    frame.borrow_mut().redraw();
}

pub fn display_image(frame: &Rc<RefCell<Frame>>, img: &RgbImage, zoom: f32) {
    let mut frame = frame.borrow_mut();
    let frame_w = frame.w();
    let frame_h = frame.h();
    
    let (new_w, new_h) = scale_image_dimensions(
        img.data_w(),
        img.data_h(),
        frame_w,
        frame_h - MENU_HEIGHT,
        zoom as f64
    );
    
    let mut scaled_img = img.clone();
    scaled_img.scale(new_w, new_h, true, true);
    frame.set_image(Some(scaled_img));
    frame.redraw();
}

pub fn scale_image_dimensions(
    image_w: i32, 
    image_h: i32, 
    frame_w: i32, 
    frame_h: i32, 
    zoom: f64
) -> (i32, i32) {
    let aspect_ratio = image_w as f64 / image_h as f64;
    let frame_aspect = frame_w as f64 / frame_h as f64;

    let (base_w, base_h) = if aspect_ratio > frame_aspect {
        let new_w = frame_w;
        let new_h = (frame_w as f64 / aspect_ratio) as i32;
        (new_w, new_h)
    } else {
        let new_h = frame_h;
        let new_w = (frame_h as f64 * aspect_ratio) as i32;
        (new_w, new_h)
    };

    ((base_w as f64 * zoom) as i32, (base_h as f64 * zoom) as i32)
}