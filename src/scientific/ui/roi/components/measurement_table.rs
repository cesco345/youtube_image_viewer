// src/scientific/ui/roi/components/measurement_table.rs

use fltk::{
    table::Table,
    enums::{Align, Color, Font, FrameType},
    prelude::*,
    draw,
};
use std::{
    cell::RefCell,
    rc::Rc,
    cmp::Ordering,
};
use crate::scientific::types::ROIMeasurements;
use super::{ROW_HEIGHT, DEFAULT_FONT_SIZE, HEADER_FONT_SIZE};

#[derive(Debug, Clone, Copy)]
pub enum SortColumn {
    Id,
    Area,
    Perimeter,
    Circularity,
    MeanIntensity,
    None,
}

#[derive(Clone)]
pub struct MeasurementTable {
    table: Table,
    data: Rc<RefCell<Vec<ROIMeasurements>>>,
    sort_column: SortColumn,
    sort_ascending: bool,
    selected_row: Option<i32>,
}

impl MeasurementTable {
    const COLUMNS: &'static [&'static str] = &[
        "ID",
        "Area",
        "Perimeter",
        "Circularity",
        "Mean Int.",
        "Min Int.", 
        "Max Int.",
        "Shape",
        "Notes"
    ];

    pub fn new(x: i32, y: i32, w: i32, h: i32, data: Vec<ROIMeasurements>) -> Self {
        let mut table = Table::new(x, y, w, h, "");
        
        table.set_rows(data.len() as i32 + 1);
        table.set_cols(Self::COLUMNS.len() as i32);
        table.set_row_height_all(ROW_HEIGHT);
        table.set_row_header(true);
        table.set_col_header(true);
        table.set_col_resize(true);
        table.set_selection_color(Color::from_rgb(230, 240, 255));
        
        let col_widths = [50, 80, 80, 80, 80, 80, 80, 100, 150];
        for (i, width) in col_widths.iter().enumerate() {
            table.set_col_width(i as i32, *width);
        }

        let shared_data = Rc::new(RefCell::new(data));
        let mut mt = Self {
            table,
            data: shared_data,
            sort_column: SortColumn::None,
            sort_ascending: true,
            selected_row: None,
        };

        mt.setup_draw_callback();
        mt.setup_sort_callback();
        mt
    }

    fn setup_draw_callback(&mut self) {
        let data = self.data.clone();
        
        self.table.draw_cell(move |t, ctx, row, col, x, y, w, h| {
            match ctx {
                fltk::table::TableContext::StartPage => {
                    draw::set_font(Font::Helvetica, DEFAULT_FONT_SIZE);
                }
                fltk::table::TableContext::ColHeader => {
                    draw::push_clip(x, y, w, h);
                    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, Color::Light2);
                    draw::set_draw_color(Color::Black);
                    draw::set_font(Font::HelveticaBold, HEADER_FONT_SIZE);
                    draw::draw_text2(
                        Self::COLUMNS[col as usize],
                        x, y, w, h,
                        Align::Center,
                    );
                    draw::pop_clip();
                }
                fltk::table::TableContext::Cell => {
                    let data = data.borrow();
                    if row == 0 || row > data.len() as i32 { return; }
                    
                    let measurement = &data[(row - 1) as usize];
                    let text = Self::format_cell_value(measurement, col);
                    
                    draw::push_clip(x, y, w, h);
                    if row == t.callback_row() {
                        draw::set_draw_color(Color::from_rgb(230, 240, 255));
                        draw::draw_rect_fill(x, y, w, h, Color::from_rgb(230, 240, 255));
                    }
                    draw::set_draw_color(Color::Black);
                    draw::draw_text2(&text, x + 2, y, w - 4, h, Align::Left);
                    draw::pop_clip();
                }
                _ => (),
            }
        });
    }

    fn setup_sort_callback(&mut self) {
        let data = self.data.clone();
        let sort_column = Rc::new(RefCell::new(self.sort_column));
        let sort_ascending = Rc::new(RefCell::new(self.sort_ascending));
        
        self.table.set_callback(move |t| {
            if t.callback_context() == fltk::table::TableContext::ColHeader {
                let col = t.callback_col();
                let mut sort_col = sort_column.borrow_mut();
                *sort_col = match col {
                    0 => SortColumn::Id,
                    1 => SortColumn::Area,
                    2 => SortColumn::Perimeter,
                    3 => SortColumn::Circularity,
                    4 => SortColumn::MeanIntensity,
                    _ => SortColumn::None,
                };
                
                let mut asc = sort_ascending.borrow_mut();
                *asc = !*asc;
                
                let mut data = data.borrow_mut();
                MeasurementTable::sort_data(&mut data, *sort_col, *asc);
                t.redraw();
            }
        });
    }

    fn format_cell_value(measurement: &ROIMeasurements, col: i32) -> String {
        match col {
            0 => format!("{}", measurement.id),
            1 => format!("{:.2} {}", measurement.area, measurement.units),
            2 => format!("{:.2} {}", measurement.perimeter, measurement.units),
            3 => format!("{:.3}", measurement.circularity),
            4 => format!("{:.2}", measurement.mean_intensity),
            5 => format!("{:.2}", measurement.min_intensity),
            6 => format!("{:.2}", measurement.max_intensity),
            7 => measurement.shape_type.to_string(),
            8 => measurement.notes.clone().unwrap_or_default(),
            _ => String::new(),
        }
    }

    fn sort_data(data: &mut Vec<ROIMeasurements>, sort_column: SortColumn, ascending: bool) {
        data.sort_by(|a, b| {
            let ordering = match sort_column {
                SortColumn::Id => a.id.cmp(&b.id),
                SortColumn::Area => a.area.partial_cmp(&b.area).unwrap_or(Ordering::Equal),
                SortColumn::Perimeter => a.perimeter.partial_cmp(&b.perimeter)
                    .unwrap_or(Ordering::Equal),
                SortColumn::Circularity => a.circularity.partial_cmp(&b.circularity)
                    .unwrap_or(Ordering::Equal),
                SortColumn::MeanIntensity => a.mean_intensity.partial_cmp(&b.mean_intensity)
                    .unwrap_or(Ordering::Equal),
                SortColumn::None => Ordering::Equal,
            };
            if ascending {
                ordering
            } else {
                ordering.reverse()
            }
        });
    }

    pub fn update_data(&mut self, new_data: &[ROIMeasurements]) {
        let mut data = self.data.borrow_mut();
        data.clear();
        data.extend_from_slice(new_data);
        self.table.set_rows((data.len() + 1) as i32);
        Self::sort_data(&mut data, self.sort_column, self.sort_ascending);
        self.table.redraw();
    }

    pub fn get_selected_measurement(&self) -> Option<ROIMeasurements> {
        if let Some(row) = self.selected_row {
            let data = self.data.borrow();
            if row > 0 && row <= data.len() as i32 {
                return Some(data[(row - 1) as usize].clone());
            }
        }
        None
    }

    pub fn set_row_callback<F: Fn(i32) + 'static>(&mut self, callback: F) {
        let callback = Rc::new(callback);
        self.table.set_callback(move |t| {
            if t.callback_context() == fltk::table::TableContext::Cell {
                callback(t.callback_row());
            }
        });
    }

    pub fn clear(&mut self) {
        self.data.borrow_mut().clear();
        self.table.set_rows(1);
        self.selected_row = None;
        self.table.redraw();
    }

    pub fn get_row_count(&self) -> usize {
        self.data.borrow().len()
    }

    pub fn get_all_measurements(&self) -> Vec<ROIMeasurements> {
        self.data.borrow().clone()
    }

    pub fn export_csv(&self, path: &std::path::Path) -> std::io::Result<()> {
        use std::io::Write;
        let mut file = std::fs::File::create(path)?;
        
        writeln!(file, "{}", Self::COLUMNS.join(","))?;
        
        for measurement in self.data.borrow().iter() {
            writeln!(file, "{},{},{},{},{},{},{},{},{}",
                measurement.id,
                measurement.area,
                measurement.perimeter,
                measurement.circularity,
                measurement.mean_intensity,
                measurement.min_intensity,
                measurement.max_intensity,
                measurement.shape_type,
                measurement.notes.as_deref().unwrap_or("")
            )?;
        }
        
        Ok(())
    }
}