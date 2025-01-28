use fltk::{
    window::Window,
    button::{Button, CheckButton},
    frame::Frame,
    group::{Pack, Scroll, PackType},
    input::{FloatInput, Input},
    menu::Choice,
    prelude::*,
    enums::{Color, FrameType, Align},
};
use std::{rc::Rc, cell::RefCell};
use crate::state::ImageState;
use crate::scientific::Channel;

pub fn show_channel_manager(frame: &Rc<RefCell<Frame>>, state: &Rc<RefCell<ImageState>>) -> bool {
    let mut window = Window::default()
        .with_size(500, 600)  // Made window larger
        .with_label("Channel Manager");
    window.set_color(Color::Dark2);
    window.make_modal(true);

    let mut scroll = Scroll::new(10, 10, 480, 480, "");  // Made scroll area larger
    scroll.set_color(Color::Dark2);
    scroll.begin();
    let mut pack = Pack::new(0, 0, 460, 480, "");
    pack.set_spacing(5);
    pack.set_color(Color::Dark2);

    let channel_list = create_channel_list(&mut pack, state.clone());
    pack.end();
    scroll.end();

    // Input area
    let mut inputs_pack = Pack::new(10, 500, 480, 90, "");
    inputs_pack.set_spacing(10);
    inputs_pack.set_color(Color::Dark2);

    let mut wavelength_label = Frame::new(0, 0, 110, 25, "Wavelength (nm):");
    wavelength_label.set_label_color(Color::White);
    let mut wavelength_input = FloatInput::new(120, 500, 70, 25, "");
    wavelength_input.set_color(Color::Dark3);
    wavelength_input.set_text_color(Color::White);

    let mut pseudo_color_label = Frame::new(200, 500, 100, 25, "Pseudo-color:");
    pseudo_color_label.set_label_color(Color::White);
    let mut pseudo_color = Choice::new(310, 500, 100, 25, "");
    pseudo_color.add_choice("Red|Green|Blue|Cyan|Magenta|Yellow");
    pseudo_color.set_color(Color::Dark3);
    pseudo_color.set_text_color(Color::White);

    // Add button
    let mut add_btn = Button::new(420, 500, 60, 25, "Add");
    add_btn.set_color(Color::Dark3);
    add_btn.set_label_color(Color::White);

    inputs_pack.end();

    // Headers for channel list
    let mut headers_pack = Pack::new(10, 45, 460, 25, "");
    headers_pack.set_type(PackType::Horizontal);
    headers_pack.set_spacing(5);

    let vis_header = Frame::new(0, 0, 25, 25, "");
    let mut name_header = Frame::new(0, 0, 100, 25, "Name");
    name_header.set_label_color(Color::White);
    let mut wave_header = Frame::new(0, 0, 100, 25, "Wavelength");
    wave_header.set_label_color(Color::White);
    let mut scale_header = Frame::new(0, 0, 175, 25, "Scale");
    scale_header.set_label_color(Color::White);
    let action_header = Frame::new(0, 0, 60, 25, "");

    headers_pack.end();

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
    channel_pack.set_type(PackType::Vertical);  // Changed to vertical layout
    channel_pack.set_spacing(5);
    channel_pack.set_color(Color::Dark2);

    let state_ref = state.borrow();
    for (i, channel) in state_ref.scientific_state.channels.iter().enumerate() {
        let mut row = Pack::new(0, 0, channel_pack.width(), 35, "");
        row.set_type(PackType::Horizontal);
        row.set_spacing(5);
        row.set_color(Color::Dark2);
        
        let mut visible = CheckButton::new(0, 0, 25, 35, "");
        visible.set_value(channel.visible);
        visible.set_color(Color::Dark3);
        
        let mut name = Frame::new(0, 0, 100, 35, &*channel.name);
        name.set_frame(FrameType::FlatBox);
        name.set_label_color(Color::White);
        
        // Wavelength info
        let wavelength_text = format!("{:.1}nm", channel.wavelength);
        let mut wavelength = Frame::new(0, 0, 100, 35, &*wavelength_text);
        
        // For scale info
        let scale_text = if let Some((ppu, unit)) = &channel.metadata.scale_calibration {
            format!("{:.2} px/{}", ppu, unit)
        } else {
            "Not calibrated".to_string()
        };
        let mut scale_info = Frame::new(0, 0, 175, 35, &*scale_text);
        scale_info.set_frame(FrameType::FlatBox);
        scale_info.set_label_color(Color::White);

        let mut delete_btn = Button::new(0, 0, 60, 35, "Delete");
        delete_btn.set_color(Color::Dark3);
        delete_btn.set_label_color(Color::White);
        
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