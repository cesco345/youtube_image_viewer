//src/scientific/ui/roi/properties_dialog.rs
use fltk::{
    window::{Window, DoubleWindow},
    group::Pack,
    button::{Button, CheckButton},
    input::Input,
    frame::Frame,
    enums::Align,
    draw::LineStyle as FltkLineStyle,
    prelude::*,
};
use std::{rc::Rc, cell::RefCell};
use crate::scientific::{
    tools::interactive::roi::properties::ROIState,
    types::ROITool,
};
use super::components::{ColorPicker, LineStyleSelector};


// In src/scientific/ui/roi/properties_dialog.rs
impl Clone for ColorPicker {
    fn clone(&self) -> Self {
        Self::new(
            self.x(),
            self.y(),
            self.width(),
            self.height(),
            &self.label(),
            self.get_color()
        )
    }
}

pub struct PropertiesDialog {
    window: DoubleWindow,
    label_input: Input,
    color_picker: ColorPicker,
    fill_color_picker: ColorPicker,
    line_style_selector: LineStyleSelector,
    line_width_input: Input,
    show_measurements: CheckButton,
    is_locked: CheckButton,
    apply_btn: Button,
    close_btn: Button,
    state: Rc<RefCell<ROIState>>,
}

impl PropertiesDialog {
    pub fn new(
        parent: Rc<RefCell<Window>>,
        state: Rc<RefCell<ROIState>>,
        roi: &ROITool
    ) -> Self {
        let mut window = DoubleWindow::new(
            0, 0, 300, 400,
            "ROI Properties"
        );

        // Center dialog relative to parent
        let parent = parent.borrow();
        let x = parent.x() + (parent.width() - window.width()) / 2;
        let y = parent.y() + (parent.height() - window.height()) / 2;
        window.set_pos(x, y);

        let mut pack = Pack::new(10, 10, 280, 380, "");
        pack.set_spacing(10);

        // Label section
        let mut label_frame = Frame::new(0, 0, 280, 25, "Label");
        label_frame.set_align(Align::Left | Align::Inside);
        let mut label_input = Input::new(0, 0, 280, 25, "");
        label_input.set_value(&roi.shape.to_string());

        // Color pickers
        let color_picker = ColorPicker::new(
            0, 0, 280, 30,
            "Outline Color",
            roi.color
        );

        let fill_color_picker = ColorPicker::new(
            0, 0, 280, 30,
            "Fill Color", 
            (0, 0, 0)  // Default fill color
        );

        // Line style selector
        let line_style_selector = LineStyleSelector::new(
            0, 0, 280, 30,
            "Line Style",
            FltkLineStyle::Solid  // Use fltk's LineStyle
        );

        // Line width input
        let mut line_width_frame = Frame::new(0, 0, 280, 25, "Line Width");
        line_width_frame.set_align(Align::Left | Align::Inside);
        let mut line_width_input = Input::new(0, 0, 280, 25, "");
        line_width_input.set_value(&roi.line_width.to_string());

        // Checkboxes
        let show_measurements = CheckButton::new(0, 0, 280, 25, "Show Measurements");
        show_measurements.set_checked(true);

        let is_locked = CheckButton::new(0, 0, 280, 25, "Lock ROI");
        is_locked.set_checked(false);

        // Buttons
        let apply_btn = Button::new(0, 0, 135, 25, "Apply");
        let close_btn = Button::new(145, 0, 135, 25, "Close");

        pack.end();
        window.end();
        window.make_modal(true);

        Self {
            window,
            label_input,
            color_picker,
            fill_color_picker,
            line_style_selector,
            line_width_input,
            show_measurements,
            is_locked,
            apply_btn,
            close_btn,
            state,
        }
    }

    fn setup_callbacks(&self) {
        let state = self.state.clone();
        let color_picker = self.color_picker.clone();
        let line_width_input = self.line_width_input.clone();
        let window_clone = self.window.clone();

        let mut apply_btn = self.apply_btn.clone();
        apply_btn.set_callback(move |_| {
            if let Ok(mut state_ref) = state.try_borrow_mut() {
                if let Some(roi_tool) = state_ref.get_active_tool_mut() {
                    if let Ok(mut roi) = roi_tool.try_borrow_mut() {
                        // Update ROI properties
                        roi.color = color_picker.get_color();
                        roi.line_width = line_width_input.value()
                            .parse()
                            .unwrap_or(2);
                    }
                    
                    // Trigger redraw
                    if let Some(frame) = state_ref.get_frame() {
                        frame.borrow_mut().redraw();
                    }
                }
            }
        });

        let mut close_btn = self.close_btn.clone();
        let mut window = window_clone;
        close_btn.set_callback(move |_| {
            window.hide();
        });
    }

    pub fn show(&mut self) {
        self.setup_callbacks();
        self.window.show();
    }

    pub fn hide(&mut self) {
        self.window.hide();
    }

    pub fn is_visible(&self) -> bool {
        self.window.visible()
    }
}