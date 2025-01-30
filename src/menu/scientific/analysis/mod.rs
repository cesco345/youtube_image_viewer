// src/menu/scientific/analysis/mod.rs
pub mod handlers;
pub mod cell;

// Re-export the handlers so they can be used via super::handlers
pub use handlers::{
    handle_cell_analysis,
    handle_batch_analysis,
    handle_show_statistics,
    handle_export_analysis,
    handle_stop_analysis,
};