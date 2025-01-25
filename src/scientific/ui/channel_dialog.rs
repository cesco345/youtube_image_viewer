// src/scientific/ui/channel_dialog.rs
use fltk::{
    window::Window,
    button::{Button, CheckButton},
    frame::Frame,
    group::{Pack, Scroll, PackType},
    input::{FloatInput, Input},
    menu::Choice,
    prelude::*,
    enums::{Color, FrameType},
};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;
use crate::scientific::Channel;

pub fn show_channel_manager(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) -> bool {
    let mut window = Window::default()
        .with_size(400, 500)
        .with_label("Channel Manager");
    window.make_modal(true);

    let scroll = Scroll::new(10, 10, 380, 380, "");
    scroll.begin();
    let mut pack = Pack::new(0, 0, 360, 380, "");
    pack.set_spacing(5);

    let channel_list = create_channel_list(&mut pack, state.clone());
    pack.end();
    scroll.end();

    let wavelength_input = FloatInput::new(120, 400, 70, 25, "Wavelength (nm):");
    let mut pseudo_color = Choice::new(120, 430, 100, 25, "Pseudo-color:");
    pseudo_color.add_choice("Red|Green|Blue|Cyan|Magenta|Yellow");

    let exposure_input = FloatInput::new(120, 460, 70, 25, "Exposure (ms):");
    let gain_input = FloatInput::new(270, 460, 70, 25, "Gain:");

    let mut add_btn = Button::new(230, 430, 70, 25, "Add");
    setup_add_button(&mut add_btn, wavelength_input.clone(), pseudo_color.clone(), frame.clone(), state.clone());

    window.end();
    window.show();

    while window.shown() {
        fltk::app::wait();
    }

    true
}

fn create_channel_list(pack: &mut Pack, state: Rc<RefCell<ImageState>>) -> Pack {
    let mut channel_pack = Pack::new(0, 0, pack.width(), 30, "");
    channel_pack.set_type(PackType::Horizontal);
    channel_pack.set_spacing(5);

    let state_ref = state.borrow();
    for (i, channel) in state_ref.scientific_state.channels.iter().enumerate() {
        let mut row = Pack::new(0, 0, channel_pack.width(), 25, "");
        row.set_type(PackType::Horizontal);
        
        let mut visible = CheckButton::new(0, 0, 25, 25, "");
        visible.set_value(channel.visible);
        
        let mut name = Frame::new(0, 0, 100, 25, &*channel.name);
        name.set_frame(FrameType::FlatBox);
        
        let mut wavelength = Frame::new(0, 0, 70, 25, format!("{:.1}nm", channel.wavelength).as_str());
        wavelength.set_frame(FrameType::FlatBox);

        let mut delete_btn = Button::new(0, 0, 60, 25, "Delete");
        let state_clone = state.clone();
        let channel_idx = i;
        delete_btn.set_callback(move |_| {
            if let Ok(mut state_ref) = state_clone.try_borrow_mut() {
                state_ref.scientific_state.channels.remove(channel_idx);
            }
        });

        row.end();
    }
    
    channel_pack.end();
    channel_pack
}

fn setup_add_button(
    add_btn: &mut Button,
    wavelength_input: FloatInput,
    pseudo_color: Choice,
    frame: Rc<RefCell<Frame>>,
    state: Rc<RefCell<ImageState>>,
) {
    add_btn.set_callback(move |_| {
        if let Ok(wavelength) = wavelength_input.value().parse::<f32>() {
            let mut state_ref = state.borrow_mut();
            if let Some(ref img) = state_ref.image {
                let color = match pseudo_color.value() {
                    0 => (255, 0, 0),    // Red
                    1 => (0, 255, 0),    // Green
                    2 => (0, 0, 255),    // Blue
                    3 => (0, 255, 255),  // Cyan
                    4 => (255, 0, 255),  // Magenta
                    5 => (255, 255, 0),  // Yellow
                    _ => (255, 255, 255), // White
                };

                let channel = Channel {
                    name: format!("Channel {}", wavelength),
                    image: img.clone(),
                    visible: true,
                    wavelength,
                    opacity: 1.0,
                    pseudo_color: color,
                    metadata: Default::default(),
                };

                state_ref.scientific_state.add_channel(channel);
                frame.borrow_mut().redraw();
            }
        }
    });
}