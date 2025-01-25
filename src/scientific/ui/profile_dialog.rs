use fltk::{
    window::Window,
    prelude::*,
};
use crate::scientific::analysis::IntensityProfile;

pub fn show_profile_dialog(profile: &IntensityProfile) {
    let mut window = Window::default()
        .with_size(400, 300)
        .with_label("Intensity Profile");
    window.make_modal(true);

    // TODO: Add plot using profile.x_values and profile.intensities

    window.end();
    window.show();

    while window.shown() {
        fltk::app::wait();
    }
}