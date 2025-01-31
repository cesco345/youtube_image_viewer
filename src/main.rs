mod menu;
mod state;
mod utils;
mod scientific;

use fltk::{
    app,
    frame::Frame,
    menu::{MenuBar, MenuFlag},
    prelude::*,
    window::Window,
    enums::{Shortcut, Event, CallbackTrigger},
};
use fltk_theme::{ColorTheme, color_themes, WidgetTheme, ThemeType};
use std::{cell::RefCell, rc::Rc};
use state::ImageState;
use utils::MENU_HEIGHT;
use crate::menu::scientific::analysis::cell::setup_cell_analysis_menu;
use crate::scientific::tools::interactive::cell_analysis_tool::CellAnalysisState;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let theme = ColorTheme::new(color_themes::DARK_THEME);
    theme.apply();

    let mut wind = Window::new(100, 100, 800, 600, "Image Viewer");
    let mut menu = MenuBar::new(0, 0, 800, MENU_HEIGHT, "");

    let frame = Rc::new(RefCell::new(Frame::new(0, MENU_HEIGHT, 800, 600 - MENU_HEIGHT, "")));
    frame.borrow_mut().set_frame(fltk::enums::FrameType::FlatBox);
    let state = Rc::new(RefCell::new(ImageState::new()));

    // Set up frame event handling
    frame.borrow_mut().set_trigger(CallbackTrigger::Release);
    

    // File Menu
    let frame_open = frame.clone();
    let state_open = state.clone();
    menu.add("&File/&Open...", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::file::open::handle_open(&frame_open, &state_open);
    });

    let frame_save = frame.clone();
    let state_save = state.clone();
    menu.add("&File/&Save", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::file::save::handle_save(&frame_save, &state_save);
    });

    let frame_save_as = frame.clone();
    let state_save_as = state.clone();
    menu.add("&File/Save &As...", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::file::save::handle_save_as(&frame_save_as, &state_save_as);
    });

    menu.add("&File/Recent Files/", Shortcut::None, MenuFlag::Submenu, |_| {});
    menu.add("&File/Exit", Shortcut::None, MenuFlag::Normal, |_| {
        menu::file::exit::handle_exit();
    });

    // Edit Menu
    let frame_crop = frame.clone();
    let state_crop = state.clone();
    menu.add("&Edit/&Cropping Images", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::crop::start_interactive_crop(&frame_crop, &state_crop);
    });

    // Watermark Menu
    let frame_add = frame.clone();
    let state_add = state.clone();
    menu.add("&Edit/&Watermark/&Add Watermark", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::watermark::dialog::show_watermark_dialog(&frame_add, &state_add);
    });

    let frame_edit = frame.clone();
    let state_edit = state.clone();
    menu.add("&Edit/&Watermark/&Edit Watermark", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::watermark::handle_edit_watermark(&frame_edit, &state_edit);
    });

    let frame_remove = frame.clone();
    let state_remove = state.clone();
    menu.add("&Edit/&Watermark/&Eraser Tool", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::watermark::handle_remove_watermark(&frame_remove, &state_remove);
    });

    let frame_preview_wm = frame.clone();
    let state_preview_wm = state.clone();
    menu.add("&Edit/&Watermark/Toggle &Preview", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::watermark::handle_toggle_preview(&frame_preview_wm, &state_preview_wm);
    });

    // Basic Filters
    let frame_basic_gray = frame.clone();
    let state_basic_gray = state.clone();
    menu.add("&Edit/&Filters/&Basic/&Grayscale", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::filters::show_filter_dialog(&frame_basic_gray, &state_basic_gray, "grayscale");
    });

    let frame_basic_sepia = frame.clone();
    let state_basic_sepia = state.clone();
    menu.add("&Edit/&Filters/&Basic/&Sepia", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::filters::show_filter_dialog(&frame_basic_sepia, &state_basic_sepia, "sepia");
    });

    let frame_basic_bright = frame.clone();
    let state_basic_bright = state.clone();
    menu.add("&Edit/&Filters/&Basic/&Brightness", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::filters::show_filter_dialog(&frame_basic_bright, &state_basic_bright, "brightness");
    });

    let frame_basic_contrast = frame.clone();
    let state_basic_contrast = state.clone();
    menu.add("&Edit/&Filters/&Basic/&Contrast", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::filters::show_filter_dialog(&frame_basic_contrast, &state_basic_contrast, "contrast");
    });

    let frame_basic_sat = frame.clone();
    let state_basic_sat = state.clone();
    menu.add("&Edit/&Filters/&Basic/&Saturation", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::filters::show_filter_dialog(&frame_basic_sat, &state_basic_sat, "saturation");
    });

    let frame_basic_threshold = frame.clone();
    let state_basic_threshold = state.clone();
    menu.add("&Edit/&Filters/&Basic/&Threshold", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::filters::show_filter_dialog(&frame_basic_threshold, &state_basic_threshold, "threshold");
    });

    let frame_basic_hue = frame.clone();
    let state_basic_hue = state.clone();
    menu.add("&Edit/&Filters/&Basic/&Hue", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::filters::show_filter_dialog(&frame_basic_hue, &state_basic_hue, "hue");
    });

    // Advanced Filters
    let frame_advanced_gaussian = frame.clone();
    let state_advanced_gaussian = state.clone();
    menu.add("&Edit/&Filters/&Advanced/&Convolution", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::filters::show_filter_dialog(&frame_advanced_gaussian, &state_advanced_gaussian, "gaussian_blur");
    });

    let frame_advanced_edge = frame.clone();
    let state_advanced_edge = state.clone();
    menu.add("&Edit/&Filters/&Advanced/&Edge Detection", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::filters::show_filter_dialog(&frame_advanced_edge, &state_advanced_edge, "edge_detection");
    });

    let frame_advanced_noise = frame.clone();
    let state_advanced_noise = state.clone();
    menu.add("&Edit/&Filters/&Advanced/&Noise", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::filters::show_filter_dialog(&frame_advanced_noise, &state_advanced_noise, "noise");
    });

    let frame_advanced_vignette = frame.clone();
    let state_advanced_vignette = state.clone();
    menu.add("&Edit/&Filters/&Advanced/&Vignette", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::filters::show_filter_dialog(&frame_advanced_vignette, &state_advanced_vignette, "vignette");
    });

    let frame_advanced_posterize = frame.clone();
    let state_advanced_posterize = state.clone();
    menu.add("&Edit/&Filters/&Advanced/&Posterize", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::filters::show_filter_dialog(&frame_advanced_posterize, &state_advanced_posterize, "posterize");
    });

    let frame_advanced_pixelate = frame.clone();
    let state_advanced_pixelate = state.clone();
    menu.add("&Edit/&Filters/&Advanced/Pi&xelate", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::filters::show_filter_dialog(&frame_advanced_pixelate, &state_advanced_pixelate, "pixelate");
    });

    let frame_advanced_motion = frame.clone();
    let state_advanced_motion = state.clone();
    menu.add("&Edit/&Filters/&Advanced/&Motion Blur", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::filters::show_filter_dialog(&frame_advanced_motion, &state_advanced_motion, "motion_blur");
    });

    // Layers Menu
    let frame_layer = frame.clone();
    let state_layer = state.clone();
    menu.add("&Edit/&Layers/&Layer Manager", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::layers::show_new_layer_dialog(&frame_layer, &state_layer);
    });

    let frame_layer_preview = frame.clone();
    let state_layer_preview = state.clone();
    menu.add("&Edit/&Layers/Toggle &Preview", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::layers::handle_toggle_preview(&frame_layer_preview, &state_layer_preview);
    });

    // Cell Analysis setup
    setup_cell_analysis_menu(&mut menu, &frame, &state);

    // Scientific Menu
    let frame_channels = frame.clone();
    let frame_roi = frame.clone();
    let frame_profile = frame.clone();
    let frame_scale = frame.clone();
    let frame_metadata = frame.clone();
    let state_channels = state.clone();
    let state_roi = state.clone();
    let state_profile = state.clone();
    let state_scale = state.clone();
    let state_metadata = state.clone();
    let frame_legend = frame.clone();
    let state_legend = state.clone();
    let frame_preview_layer = frame.clone();
let state_preview_layer = state.clone();
menu.add(
    "&Scientific/&View/Toggle Preview Layer",
    Shortcut::None,
    MenuFlag::Normal,
    move |_| {
        if let Ok(mut state_ref) = state_preview_layer.try_borrow_mut() {
            state_ref.scientific_state.toggle_preview_layer();
            frame_preview_layer.borrow_mut().redraw();
        }
    }
);
let frame_base_image = frame.clone();
let state_base_image = state.clone();
menu.add(
    "&Scientific/&View/Toggle Base Image",
    Shortcut::None,
    MenuFlag::Normal,
    move |_| {
        if let Ok(mut state_ref) = state_base_image.try_borrow_mut() {
            state_ref.scientific_state.toggle_base_image();
            frame_base_image.borrow_mut().redraw();
        }
    }
);
let frame_drawing = frame.clone();
let state_drawing = state.clone();
menu.add(
    "&Scientific/&View/Toggle Drawing Layer",
    Shortcut::None,
    MenuFlag::Normal,
    move |_| {
        if let Ok(mut state_ref) = state_drawing.try_borrow_mut() {
            state_ref.scientific_state.toggle_drawing_layer();
            frame_drawing.borrow_mut().redraw();
        }
    }
);

    menu.add("&Scientific/&Channel Manager", Shortcut::None, MenuFlag::Normal, move |_| {
        scientific::ui::show_channel_manager(&frame_channels, &state_channels);
    });

    // Scientific tool invocations
    menu.add("&Scientific/&Measurements/Calibrations", Shortcut::None, MenuFlag::Normal, move |_| {
        scientific::tools::handlers::handle_start_calibration(&frame_scale, &state_scale);
    });
    
    menu.add("&Scientific/&Measurements/Toggle Scale Legend", Shortcut::None, MenuFlag::Normal, move |_| {
        scientific::tools::handlers::handle_toggle_scale_legend(&frame_legend, &state_legend);
    });

    menu.add("&Scientific/&Measurements/ROI", Shortcut::None, MenuFlag::Normal, move |_| {
        scientific::tools::interactive::start_interactive_roi(&frame_roi, &state_roi);
    });

    menu.add("&Scientific/&Measurements/Line Profile", Shortcut::None, MenuFlag::Normal, move |_| {
        scientific::tools::interactive::start_interactive_profile(&frame_profile, &state_profile);
    });

    menu.add("&Scientific/&Metadata/Edit Properties", Shortcut::None, MenuFlag::Normal, move |_| {
        scientific::tools::interactive::start_metadata_editor(&frame_metadata, &state_metadata);
    });
    

    // Filters Preview Toggle
    let frame_preview = frame.clone();
    let state_preview = state.clone();
    menu.add("&Edit/&Filters/Toggle Preview", Shortcut::None, MenuFlag::Normal, move |_| {
        menu::edit::filters::handle_toggle_preview(&frame_preview, &state_preview);
    });

    // Theme Options
    menu.add("&View/&Themes/Color Themes/Dark", Shortcut::None, MenuFlag::Normal, |_| {
        let theme = ColorTheme::new(color_themes::DARK_THEME);
        theme.apply();
    });

    menu.add("&View/&Themes/Color Themes/Black", Shortcut::None, MenuFlag::Normal, |_| {
        let theme = ColorTheme::new(color_themes::BLACK_THEME);
        theme.apply();
    });

    menu.add("&View/&Themes/Color Themes/Gray", Shortcut::None, MenuFlag::Normal, |_| {
        let theme = ColorTheme::new(color_themes::GRAY_THEME);
        theme.apply();
    });

    menu.add("&View/&Themes/Widget Themes/Dark", Shortcut::None, MenuFlag::Normal, |_| {
        let widget_theme = WidgetTheme::new(ThemeType::Dark);
        widget_theme.apply();
    });

    menu.add("&View/&Themes/Widget Themes/Classic", Shortcut::None, MenuFlag::Normal, |_| {
        let widget_theme = WidgetTheme::new(ThemeType::Classic);
        widget_theme.apply();
    });

    menu.add("&Image/", Shortcut::None, MenuFlag::Normal, |_| {});
    menu.add("&Info/", Shortcut::None, MenuFlag::Normal, |_| {});

    wind.end();
    wind.show();
    app.run().unwrap();
}










