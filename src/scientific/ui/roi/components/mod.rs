mod color_picker;
mod line_style_selector;
pub mod measurement_table;

pub use color_picker::ColorPicker;
pub use line_style_selector::LineStyleSelector;
pub use measurement_table::MeasurementTable;  // Make public

// Common UI constants
pub const PADDING: i32 = 5;
pub const BUTTON_HEIGHT: i32 = 25;
pub const INPUT_HEIGHT: i32 = 25;
pub const ROW_HEIGHT: i32 = 25;
pub const HEADER_HEIGHT: i32 = 30;

// Common UI color scheme
pub const PRIMARY_COLOR: (u8, u8, u8) = (0, 121, 194);    // Blue
pub const SECONDARY_COLOR: (u8, u8, u8) = (88, 88, 88);   // Gray
pub const SUCCESS_COLOR: (u8, u8, u8) = (40, 167, 69);    // Green
pub const WARNING_COLOR: (u8, u8, u8) = (255, 193, 7);    // Yellow
pub const ERROR_COLOR: (u8, u8, u8) = (220, 53, 69);      // Red

// Font settings
pub const DEFAULT_FONT_SIZE: i32 = 12;
pub const HEADER_FONT_SIZE: i32 = 14;
pub const TITLE_FONT_SIZE: i32 = 16;