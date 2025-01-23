// src/menu/edit/watermark/removal_tool.rs
use fltk::{
    draw,
    enums::{Color, Event},
    frame::Frame,
    prelude::*,
};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;
use super::RemovalArea;

pub struct WatermarkRemovalTool {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
    is_selecting: bool,
    image_offset_x: i32,
    image_offset_y: i32,
    image_scale: f64,
}

impl WatermarkRemovalTool {
    pub fn new() -> Self {
        Self {
            start_x: 0,
            start_y: 0,
            end_x: 0,
            end_y: 0,
            is_selecting: false,
            image_offset_x: 0,
            image_offset_y: 0,
            image_scale: 1.0,
        }
    }

    pub fn calculate_image_position(&mut self, frame: &Frame, image_w: i32, image_h: i32, zoom: f64) {
        let frame_w = frame.w();
        let frame_h = frame.h();
        let menu_height = crate::utils::MENU_HEIGHT;

        // Calculate scaled dimensions
        let scaled_w = (image_w as f64 * zoom) as i32;
        let scaled_h = (image_h as f64 * zoom) as i32;

        // Center the image in the frame
        self.image_offset_x = (frame_w - scaled_w) / 2;
        self.image_offset_y = ((frame_h - menu_height) - scaled_h) / 2 + menu_height;
        self.image_scale = zoom;
    }

    pub fn start_selection(&mut self, x: i32, y: i32) {
        self.start_x = x;
        self.start_y = y;
        self.end_x = x;
        self.end_y = y;
        self.is_selecting = true;
    }

    pub fn update_selection(&mut self, x: i32, y: i32) {
        if self.is_selecting {
            self.end_x = x;
            self.end_y = y;
        }
    }

    pub fn finish_selection(&mut self) -> Option<RemovalArea> {
        if !self.is_selecting {
            return None;
        }
        self.is_selecting = false;

        // Convert screen coordinates to image coordinates
        let start_x = ((self.start_x - self.image_offset_x) as f64 / self.image_scale) as i32;
        let start_y = ((self.start_y - self.image_offset_y) as f64 / self.image_scale) as i32;
        let end_x = ((self.end_x - self.image_offset_x) as f64 / self.image_scale) as i32;
        let end_y = ((self.end_y - self.image_offset_y) as f64 / self.image_scale) as i32;

        let (x, width) = if end_x >= start_x {
            (start_x, end_x - start_x)
        } else {
            (end_x, start_x - end_x)
        };
        
        let (y, height) = if end_y >= start_y {
            (start_y, end_y - start_y)
        } else {
            (end_y, start_y - end_y)
        };

        Some(RemovalArea { x, y, width, height })
    }

    pub fn draw(&self) {
        if self.is_selecting {
            draw::set_draw_color(Color::Red);
            draw::set_line_style(draw::LineStyle::Solid, 2);
            draw::draw_rect(
                self.start_x.min(self.end_x),
                self.start_y.min(self.end_y),
                (self.end_x - self.start_x).abs(),
                (self.end_y - self.start_y).abs()
            );
        }
    }
}

pub fn start_watermark_removal(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
    let state = state.clone();
    let mut removal_tool = WatermarkRemovalTool::new();
    
    // Initialize tool with current image position
    if let Ok(state_ref) = state.try_borrow() {
        if let Some(ref image) = state_ref.image {
            removal_tool.calculate_image_position(
                &frame.borrow(),
                image.data_w(),
                image.data_h(),
                state_ref.zoom.into()
            );
        }
    }

    let mut frame = frame.borrow_mut();
    frame.handle(move |f, ev| {
        match ev {
            Event::Push => {
                let coords = fltk::app::event_coords();
                removal_tool.start_selection(coords.0, coords.1);
                f.redraw();
                true
            },
            Event::Drag => {
                let coords = fltk::app::event_coords();
                removal_tool.update_selection(coords.0, coords.1);
                f.redraw();
                true
            },
            Event::Released => {
                if let Some(area) = removal_tool.finish_selection() {
                    if let Ok(mut state_ref) = state.try_borrow_mut() {
                        if let Some(ref current_image) = state_ref.image.clone() {
                            if let Ok(Some(new_image)) = state_ref.watermark_state.remove_watermark_area(current_image, &area) {
                                f.set_image(Some(new_image.clone()));
                                f.redraw();
                                state_ref.image = Some(new_image);
                            }
                        }
                    }
                }
                true
            },
            _ => false,
        }
    });
}