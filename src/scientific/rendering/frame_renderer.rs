//src/scientific/rendering/frame_renderer.rs
use fltk::{
    frame::Frame,
    prelude::*,
    enums::{Color, Font, Align},
    draw,
};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;
use crate::scientific::rendering::ScaleRenderer;
use crate::utils::{scale_image_dimensions, MENU_HEIGHT};

pub struct FrameRenderer;

impl FrameRenderer {
    pub fn setup_frame_draw(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
        let state = state.clone();
        
        frame.borrow_mut().draw(move |f| {
            // draw the base image if available
            if let Some(base_img) = state.borrow().image.as_ref() {
                let mut img = base_img.clone();
                let zoom = state.borrow().zoom;
                
                // scale image according to frame size and zoom
                let (new_w, new_h) = scale_image_dimensions(
                    img.data_w(),
                    img.data_h(),
                    f.width(),
                    f.height() - MENU_HEIGHT,
                    zoom as f64
                );
                img.scale(new_w, new_h, true, true);
                img.draw(f.x(), f.y(), f.width(), f.height());
            }
        });
    }

    pub fn setup_scientific_frame(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) {
        let state_clone = state.clone();
        
        // Set a single draw callback that handles both image and scale
        frame.borrow_mut().draw(move |f| {
            let state_ref = state_clone.borrow();
            
            // Draw base image first
            if let Some(base_img) = state_ref.image.as_ref() {
                let mut img = base_img.clone();
                let zoom = state_ref.zoom;
                
                // Scale image according to frame size and zoom
                let (new_w, new_h) = scale_image_dimensions(
                    img.data_w(),
                    img.data_h(),
                    f.width(),
                    f.height() - MENU_HEIGHT,
                    zoom as f64
                );
                img.scale(new_w, new_h, true, true);
                img.draw(f.x(), f.y(), f.width(), f.height());
                
                // Draw the scale if enabled
                if state_ref.scientific_state.show_legend {
                    ScaleRenderer::draw_legend(
                        f.x(),
                        f.y(),
                        f.width(),
                        f.height(),
                        state_ref.scientific_state.legend_position,
                        &state_ref.scientific_state.calibration.unit,
                        state_ref.scientific_state.calibration.pixels_per_unit,
                    );
                }
            }
        });
    }

    /// draw text with background for better visibility
    pub fn draw_text_with_background(
        text: &str,
        x: i32,
        y: i32,
        font_size: i32,
        font: Font,
        text_color: Color,
        bg_color: Color
    ) {
        let text_width = text.len() * 8; // Approximate width
        let text_height = font_size + 4;

        // draw background
        draw::set_draw_color(bg_color);
        draw::draw_rectf(
            x - 2,
            y - text_height + 2,
            text_width as i32 + 4,
            text_height,
        );

        // draw text
        draw::set_font(font, font_size);
        draw::set_draw_color(text_color);
        draw::draw_text2(
            text,
            x,
            y,
            0,
            0,
            Align::Left,
        );
    }

    /// draw a line with endpoints
    pub fn draw_line_with_endpoints(
        start: (i32, i32),
        end: (i32, i32),
        color: Color,
        line_width: i32,
        endpoint_radius: f64
    ) {
        draw::set_draw_color(color);
        draw::set_line_style(draw::LineStyle::Solid, line_width);
        draw::draw_line(start.0, start.1, end.0, end.1);
        
        if endpoint_radius > 0.0 {
            draw::draw_circle(start.0 as f64, start.1 as f64, endpoint_radius);
            draw::draw_circle(end.0 as f64, end.1 as f64, endpoint_radius);
        }
    }

    /// draw a measurement with value
    pub fn draw_measurement(
        start: (i32, i32),
        end: (i32, i32),
        value: f64,
        unit: &str,
        color: Color
    ) {
        // draw the line
        Self::draw_line_with_endpoints(start, end, color, 2, 3.0);

        // calculate text position (above middle of line)
        let text_x = (start.0 + end.0) / 2 - 20;
        let text_y = (start.1 + end.1) / 2 - 15;
        let text = format!("{:.1} {}", value, unit);

        // draw measurement text
        Self::draw_text_with_background(
            &text,
            text_x,
            text_y,
            12,
            Font::Helvetica,
            Color::White,
            Color::Black,
        );
    }

    /// draw guide text for user interaction
    pub fn draw_guide_text(text: &str, x: i32, y: i32) {
        Self::draw_text_with_background(
            text,
            x,
            y,
            14,
            Font::Helvetica,
            Color::White,
            Color::Black,
        );
    }

    /// calculate position on frame relative to anchor point
    pub fn calculate_position(
        frame_x: i32,
        frame_y: i32,
        frame_w: i32,
        frame_h: i32,
        offset_x: i32,
        offset_y: i32,
        anchor: Align,
    ) -> (i32, i32) {
        match anchor {
            Align::TopLeft => (frame_x + offset_x, frame_y + offset_y),
            Align::TopRight => (frame_x + frame_w - offset_x, frame_y + offset_y),
            Align::BottomLeft => (frame_x + offset_x, frame_y + frame_h - offset_y),
            Align::BottomRight => (frame_x + frame_w - offset_x, frame_y + frame_h - offset_y),
            _ => (frame_x + offset_x, frame_y + offset_y),
        }
    }
}