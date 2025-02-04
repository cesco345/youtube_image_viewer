// src/scientific/tools/interactive/roi/shapes/rectangle.rs

use fltk::draw;
use super::super::properties::ROIProperties;

pub struct RectangleRenderer;

impl RectangleRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, top_left: (i32, i32), width: i32, height: i32, properties: &ROIProperties) {
        // Draw fill if specified
        if let Some(fill_color) = properties.fill_color {
            draw::set_draw_color(fltk::enums::Color::from_rgb(
                fill_color.0, fill_color.1, fill_color.2
            ));
            draw::begin_complex_polygon();
            draw::vertex(top_left.0 as f64, top_left.1 as f64);
            draw::vertex((top_left.0 + width) as f64, top_left.1 as f64);
            draw::vertex((top_left.0 + width) as f64, (top_left.1 + height) as f64);
            draw::vertex(top_left.0 as f64, (top_left.1 + height) as f64);
            draw::end_complex_polygon();
        }

        // Draw outline
        let (r, g, b) = properties.outline_color;
        draw::set_draw_color(fltk::enums::Color::from_rgb(r, g, b));
        draw::set_line_width(properties.line_width);
        
        // Draw rectangle outline
        draw::draw_rect(
            top_left.0,
            top_left.1,
            width,
            height
        );

        // Draw control points at corners
        self.draw_control_points(top_left, width, height);
    }

    fn draw_control_points(&self, top_left: (i32, i32), width: i32, height: i32) {
        let points = [
            top_left,
            (top_left.0 + width, top_left.1),
            (top_left.0 + width, top_left.1 + height),
            (top_left.0, top_left.1 + height),
        ];

        for &(x, y) in &points {
            draw::draw_point(x, y);
            
            // Draw small crosshair for better visibility
            draw::draw_line(x - 3, y, x + 3, y);
            draw::draw_line(x, y - 3, x, y + 3);
        }
    }
}