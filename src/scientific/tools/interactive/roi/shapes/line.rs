// src/scientific/tools/interactive/roi/shapes/line.rs

use fltk::draw;
use super::super::properties::ROIProperties;

pub struct LineRenderer;

impl LineRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, points: &[(i32, i32)], properties: &ROIProperties) {
        if points.len() < 2 {
            return;
        }

        // Draw line segments
        let (r, g, b) = properties.outline_color;
        draw::set_draw_color(fltk::enums::Color::from_rgb(r, g, b));
        draw::set_line_width(properties.line_width);

        // Draw main line segments
        for i in 0..points.len() - 1 {
            draw::draw_line(
                points[i].0,
                points[i].1,
                points[i + 1].0,
                points[i + 1].1
            );
        }

        // Draw length measurements if required
        if properties.show_measurements {
            self.draw_measurements(points);
        }

        // Draw vertices
        for &point in points {
            self.draw_vertex(point);
        }
    }

    fn draw_vertex(&self, point: (i32, i32)) {
        let (x, y) = point;
        // Draw point
        draw::draw_point(x, y);
        
        // Draw crosshair for better visibility
        draw::draw_line(x - 3, y, x + 3, y);
        draw::draw_line(x, y - 3, x, y + 3);
    }

    fn draw_measurements(&self, points: &[(i32, i32)]) {
        for i in 0..points.len() - 1 {
            let p1 = points[i];
            let p2 = points[i + 1];
            
            // Calculate midpoint for label placement
            let mid_x = (p1.0 + p2.0) / 2;
            let mid_y = (p1.1 + p2.1) / 2;
            
            // Calculate length
            let dx = (p2.0 - p1.0) as f64;
            let dy = (p2.1 - p1.1) as f64;
            let length = (dx * dx + dy * dy).sqrt();
            
            // Draw length label
            draw::set_font(fltk::enums::Font::Helvetica, 12);
            draw::draw_text2(
                &format!("{:.1}", length),
                mid_x,
                mid_y,
                0,
                0,
                fltk::enums::Align::Center,
            );
        }
    }
}