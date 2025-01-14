use fltk::{frame::Frame, image::RgbImage, prelude::*};

pub const MENU_HEIGHT: i32 = 25;

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

pub fn display_image(frame: &mut Frame, image: &mut RgbImage, zoom: f64) {
    let frame_w = frame.w();
    let frame_h = frame.h();
                
    let (new_w, new_h) = scale_image_dimensions(
        image.data_w(),
        image.data_h(),
        frame_w,
        frame_h - MENU_HEIGHT,
        zoom
    );

    image.scale(new_w, new_h, true, true);
    frame.set_image(Some(image.clone()));
    frame.redraw();
}