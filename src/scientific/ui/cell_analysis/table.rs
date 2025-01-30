use fltk::{
    table::Table,
    draw,
    enums::{Color, Align, FrameType, Font},
    prelude::*,
};
use super::TABLE_ROW_HEIGHT;

pub fn create_data_table(x: i32, y: i32, w: i32, h: i32, data: Vec<Vec<String>>) -> Table {
    let mut table = Table::default()
        .with_size(w, h)
        .with_pos(x, y);
    
    table.set_rows(data.len() as i32);
    table.set_cols(data[0].len() as i32);
    table.set_row_height_all(TABLE_ROW_HEIGHT);
    table.set_row_header(true);
    table.set_col_header(true);
    table.set_col_resize(true);

    table.draw_cell(move |_, ctx, row, col, x, y, w, h| {
        match ctx {
            fltk::table::TableContext::StartPage => {
                draw::set_font(Font::Helvetica, 14);
            },
            fltk::table::TableContext::ColHeader => {
                draw_header_cell(&data[0][col as usize], x, y, w, h);
            },
            fltk::table::TableContext::Cell => {
                if row < data.len() as i32 && col < data[row as usize].len() as i32 {
                    draw_data_cell(&data[row as usize][col as usize], x, y, w, h);
                }
            },
            _ => {}
        }
    });

    table.end();
    table
}

fn draw_header_cell(text: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    
    // Draw background
    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, Color::Light2);
    
    // Draw text
    draw::set_draw_color(Color::Black);
    draw::set_font(Font::HelveticaBold, 14);
    draw::draw_text2(text, x, y, w, h, Align::Center);
    
    draw::pop_clip();
}

fn draw_data_cell(text: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    
    // Draw cell background
    draw::draw_box(FrameType::FlatBox, x, y, w, h, Color::White);
    
    // Draw cell border
    draw::set_draw_color(Color::from_rgb(240, 240, 240));
    draw::draw_rect(x, y, w, h);
    
    // Draw text
    draw::set_draw_color(Color::Black);
    draw::set_font(Font::Helvetica, 14);
    
    // Add some padding for the text
    let padding = 5;
    let text_x = x + padding;
    let text_w = w - (2 * padding);
    
    // If the cell contains a number, right-align it
    let alignment = if text.chars().next().map_or(false, |c| c.is_digit(10) || c == '-') {
        Align::Right
    } else {
        Align::Left
    };
    
    draw::draw_text2(text, text_x, y, text_w, h, alignment);
    
    draw::pop_clip();
}

// Optional: Add function to set custom column widths based on content
pub fn set_col_widths(table: &mut Table, data: &[Vec<String>]) {
    if data.is_empty() {
        return;
    }

    let num_cols = data[0].len();
    for col in 0..num_cols {
        // Find the maximum width needed for this column
        let max_width = data.iter()
            .map(|row| row.get(col).map_or(0, |text| text.len()))
            .max()
            .unwrap_or(0);

        // Convert character count to pixels (approximate)
        let width = (max_width as i32 * 10).max(60);
        table.set_col_width(col as i32, width);
    }
}

// Helper function to customize table appearance
pub fn customize_table(table: &mut Table) {
    table.set_color(Color::White);
    table.set_selection_color(Color::from_rgb(240, 240, 255));
    table.set_col_header_height(TABLE_ROW_HEIGHT);
    table.set_row_header_width(60);
}