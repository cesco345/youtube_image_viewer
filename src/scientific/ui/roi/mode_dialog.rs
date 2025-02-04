use fltk::{
    window::Window,
    button::Button,
    group::Pack,
    prelude::*,
    enums::Color,
};
use std::rc::Rc;
use crate::scientific::types::ROIShape;

pub fn show_roi_mode_dialog(on_select: impl Fn(ROIShape) + 'static) {
    let mut window = Window::default()
        .with_size(200, 250)
        .with_label("Select ROI Type");
    
    // Center the window on screen
    window.set_pos(
        (fltk::app::screen_size().0 / 2.0 - 100.0) as i32,
        (fltk::app::screen_size().1 / 2.0 - 125.0) as i32
    );
    
    // Set window background color to a darker gray
    window.set_color(Color::from_rgb(80, 80, 80));
    
    let mut pack = Pack::new(10, 10, 180, 230, "");
    pack.set_spacing(8);
    // Make pack background match window
    pack.set_color(Color::from_rgb(80, 80, 80));
    
    const BTN_HEIGHT: i32 = 40;
    // Button colors
    let btn_color = Color::from_rgb(100, 100, 100);
    let btn_selection_color = Color::from_rgb(120, 120, 120);
    let btn_label_color = Color::White;
    
    // Helper function to style buttons consistently
    let style_button = |mut btn: Button| {
        btn.set_color(btn_color);
        btn.set_selection_color(btn_selection_color);
        btn.set_label_color(btn_label_color);
        btn.set_frame(fltk::enums::FrameType::FlatBox);
        btn
    };

    // Create and style all buttons
    let mut rect_button = style_button(Button::new(0, 0, 180, BTN_HEIGHT, "Rectangle"));
    let mut ellipse_button = style_button(Button::new(0, 0, 180, BTN_HEIGHT, "Ellipse"));
    let mut poly_button = style_button(Button::new(0, 0, 180, BTN_HEIGHT, "Polygon"));
    let mut line_button = style_button(Button::new(0, 0, 180, BTN_HEIGHT, "Line"));
    let mut close_button = style_button(Button::new(0, 0, 180, BTN_HEIGHT, "Cancel"));

    let on_select = Rc::new(on_select);
    
    {
        let mut win = window.clone();
        let on_select = on_select.clone();
        rect_button.set_callback(move |_| {
            on_select(ROIShape::Rectangle { width: 0, height: 0 });
            win.hide();
        });
    }
    
    {
        let mut win = window.clone();
        let on_select = on_select.clone();
        ellipse_button.set_callback(move |_| {
            on_select(ROIShape::Ellipse { width: 0, height: 0 });
            win.hide();
        });
    }
    
    {
        let mut win = window.clone();
        let on_select = on_select.clone();
        poly_button.set_callback(move |_| {
            on_select(ROIShape::Polygon { points: Vec::new() });
            win.hide();
        });
    }
    
    {
        let mut win = window.clone();
        let on_select = on_select.clone();
        line_button.set_callback(move |_| {
            on_select(ROIShape::Line { points: Vec::new() });
            win.hide();
        });
    }
    
    {
        let mut win = window.clone();
        close_button.set_callback(move |_| {
            win.hide();
        });
    }
    
    pack.end();
    window.end();
    window.make_modal(true);
    window.show();
}