// src/scientific/tools/interactive/roi/shapes/mod.rs

mod polygon;
mod rectangle;
mod ellipse;
mod line;

pub use polygon::PolygonRenderer;
pub use rectangle::RectangleRenderer;
pub use ellipse::EllipseRenderer;
pub use line::LineRenderer;

use super::properties::ROIProperties;
use crate::scientific::types::ROIShape;

pub struct ShapeRenderer {
    polygon_renderer: PolygonRenderer,
    rectangle_renderer: RectangleRenderer,
    ellipse_renderer: EllipseRenderer,
    line_renderer: LineRenderer,
}

impl ShapeRenderer {
    pub fn new() -> Self {
        Self {
            polygon_renderer: PolygonRenderer::new(),
            rectangle_renderer: RectangleRenderer::new(),
            ellipse_renderer: EllipseRenderer::new(),
            line_renderer: LineRenderer::new(),
        }
    }

    pub fn render(&self, shape: &ROIShape, properties: &ROIProperties) {
        match shape {
            ROIShape::Polygon { points } => {
                self.polygon_renderer.render(points, properties);
            },
            ROIShape::Rectangle { width, height } => {
                // Assuming the first point in properties.points is the top-left corner
                if let Some(&top_left) = properties.points.first() {
                    self.rectangle_renderer.render(top_left, *width, *height, properties);
                }
            },
            ROIShape::Ellipse { width, height } => {
                // Assuming the first point in properties.points is the center
                if let Some(&center) = properties.points.first() {
                    self.ellipse_renderer.render(center, *width, *height, properties);
                }
            },
            ROIShape::Line { points } => {
                self.line_renderer.render(points, properties);
            }
        }
    }
}
