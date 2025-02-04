// src/scientific/ui/roi/mod.rs
mod mode_dialog;
mod batch_dialog;
mod components;
mod export_dialog;
mod measurement_dialog;
mod properties_dialog;

// Public exports
pub use mode_dialog::*;
pub use batch_dialog::{BatchDialog, show_batch_analysis_dialog};

pub use batch_dialog::*;
pub use export_dialog::*;
pub use measurement_dialog::*;
pub use properties_dialog::*;

pub use self::batch_dialog::BatchStatistics;