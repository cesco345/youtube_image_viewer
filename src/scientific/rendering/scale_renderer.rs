//src/scientific/rendering/scale_renderer.rs
use fltk::{enums::{Color, Font, Align}, draw};
use crate::scientific::types::LegendPosition;
use crate::scientific::rendering::frame_renderer::FrameRenderer;

pub struct ScaleRenderer;

impl ScaleRenderer {
    pub fn draw_legend(
        x: i32, 
        y: i32, 
        width: i32, 
        height: i32,
        position: LegendPosition,
        unit: &str,
        pixels_per_unit: f32,
    ) {
        let (x_offset, y_offset) = Self::calculate_position(x, y, width, height, position);
        let scale_length = 100;

        // Draw scale bar
        draw::set_draw_color(Color::White);
        draw::set_line_style(draw::LineStyle::Solid, 2);
        draw::draw_line(x_offset, y_offset, x_offset + scale_length, y_offset);
        
        // Draw tick marks
        let tick_height = 5;
        draw::draw_line(x_offset, y_offset - tick_height, x_offset, y_offset + tick_height);
        draw::draw_line(x_offset + scale_length, y_offset - tick_height, x_offset + scale_length, y_offset + tick_height);
        
        // Draw text
        draw::set_font(Font::Helvetica, 12);
        draw::set_draw_color(Color::White);
        draw::draw_text2(
            &format!("100 {}", unit),
            x_offset,
            y_offset - 15,
            0, 0,
            Align::Left,
        );
    }

    pub fn draw_scale_preview(
        start: (i32, i32),
        end: Option<(i32, i32)>,
        unit: &str,
    ) {
        if let Some(end) = end {
            // Draw the measurement line
            draw::set_draw_color(Color::White);
            draw::set_line_style(draw::LineStyle::Solid, 2);
            draw::draw_line(start.0, start.1, end.0, end.1);
            
            // Draw endpoints
            draw::draw_circle(start.0 as f64, start.1 as f64, 3.0);
            draw::draw_circle(end.0 as f64, end.1 as f64, 3.0);
            
            // Show pixel distance
            let dx = end.0 - start.0;
            let dy = end.1 - start.1;
            let distance = ((dx * dx + dy * dy) as f64).sqrt();
            draw::draw_text2(
                &format!("{:.1} px", distance),
                end.0 + 10,
                end.1 + 10,
                0, 0,
                Align::Left,
            );
        } else {
            // Draw guide text
            draw::set_font(Font::Helvetica, 14);
            draw::set_draw_color(Color::White);
            draw::draw_text2(
                "Click and drag to draw scale line",
                start.0 + 10,
                start.1 + 20,
                0, 0,
                Align::Left,
            );
        }
    }

    fn calculate_position(x: i32, y: i32, width: i32, height: i32, position: LegendPosition) -> (i32, i32) {
        let margin = 20;
        let scale_length = 100;
        
        match position {
            LegendPosition::TopLeft => (
                x + margin,  // Left margin
                y + margin   // Top margin
            ),
            LegendPosition::TopRight => (
                x + width - scale_length - margin,  // Right align considering scale length
                y + margin
            ),
            LegendPosition::BottomLeft => (
                x + margin,
                y + height - margin - 20  // Bottom margin with space for text
            ),
            LegendPosition::BottomRight => (
                x + width - scale_length - margin,
                y + height - margin - 20
            ),
        }
    }
}