// src/scientific/tools/interactive/roi/interaction.rs

use fltk::enums::Event;
use super::properties::ROIState;
use crate::scientific::types::ROIShape;

pub enum ROIEvent {
    MouseDown { x: i32, y: i32 },
    MouseDrag { x: i32, y: i32 },
    MouseUp { x: i32, y: i32 },
    KeyPress(char),
    Cancel,
    Complete,
}

pub struct InteractionHandler {
    last_position: Option<(i32, i32)>,
    is_modified: bool,
}

impl InteractionHandler {
    pub fn new() -> Self {
        Self {
            last_position: None,
            is_modified: false,
        }
    }

    pub fn handle(&mut self, event: ROIEvent, state: &mut ROIState) -> bool {
        if state.properties.is_locked {
            return false;
        }

        match event {
            ROIEvent::MouseDown { x, y } => {
                self.handle_mouse_down(x, y, state)
            },
            ROIEvent::MouseDrag { x, y } => {
                self.handle_mouse_drag(x, y, state)
            },
            ROIEvent::MouseUp { x, y } => {
                self.handle_mouse_up(x, y, state)
            },
            ROIEvent::KeyPress(key) => {
                self.handle_key_press(key, state)
            },
            ROIEvent::Cancel => {
                self.handle_cancel(state)
            },
            ROIEvent::Complete => {
                self.handle_complete(state)
            }
        }
    }

    fn handle_mouse_down(&mut self, x: i32, y: i32, state: &mut ROIState) -> bool {
        if !state.is_drawing {
            state.start_drawing(ROIShape::Polygon { points: Vec::new() });
        }
        
        state.add_point((x, y));
        self.last_position = Some((x, y));
        self.is_modified = true
        true
    }

    fn handle_mouse_drag(&mut self, x: i32, y: i32, state: &mut ROIState) -> bool {
        if let Some(last_pos) = self.last_position {
            // Only add new point if moved significantly
            let dx = x - last_pos.0;
            let dy = y - last_pos.1;
            if dx * dx + dy * dy > 25 { // Minimum distance threshold
                state.add_point((x, y));
                self.last_position = Some((x, y));
                self.is_modified = true;
            }
        }
        true
    }

    fn handle_mouse_up(&mut self, x: i32, y: i32, state: &mut ROIState) -> bool {
        match state.current_shape {
            Some(ROIShape::Polygon { .. }) => {
                // For polygons, add final point and continue drawing
                state.add_point((x, y));
            },
            Some(ROIShape::Line { .. }) => {
                // For lines, complete the drawing
                state.add_point((x, y));
                state.finish_drawing();
            },
            _ => {
                // For other shapes, complete the drawing
                state.finish_drawing();
            }
        }
        
        self.last_position = None;
        self.is_modified = false;
        true
    }

    fn handle_key_press(&mut self, key: char, state: &mut ROIState) -> bool {
        match key {
            '\r' | '\n' => { // Enter key
                if state.is_drawing {
                    if let Some(first_point) = state.points.first().copied() {
                        state.add_point(first_point); // Close polygon
                        state.finish_drawing();
                        self.is_modified = false;
                    }
                }
                true
            },
            '\x1b' => { // Escape key
                state.clear();
                self.last_position = None;
                self.is_modified = false;
                true
            },
            _ => false
        }
    }

    fn handle_cancel(&mut self, state: &mut ROIState) -> bool {
        state.clear();
        self.last_position = None;
        self.is_modified = false;
        true
    }

    fn handle_complete(&mut self, state: &mut ROIState) -> bool {
        if state.points.len() >= 3 {
            if let Some(first_point) = state.points.first().copied() {
                state.add_point(first_point); // Close polygon
                state.finish_drawing();
                self.is_modified = false;
                return true;
            }
        }
        false
    }

    pub fn convert_fltk_event(ev: Event) -> Option<ROIEvent> {
        match ev {
            Event::Push => {
                let coords = fltk::app::event_coords();
                Some(ROIEvent::MouseDown { x: coords.0, y: coords.1 })
            },
            Event::Drag => {
                let coords = fltk::app::event_coords();
                Some(ROIEvent::MouseDrag { x: coords.0, y: coords.1 })
            },
            Event::Released => {
                let coords = fltk::app::event_coords();
                Some(ROIEvent::MouseUp { x: coords.0, y: coords.1 })
            },
            Event::KeyUp => {
                if let Some(key) = std::char::from_u32(fltk::app::event_key() as u32) {
                    Some(ROIEvent::KeyPress(key))
                } else {
                    None
                }
            },
            _ => None
        }
    }
}