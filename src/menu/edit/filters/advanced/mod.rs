// menu/edit/filters/advanced/mod.rs
mod convolution;
mod edge_detection;
mod noise;
mod vignette;
mod posterize;
mod pixelate;
mod motion_blur;

pub use edge_detection::{EdgeDetectionFilter, EdgeDetectionMethod};
pub use noise::NoiseFilter;
pub use vignette::VignetteFilter;
pub use posterize::PosterizeFilter;
pub use pixelate::PixelateFilter;
pub use motion_blur::MotionBlurFilter;

pub use convolution::{ConvolutionFilter, ConvolutionType};