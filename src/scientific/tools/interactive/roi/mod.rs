// src/scientific/tools/interactive/roi/mod.rs
mod renderer;
mod properties;
mod measurements;
mod interaction;
mod shapes;
mod utils;

pub use renderer::*;
pub use properties::*;
pub use measurements::*;
pub use interaction::*;
pub use shapes::*;
pub use utils::*;

// This will be our main entry point that coordinates everything
pub struct ROIManager {
    state: ROIState,
    renderer: ROIRenderer,
    interaction_handler: InteractionHandler,
}

impl ROIManager {
    pub fn new() -> Self {
        Self {
            state: ROIState::new(),
            renderer: ROIRenderer::new(),
            interaction_handler: InteractionHandler::new(),
        }
    }

    pub fn handle_event(&mut self, event: ROIEvent) -> bool {
        self.interaction_handler.handle(event, &mut self.state)
    }

    pub fn draw(&self, frame: &mut Frame) {
        self.renderer.draw(frame, &self.state);
    }
}