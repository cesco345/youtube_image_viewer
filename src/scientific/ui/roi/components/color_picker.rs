use fltk::{
    button::Button,
    frame::Frame,
    group::{Pack, PackType},
    prelude::*,
    enums::{Color, FrameType},
    valuator::ValueInput,
};
use std::{rc::Rc, cell::RefCell};

pub struct ColorPicker {
    group: Pack,
    color_preview: Frame,
    r_input: ValueInput,
    g_input: ValueInput,
    b_input: ValueInput,
    color_buttons: Vec<Button>,
    current_color: Rc<RefCell<(u8, u8, u8)>>,
}

impl ColorPicker {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str, initial_color: (u8, u8, u8)) -> Self {
        let mut group = Pack::new(x, y, w, h, Some(label));
        group.set_spacing(5);

        // Color preview and RGB inputs
        let mut preview_pack = Pack::new(0, 0, w, 30, None);
        preview_pack.set_type(PackType::Horizontal);
        preview_pack.set_spacing(5);

        let mut color_preview = Frame::new(0, 0, 50, 30, None);
        color_preview.set_frame(FrameType::DownBox);
        color_preview.set_color(Color::from_rgb(initial_color.0, initial_color.1, initial_color.2));

        let mut rgb_group = Pack::new(0, 0, w - 60, 30, None);
        rgb_group.set_type(PackType::Horizontal);
        rgb_group.set_spacing(5);

        let mut r_input = ValueInput::new(0, 0, 50, 30, Some("R"));
        let mut g_input = ValueInput::new(0, 0, 50, 30, Some("G"));
        let mut b_input = ValueInput::new(0, 0, 50, 30, Some("B"));

        for input in [&mut r_input, &mut g_input, &mut b_input].iter_mut() {
            input.set_range(0.0, 255.0);
            input.set_step(1.0, 1);
            input.set_precision(0);
        }

        r_input.set_value(initial_color.0 as f64);
        g_input.set_value(initial_color.1 as f64);
        b_input.set_value(initial_color.2 as f64);

        rgb_group.end();
        preview_pack.end();

        // Predefined color buttons
        let mut button_pack = Pack::new(0, 0, w, 30, None);
        button_pack.set_type(PackType::Horizontal);
        button_pack.set_spacing(5);

        let predefined_colors = [
            ((255, 0, 0), "Red"),
            ((0, 255, 0), "Green"),
            ((0, 0, 255), "Blue"),
            ((255, 255, 0), "Yellow"),
            ((255, 0, 255), "Magenta"),
            ((0, 255, 255), "Cyan"),
        ];

        let mut color_buttons = Vec::new();
        for (color, label) in predefined_colors.iter() {
            let mut btn = Button::new(0, 0, 80, 30, Some(*label));  // Dereference the label str
            btn.set_color(Color::from_rgb(color.0, color.1, color.2));
            color_buttons.push(btn);
        }

        button_pack.end();
        group.end();

        let mut picker = Self {
            group,
            color_preview,
            r_input,
            g_input,
            b_input,
            color_buttons,
            current_color: Rc::new(RefCell::new(initial_color)),
        };

        picker.setup_callbacks();
        picker
    }
    pub fn x(&self) -> i32 {
        self.group.x()
    }
    
    pub fn y(&self) -> i32 {
        self.group.y()
    }
    
    pub fn width(&self) -> i32 {
        self.group.width()
    }
    
    pub fn height(&self) -> i32 {
        self.group.height()
    }
    
    pub fn label(&self) -> String {
        self.group.label()
    }

    fn setup_callbacks(&mut self) {
        let preview = Rc::new(RefCell::new(self.color_preview.clone()));
        let current_color = self.current_color.clone();

        // RGB input callbacks
        let mut rgb_inputs = [
            (&mut self.r_input, 0),
            (&mut self.g_input, 1),
            (&mut self.b_input, 2),
        ];

        for (input, color_idx) in rgb_inputs.iter_mut() {
            let preview = preview.clone();
            let current_color = current_color.clone();
            let idx = *color_idx;

            input.set_callback(move |v| {
                let mut color = current_color.borrow_mut();
                let value = v.value() as u8;
                match idx {
                    0 => color.0 = value,
                    1 => color.1 = value,
                    2 => color.2 = value,
                    _ => unreachable!(),
                }
                if let Ok(mut preview) = preview.try_borrow_mut() {
                    preview.set_color(Color::from_rgb(color.0, color.1, color.2));
                    preview.redraw();
                }
            });
        }

        // Predefined color button callbacks
        let r_input = self.r_input.clone();
        let g_input = self.g_input.clone();
        let b_input = self.b_input.clone();
        
        for btn in self.color_buttons.iter_mut() {
            let preview = preview.clone();
            let current_color = current_color.clone();
            let r_input = Rc::new(RefCell::new(r_input.clone()));
            let g_input = Rc::new(RefCell::new(g_input.clone()));
            let b_input = Rc::new(RefCell::new(b_input.clone()));
            
            btn.set_callback(move |b| {
                let color = b.color().to_rgb();
                *current_color.borrow_mut() = color;
                
                if let Ok(mut preview) = preview.try_borrow_mut() {
                    preview.set_color(b.color());
                    preview.redraw();
                }
                
                if let Ok(mut r) = r_input.try_borrow_mut() {
                    r.set_value(color.0 as f64);
                }
                if let Ok(mut g) = g_input.try_borrow_mut() {
                    g.set_value(color.1 as f64);
                }
                if let Ok(mut b) = b_input.try_borrow_mut() {
                    b.set_value(color.2 as f64);
                }
            });
        }
    }

    pub fn get_color(&self) -> (u8, u8, u8) {
        *self.current_color.borrow()
    }

    pub fn set_color(&mut self, color: (u8, u8, u8)) {
        *self.current_color.borrow_mut() = color;
        self.color_preview.set_color(Color::from_rgb(color.0, color.1, color.2));
        self.r_input.set_value(color.0 as f64);
        self.g_input.set_value(color.1 as f64);
        self.b_input.set_value(color.2 as f64);
        self.color_preview.redraw();
    }
}