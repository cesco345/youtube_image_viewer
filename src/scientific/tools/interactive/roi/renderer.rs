// src/scientific/tools/interactive/roi/renderer.rs

use fltk::{draw, frame::Frame, prelude::*};
use super::shapes::ShapeRenderer;
use super::properties::ROIProperties;
use crate::scientific::types::LineStyle;

pub struct ROIRenderer {
    shape_renderer: ShapeRenderer,
}

impl ROIRenderer {
    pub fn new() -> Self {
        Self {
            shape_renderer: ShapeRenderer::new(),
        }
    }

    pub fn draw(&self, frame: &mut Frame, state: &ROIState) {
        self.setup_drawing_context(&state.properties);
        
        if let Some(shape) = &state.current_shape {
            self.shape_renderer.render(shape, &state.properties);
        }
        
        if state.show_measurements {
            self.draw_measurements(&state.measurements);
        }
        
        if !state.properties.label.is_empty() {
            self.draw_label(&state.properties.label, state.get_label_position());
        }
    }

    fn setup_drawing_context(&self, props: &ROIProperties) {
        let (r, g, b) = props.outline_color;
        draw::set_draw_color(fltk::enums::Color::from_rgb(r, g, b));
        draw::set_line_width(props.line_width);
        
        match props.line_style {
            LineStyle::Solid => draw::set_line_style(draw::LineStyle::Solid, props.line_width),
            LineStyle::Dashed => draw::set_line_style(draw::LineStyle::Dash, props.line_width),
            LineStyle::Dotted => draw::set_line_style(draw::LineStyle::Dot, props.line_width),
        }
    }

    fn draw_measurements(&self, measurements: &Option<ROIMeasurements>) {
        if let Some(m) = measurements {
            draw::set_font(fltk::enums::Font::Helvetica, 12);
            let text = format!(
                "Area: {:.1} px²\nPerimeter: {:.1} px\nMean: {:.1}",
                m.area, m.perimeter, m.mean_intensity
            );
            // Position will be handled by shape-specific logic
            draw::draw_text(&text, m.centroid.0 as i32, m.centroid.1 as i32);
        }
    }

    fn draw_label(&self, label: &str, position: (i32, i32)) {
        draw::set_font(fltk::enums::Font::Helvetica, 14);
        draw::draw_text(label, position.0, position.1 - 15);
    }
}