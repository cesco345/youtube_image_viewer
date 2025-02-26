//src/menu/edit/layers/mod.rs
mod dialog;
pub mod handlers;
mod color_filter;
mod interactive_tool;
mod color_tool;

pub use dialog::show_new_layer_dialog;  
pub use color_tool::start_interactive_color;
pub use handlers::*;  

