mod menu;
mod state;
mod utils;

use fltk::{
   app,
   frame::Frame,
   menu::{MenuBar, MenuFlag},
   prelude::*,
   window::Window,
   enums::Shortcut,
};
use fltk_theme::{ColorTheme, color_themes, widget_themes, WidgetTheme, ThemeType};
use std::{cell::RefCell, rc::Rc};
use state::ImageState;
use utils::MENU_HEIGHT;

fn main() {
   let app = app::App::default().with_scheme(app::Scheme::Gtk);
   
   // start with the initial dark theme
   let theme = ColorTheme::new(color_themes::DARK_THEME);
   theme.apply();

   let mut wind = Window::new(100, 100, 800, 600, "Image Viewer");
   let mut menu = MenuBar::new(0, 0, 800, MENU_HEIGHT, "");

   let frame = Rc::new(RefCell::new(Frame::new( 
       0,
       MENU_HEIGHT,
       800,
       600 - MENU_HEIGHT,
       "",
   )));
   frame.borrow_mut().set_frame(fltk::enums::FrameType::FlatBox);
   
   let state = Rc::new(RefCell::new(ImageState::new()));

   // file menu
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

   // edit menu
   let frame_crop = frame.clone();
   let state_crop = state.clone();
   menu.add("&Edit/&Cropping Images", Shortcut::None, MenuFlag::Normal, move |_| {
       menu::edit::crop::start_interactive_crop(&frame_crop, &state_crop);
   });
   
   // watermark menu items
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

   // basic Filters
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

   // advanced filters
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

   // filters Preview Toggle
   let frame_preview = frame.clone();
   let state_preview = state.clone();
   menu.add("&Edit/&Filters/Toggle Preview", Shortcut::None, MenuFlag::Normal, move |_| {
       menu::edit::filters::handle_toggle_preview(&frame_preview, &state_preview);
   });

   // theme options in view menu
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

   // other menus
   menu.add("&Image/", Shortcut::None, MenuFlag::Normal, |_| {});
   menu.add("&Info/", Shortcut::None, MenuFlag::Normal, |_| {});

   wind.end();
   wind.show();

   app.run().unwrap();
}










