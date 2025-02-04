//src/scientific/ui/roi/components/line_style_selector.rs

use fltk::{
    group::{Pack, PackType},
    button::RadioRoundButton,  
    frame::Frame,
    valuator::ValueInput,
    prelude::*,
    draw::LineStyle,
    enums::{FrameType, Align},
};

pub struct LineStyleSelector {
    group: Pack,
    style_buttons: Vec<RadioRoundButton>,
    width_input: ValueInput,
    current_style: LineStyle,
    current_width: i32,
}

impl LineStyleSelector {
    pub fn new(
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        label: &str,
        initial_style: LineStyle
    ) -> Self {
        let mut group = Pack::new(x, y, w, h, Some(label));
        group.set_spacing(5);

        // Style selection
        let mut style_pack = Pack::new(0, 0, w, 30, None);
        style_pack.set_type(PackType::Horizontal);
        style_pack.set_spacing(10);

        let mut style_buttons = Vec::new();
        let style_options = [
            (LineStyle::Solid, "Solid"),
            (LineStyle::Dash, "Dashed"),
            (LineStyle::Dot, "Dotted"),
        ];

        for (style, label) in &style_options {
            let mut btn = RadioRoundButton::new(0, 0, 80, 30, Some(*label));
            if *style == initial_style {
                btn.set_value(true);
            }
            style_buttons.push(btn);
        }

        style_pack.end();

        // Line width input
        let mut width_pack = Pack::new(0, 0, w, 30, None);
        width_pack.set_type(PackType::Horizontal);
        width_pack.set_spacing(5);

        let mut width_label = Frame::new(0, 0, 80, 30, Some("Line Width:"));
        width_label.set_align(Align::Left | Align::Inside);

        let mut width_input = ValueInput::new(0, 0, 60, 30, None);
        width_input.set_range(1.0, 10.0);
        width_input.set_step(1.0, 1);
        width_input.set_precision(0);
        width_input.set_value(2.0);  // Default width

        width_pack.end();
        group.end();

        let mut selector = Self {
            group,
            style_buttons,
            width_input,
            current_style: initial_style,
            current_width: 2,
        };

        selector.setup_callbacks();
        selector
    }

    fn setup_callbacks(&mut self) {
        // Create a shared current_style that can be modified in callbacks
        let current_style = std::rc::Rc::new(std::cell::RefCell::new(self.current_style));
        
        for (idx, btn) in self.style_buttons.iter_mut().enumerate() {
            let current_style = current_style.clone();
            btn.set_callback(move |b| {
                if b.value() {
                    let new_style = match idx {
                        0 => LineStyle::Solid,
                        1 => LineStyle::Dash,
                        2 => LineStyle::Dot,
                        _ => LineStyle::Solid,
                    };
                    *current_style.borrow_mut() = new_style;
                }
            });
        }

        // Create a shared current_width that can be modified in callbacks
        let current_width = std::rc::Rc::new(std::cell::RefCell::new(self.current_width));
        let width_clone = current_width.clone();
        
        self.width_input.set_callback(move |v| {
            *width_clone.borrow_mut() = v.value() as i32;
        });
    }

    pub fn get_style(&self) -> LineStyle {
        self.current_style
    }

    pub fn get_width(&self) -> i32 {
        self.current_width 
    }

    pub fn set_style(&mut self, style: LineStyle) {
        self.current_style = style;
        for (idx, btn) in self.style_buttons.iter_mut().enumerate() {
            let should_set = matches!(
                (idx, style),
                (0, LineStyle::Solid) |
                (1, LineStyle::Dash) |
                (2, LineStyle::Dot)
            );
            btn.set_value(should_set);
        }
    }

    pub fn set_width(&mut self, width: i32) {
        self.current_width = width;
        self.width_input.set_value(width as f64);
    }
}