pub mod analysis;
pub mod calibration;
pub mod layers;
pub mod state;
pub mod tools;
pub mod ui;
pub mod types;
pub mod rendering;
pub mod reporting;
pub mod visualization;

pub use layers::{Channel, Annotation, AnnotationType, Metadata};
pub use analysis::{IntensityProfile, ColocalizationAnalysis};
pub use calibration::SpatialCalibration;
pub use state::ScientificState;
pub use types::{LegendPosition, ROIShape, ROITool, MeasurementTool};
pub use reporting::CalibrationReport;
pub use visualization::CellVisualizer;