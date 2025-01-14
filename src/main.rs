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
    dialog::{FileDialog, FileDialogType, alert, message},
};
use std::{cell::RefCell, path::PathBuf, rc::Rc};
use state::ImageState;
use utils::MENU_HEIGHT;

fn main() {
    let app = app::App::default();
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

    // File menu
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

    
    let frame_crop = frame.clone();
let state_crop = state.clone();
menu.add("&Edit/&Crop", Shortcut::None, MenuFlag::Normal, move |_| {
    menu::edit::crop::handle_crop(&frame_crop, &state_crop);
});
// Other menus as stubs for now
    menu.add("&View/", Shortcut::None, MenuFlag::Normal, |_| {});
    menu.add("&Image/", Shortcut::None, MenuFlag::Normal, |_| {});
    menu.add("&Info/", Shortcut::None, MenuFlag::Normal, |_| {});

    wind.end();
    wind.show();

    app.run().unwrap();
}










