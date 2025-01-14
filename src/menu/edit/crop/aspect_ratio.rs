// src/menu/edit/crop/aspect_ratio.rs
pub struct CropSelection {
    pub start_x: i32,
    pub start_y: i32,
    pub end_x: i32,
    pub end_y: i32,
    pub aspect_ratio: Option<f64>, // None for free-form, Some(w/h) for fixed ratio
    pub is_selecting: bool
}

impl CropSelection {
    pub fn new() -> Self {
        Self {
            start_x: 0,
            start_y: 0,
            end_x: 0,
            end_y: 0,
            aspect_ratio: None,
            is_selecting: false
        }
    }

    pub fn get_dimensions(&self) -> (i32, i32, i32, i32) {
        let x = self.start_x.min(self.end_x);
        let y = self.start_y.min(self.end_y);
        let w = (self.start_x - self.end_x).abs();
        let h = (self.start_y - self.end_y).abs();
        (x, y, w, h)
    }

    pub fn adjust_to_ratio(&mut self) {
        if let Some(ratio) = self.aspect_ratio {
            let current_w = (self.end_x - self.start_x).abs() as f64;
            let current_h = (self.end_y - self.start_y).abs() as f64;
            let current_ratio = current_w / current_h;

            if current_ratio > ratio {
                // Too wide - adjust width
                let new_w = (current_h * ratio) as i32;
                if self.end_x > self.start_x {
                    self.end_x = self.start_x + new_w;
                } else {
                    self.end_x = self.start_x - new_w;
                }
            } else {
                // Too tall - adjust height
                let new_h = (current_w / ratio) as i32;
                if self.end_y > self.start_y {
                    self.end_y = self.start_y + new_h;
                } else {
                    self.end_y = self.start_y - new_h;
                }
            }
        }
    }
}