// pixelate_tool.rs
use fltk::{
    dialog::{alert, choice2},
    frame::Frame,
    prelude::*,
    enums::{Color, Event},
    app,
    draw,
};
use std::{cell::RefCell, rc::Rc};
use crate::state::ImageState;
use crate::menu::edit::crop::crop_tool::CropSelection;
use super::ImageFilter;
use super::advanced::PixelateFilter;

pub fn start_interactive_pixelate(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    block_size: u32
) {
    let mut state_ref = state.borrow_mut();
    if state_ref.image.is_none() {
        alert(300, 300, "Please open an image first");
        return;
    }
    
    let original_image = state_ref.image.clone();
    
    // Initialize crop selection for pixelation area
    if let Some(img) = &original_image {
        let frame_ref = frame.borrow();
        state_ref.crop_selection = Some(CropSelection::new(
            img.data_w(),
            img.data_h(),
            frame_ref.w(),
            frame_ref.h()
        ));
    }
    drop(state_ref);

    let frame_clone = frame.clone();
    let state_clone = state.clone();
    let mut frame = frame.borrow_mut();

    let draw_callback = {
        let state_clone = state_clone.clone();
        let original_image = original_image.clone();
        move |f: &mut Frame| {
            if let Some(img) = &original_image {
                f.set_image(Some(img.clone()));

                let dimensions = state_clone
                    .try_borrow()
                    .ok()
                    .and_then(|state_ref| state_ref.crop_selection.as_ref().map(|s| (s.is_selecting, s.get_dimensions())));
                
                if let Some((true, (x, y, w, h))) = dimensions {
                    draw::set_draw_color(Color::Yellow);
                    draw::set_line_style(draw::LineStyle::Solid, 2);
                    draw::draw_rect(x, y, w, h);
                    
                    let grid_size = block_size.min(20) as i32;
                    draw::set_line_style(draw::LineStyle::Dot, 1);
                    
                    for i in (x..x + w).step_by(grid_size as usize) {
                        draw::draw_line(i, y, i, y + h);
                    }
                    for i in (y..y + h).step_by(grid_size as usize) {
                        draw::draw_line(x, i, x + w, i);
                    }
                }
            }
        }
    };

    frame.draw(draw_callback);

    let handle_callback = {
        let state_clone = state_clone.clone();
        let frame_clone = frame_clone.clone();
        let block_size = block_size;
        move |f: &mut Frame, ev: Event| -> bool {
            match ev {
                Event::Push => {
                    if let Ok(mut state) = state_clone.try_borrow_mut() {
                        if let Some(selection) = &mut state.crop_selection {
                            selection.reset();
                            selection.start_x = app::event_x();
                            selection.start_y = app::event_y();
                            selection.is_selecting = true;
                            f.redraw();
                        }
                    }
                    true
                },
                Event::Drag => {
                    if let Ok(mut state) = state_clone.try_borrow_mut() {
                        if let Some(selection) = &mut state.crop_selection {
                            selection.end_x = app::event_x();
                            selection.end_y = app::event_y();
                            f.redraw();
                        }
                    }
                    true
                },
                Event::Released => {
                    let should_apply = {
                        let mut dimensions = None;
                        if let Ok(mut state) = state_clone.try_borrow_mut() {
                            if let Some(selection) = &mut state.crop_selection {
                                selection.is_selecting = false;
                                selection.end_x = app::event_x();
                                selection.end_y = app::event_y();
                                dimensions = Some(selection.get_dimensions());
                            }
                        }
                        
                        if let Some((_, _, w, h)) = dimensions {
                            w > 5 && h > 5 && choice2(300, 300, "Apply pixelation to selected area?", "Yes", "No", "") == Some(0)
                        } else {
                            false
                        }
                    };

                    if should_apply {
                        if let Ok(mut state) = state_clone.try_borrow_mut() {
                            if let (Some(selection), Some(current_image)) = (state.crop_selection.as_ref(), &state.image) {
                                let filter = PixelateFilter::new(block_size)
                                    .with_selection(selection.clone())
                                    .with_feather(5)
                                    .with_intensity(1.0);
                                
                                if let Ok(Some(new_image)) = state.filter_state.apply_filter(current_image, &filter) {
                                    state.image = Some(new_image.clone());
                                    frame_clone.borrow_mut().set_image(state.image.clone());
                                }
                            }
                            state.crop_selection = None;
                        }
                    } else {
                        if let Ok(mut state) = state_clone.try_borrow_mut() {
                            state.crop_selection = None;
                        }
                    }
                    
                    f.redraw();
                    true
                },
                _ => false,
            }
        }
    };

    frame.handle(handle_callback);
}