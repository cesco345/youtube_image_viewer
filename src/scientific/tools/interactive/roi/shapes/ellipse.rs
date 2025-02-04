// src/scientific/tools/interactive/roi/shapes/ellipse.rs

use fltk::draw;
use super::super::properties::ROIProperties;

pub struct EllipseRenderer;

impl EllipseRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, center: (i32, i32), width: i32, height: i32, properties: &ROIProperties) {
        // Draw fill if specified
        if let Some(fill_color) = properties.fill_color {
            draw::set_draw_color(fltk::enums::Color::from_rgb(
                fill_color.0, fill_color.1, fill_color.2
            ));
            draw::begin_complex_polygon();
            self.draw_ellipse_points(center, width, height);
            draw::end_complex_polygon();
        }

        // Draw outline
        let (r, g, b) = properties.outline_color;
        draw::set_draw_color(fltk::enums::Color::from_rgb(r, g, b));
        draw::set_line_width(properties.line_width);
        
        // Draw the ellipse outline
        draw::arc(
            (center.0 - width/2) as f64,
            (center.1 - height/2) as f64,
            width as f64,
            height as f64,
            0.0,
            360.0
        );

        // Draw control points
        self.draw_control_points(center, width, height);
    }

    fn draw_ellipse_points(&self, center: (i32, i32), width: i32, height: i32) {
        const SEGMENTS: i32 = 72; // Number of segments for smooth ellipse
        for i in 0..=SEGMENTS {
            let angle = (i as f64 * 360.0 / SEGMENTS as f64).to_radians();
            let x = center.0 as f64 + (width as f64 / 2.0) * angle.cos();
            let y = center.1 as f64 + (height as f64 / 2.0) * angle.sin();
            draw::vertex(x, y);
        }
    }

    fn draw_control_points(&self, center: (i32, i32), width: i32, height: i32) {
        // Draw center point
        draw::draw_point(center.0, center.1);
        
        // Draw major axis endpoints
        draw::draw_point(center.0 - width/2, center.1);
        draw::draw_point(center.0 + width/2, center.1);
        
        // Draw minor axis endpoints
        draw::draw_point(center.0, center.1 - height/2);
        draw::draw_point(center.0, center.1 + height/2);
    }
}
