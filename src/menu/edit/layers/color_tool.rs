use fltk::{
    dialog::{alert, choice2},
    frame::Frame,
    prelude::*,
    enums::{Color, Event, Key},
    app,
    draw,
};
use std::{cell::RefCell, rc::Rc};
use crate::state::ImageState;
use crate::menu::edit::crop::crop_tool::CropSelection;
use crate::utils::{scale_image_dimensions, MENU_HEIGHT};
use super::color_filter::ColorFilter;

pub fn start_interactive_color(
    frame: &Rc<RefCell<Frame>>, 
    state: &Rc<RefCell<ImageState>>,
    color: (u8, u8, u8)
) {
    println!("Starting handle_create_layer with color RGB({}, {}, {})", color.0, color.1, color.2);
    
    let mut state_ref = state.borrow_mut();
    if state_ref.image.is_none() {
        alert(300, 300, "Please open an image first");
        return;
    }
    
    let original_image = state_ref.image.clone();
    
    if let Some(img) = &original_image {
        let frame_ref = frame.borrow();
        let (displayed_w, displayed_h) = scale_image_dimensions(
            img.data_w(),
            img.data_h(),
            frame_ref.w(),
            frame_ref.h() - MENU_HEIGHT,
            1.0
        );
        
        state_ref.crop_selection = Some(CropSelection::new(
            displayed_w,
            displayed_h,
            displayed_w,
            displayed_h
        ));
    }
    drop(state_ref);

    let frame_clone = frame.clone();
    let state_clone = state.clone();
    let mut frame = frame.borrow_mut();
    let continue_drawing = Rc::new(RefCell::new(true));

    let draw_callback = {
        let state_clone = state_clone.clone();
        let original_image = original_image.clone();
        let frame_clone = frame_clone.clone();
        move |f: &mut Frame| {
            if let Some(img) = &original_image {
                if let Ok(state_ref) = state_clone.try_borrow() {
                    if let Some(composite) = state_ref.layer_state.get_composite_image() {
                        if state_ref.layer_state.is_preview_active() {
                            f.set_image(Some(composite.clone()));
                        } else {
                            f.set_image(Some(img.clone()));
                        }
                    } else {
                        f.set_image(Some(img.clone()));
                    }
                }

                let dimensions = state_clone
                    .try_borrow()
                    .ok()
                    .and_then(|state_ref| state_ref.crop_selection.as_ref().map(|s| (s.is_selecting, s.get_dimensions())));
                
                if let Some((true, (x, y, w, h))) = dimensions {
                    draw::set_draw_color(Color::Yellow);
                    draw::set_line_style(draw::LineStyle::Solid, 2);
                    draw::draw_rect(x, y, w, h);
                    
                    let grid_size = 20;
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
        let color = color;
        let continue_drawing = continue_drawing.clone();
        
        move |f: &mut Frame, ev: Event| -> bool {
            if !*continue_drawing.borrow() {
                return false;
            }

            match ev {
                Event::KeyDown => {
                    if app::event_key() == Key::Escape {
                        *continue_drawing.borrow_mut() = false;
                        if let Ok(mut state) = state_clone.try_borrow_mut() {
                            state.crop_selection = None;
                        }
                        f.redraw();
                        super::dialog::show_new_layer_dialog(&frame_clone, &state_clone);
                        return true;
                    }
                    false
                },
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
                    let (selection_data, current_image) = {
                        let state = state_clone.try_borrow().ok();
                        (
                            state.as_ref().and_then(|s| s.crop_selection.as_ref().map(|sel| sel.clone())),
                            state.and_then(|s| s.image.clone())
                        )
                    };
                
                    if let (Some(selection), Some(current_image)) = (selection_data, current_image) {
                        let (sx, sy, sw, sh) = selection.get_dimensions();
                        let frame_w = frame_clone.borrow().w();
                        let frame_h = frame_clone.borrow().h();
                        let image_dims = (current_image.data_w(), current_image.data_h());

                        let (displayed_w, displayed_h) = scale_image_dimensions(
                            image_dims.0,
                            image_dims.1,
                            frame_w,
                            frame_h - MENU_HEIGHT,
                            1.0
                        );

                        let offset_x = (frame_w - displayed_w) / 2;
                        let offset_y = MENU_HEIGHT + (frame_h - MENU_HEIGHT - displayed_h) / 2;

                        let adjusted_x = sx - offset_x;
                        let adjusted_y = sy - offset_y;

                        let scale_x = image_dims.0 as f64 / displayed_w as f64;
                        let scale_y = image_dims.1 as f64 / displayed_h as f64;

                        let img_x = (adjusted_x as f64 * scale_x).round() as i32;
                        let img_y = (adjusted_y as f64 * scale_y).round() as i32;
                        let img_w = (sw as f64 * scale_x).round() as i32;
                        let img_h = (sh as f64 * scale_y).round() as i32;

                        let img_x = img_x.max(0).min(image_dims.0 as i32 - 1);
                        let img_y = img_y.max(0).min(image_dims.1 as i32 - 1);
                        let img_w = img_w.min(image_dims.0 as i32 - img_x);
                        let img_h = img_h.min(image_dims.1 as i32 - img_y);

                        let should_apply = img_w > 5 && img_h > 5 && 
                            choice2(300, 300, "Apply color to selected area?", "Yes", "No", "") == Some(0);
                
                        if should_apply {
                            if let Ok(mut state) = state_clone.try_borrow_mut() {
                                if state.layer_state.get_layer_count() == 0 {
                                    state.layer_state.set_original_image(current_image.clone());
                                }

                                let mut image_selection = CropSelection::new(
                                    image_dims.0,
                                    image_dims.1,
                                    image_dims.0,
                                    image_dims.1
                                );
                                
                                image_selection.start_x = img_x;
                                image_selection.start_y = img_y;
                                image_selection.end_x = img_x + img_w;
                                image_selection.end_y = img_y + img_h;
                                image_selection.is_selecting = false;
                        
                                let filter = ColorFilter::new(color)
                                    .with_selection(image_selection.clone())
                                    .with_feather(5)
                                    .with_intensity(0.8);
                                
                                if let Ok(Some(_)) = state.filter_state.apply_filter(&current_image, &filter) {
                                    let layer_index = state.layer_state.add_layer(color, image_selection);
                                    if let Some(layer) = state.layer_state.get_layer_mut(layer_index) {
                                        layer.opacity = 0.8;
                                    }
                                    
                                    // here it keeps the selection active for next area
                                    state.crop_selection = Some(CropSelection::new(
                                        displayed_w,
                                        displayed_h,
                                        displayed_w,
                                        displayed_h
                                    ));
                                }
                            }
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