// src/scientific/tools/interactive/roi/shapes/polygon.rs
pub struct PolygonRenderer;

impl PolygonRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, points: &[(i32, i32)], properties: &ROIProperties) {
        if points.len() < 2 {
            return;
        }

        // Draw fill if specified
        if let Some(fill_color) = properties.fill_color {
            draw::set_draw_color(fltk::enums::Color::from_rgb(
                fill_color.0, fill_color.1, fill_color.2
            ));
            draw::begin_complex_polygon();
            for &(x, y) in points {
                draw::vertex(x as f64, y as f64);
            }
            draw::end_complex_polygon();
        }

        // Draw outline
        let (r, g, b) = properties.outline_color;
        draw::set_draw_color(fltk::enums::Color::from_rgb(r, g, b));
        
        for i in 0..points.len() - 1 {
            draw::draw_line(
                points[i].0, points[i].1,
                points[i + 1].0, points[i + 1].1
            );
        }
        
        // Close the polygon
        if points.len() > 2 {
            let last = points.len() - 1;
            draw::draw_line(
                points[last].0, points[last].1,
                points[0].0, points[0].1
            );
        }
    }
}