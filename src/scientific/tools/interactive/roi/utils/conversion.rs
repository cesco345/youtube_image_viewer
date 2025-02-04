// src/scientific/tools/interactive/roi/utils/conversion.rs

use super::{ScalingInfo, ROIResult, ROIError};
use fltk::frame::Frame;

pub struct CoordinateConverter {
    scaling: Option<ScalingInfo>,
}

impl CoordinateConverter {
    pub fn new() -> Self {
        Self { scaling: None }
    }

    pub fn update_scaling(&mut self, frame: &Frame, img_w: i32, img_h: i32) {
        let frame_w = frame.width() as f32;
        let frame_h = frame.height() as f32;
        let img_w = img_w as f32;
        let img_h = img_h as f32;

        let frame_aspect = frame_w / frame_h;
        let img_aspect = img_w / img_h;

        let (scale, offset_x, offset_y) = if frame_aspect > img_aspect {
            let scale = frame_h / img_h;
            let offset_x = ((frame_w - (img_w * scale)) / 2.0) as i32;
            (scale, offset_x, 0)
        } else {
            let scale = frame_w / img_w;
            let offset_y = ((frame_h - (img_h * scale)) / 2.0) as i32;
            (scale, 0, offset_y)
        };

        self.scaling = Some(ScalingInfo {
            scale,
            offset_x,
            offset_y,
            frame_x: frame.x(),
            frame_y: frame.y(),
            img_w: img_w as i32,
            img_h: img_h as i32,
        });
    }

    pub fn display_to_image(&self, display_x: i32, display_y: i32) -> ROIResult<(i32, i32)> {
        let scaling = self.scaling.as_ref().ok_or_else(|| {
            ROIError::ValidationError("Scaling information not initialized".to_string())
        })?;

        let rel_x = (display_x - scaling.frame_x - scaling.offset_x) as f32;
        let rel_y = (display_y - scaling.frame_y - scaling.offset_y) as f32;

        let img_x = (rel_x / scaling.scale) as i32;
        let img_y = (rel_y / scaling.scale) as i32;

        let img_x = img_x.clamp(0, scaling.img_w - 1);
        let img_y = img_y.clamp(0, scaling.img_h - 1);

        Ok((img_x, img_y))
    }

    pub fn image_to_display(&self, image_x: i32, image_y: i32) -> ROIResult<(i32, i32)> {
        let scaling = self.scaling.as_ref().ok_or_else(|| {
            ROIError::ValidationError("Scaling information not initialized".to_string())
        })?;

        let display_x = (image_x as f32 * scaling.scale) as i32 
            + scaling.offset_x 
            + scaling.frame_x;
        let display_y = (image_y as f32 * scaling.scale) as i32 
            + scaling.offset_y 
            + scaling.frame_y;

        Ok((display_x, display_y))
    }

    pub fn convert_points(&self, points: &[(i32, i32)], to_display: bool) -> ROIResult<Vec<(i32, i32)>> {
        points
            .iter()
            .map(|&(x, y)| {
                if to_display {
                    self.image_to_display(x, y)
                } else {
                    self.display_to_image(x, y)
                }
            })
            .collect()
    }

    pub fn get_scaling_info(&self) -> Option<ScalingInfo> {
        self.scaling.clone()
    }
}