use fltk::{
    prelude::*,
    window::Window,
    button::Button,
    group::{Pack, PackType, Scroll},
    frame::Frame,
    enums::{Color, Align},
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn show_calibration_welcome_dialog() -> bool {
    println!("Opening calibration welcome dialog");
    let mut win = Window::default()
        .with_size(400, 400)
        .with_label("Image Calibration");
    win.set_color(Color::Background);

    let proceed = Arc::new(AtomicBool::new(false));

    // Add a scroll container
    let mut scroll = Scroll::new(10, 10, 380, 330, "");
    scroll.set_color(Color::Background);

    // Main vertical pack
    let mut main_pack = Pack::default()
        .with_size(360, 320);
    main_pack.set_spacing(15);

    // Title
    let mut title = Frame::default()
        .with_size(360, 25)
        .with_label("Image Calibration Guide:");
    title.set_label_color(Color::Foreground);
    title.set_align(Align::Left | Align::Inside);

    // Description
    let mut desc = Frame::default()
        .with_size(360, 40)
        .with_label("This tool helps you calibrate your image for accurate measurements.");
    desc.set_label_color(Color::Foreground);
    desc.set_align(Align::Left | Align::Inside | Align::Wrap);

    // Steps
    for step in [
        "1. Draw a line on a known distance in your image",
        "2. Enter the real-world distance and units",
        "3. Save calibrations for different objectives (5X, 10X, etc.)",
        "4. Export your calibrations for future reference"
    ] {
        let mut step_frame = Frame::default()
            .with_size(360, 25)
            .with_label(step);
        step_frame.set_align(Align::Left | Align::Inside);
        step_frame.set_label_color(Color::Foreground);
    }

    main_pack.end();
    scroll.end();

    // Button pack at the bottom, outside scroll area
    let mut button_pack = Pack::default()
        .with_size(380, 40)
        .with_pos(10, 350);
    button_pack.set_spacing(20);
    button_pack.set_type(PackType::Horizontal);

    let mut start_btn = Button::default()
        .with_size(180, 40)
        .with_label("Start Calibration");
    start_btn.set_color(Color::Light3);

    let mut cancel_btn = Button::default()
        .with_size(180, 40)
        .with_label("Cancel");
    cancel_btn.set_color(Color::Light3);

    button_pack.end();
    win.end();
    win.make_modal(true);

    start_btn.set_callback({
        let mut win = win.clone();
        let proceed = proceed.clone();
        move |_| {
            println!("Start button clicked");
            proceed.store(true, Ordering::SeqCst);
            win.hide();
        }
    });

    cancel_btn.set_callback({
        let mut win = win.clone();
        move |_| win.hide()
    });

    win.show();
    while win.shown() {
        fltk::app::wait();
    }

    let final_proceed = proceed.load(Ordering::SeqCst);
    println!("Dialog closed with proceed = {}", final_proceed);
    final_proceed
}