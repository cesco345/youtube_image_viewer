use image;
use minifb::{Window, WindowOptions, Key};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Your original grayscale conversion
    let img = image::open("solid_circle.png").unwrap();
    let gray_img = img.to_luma8();
    gray_img.save("grayscale.png").unwrap();

    // Get image dimensions
    let (width, height) = gray_img.dimensions();
    let scale = 4;
    let window_width = width as usize * scale;
    let window_height = height as usize * scale;

    // Create window
    let mut window = Window::new(
        "Grayscale Image",
        window_width,
        window_height,
        WindowOptions {
            resize: true,
            scale: minifb::Scale::X1,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )?;

    // Create display buffer
    let mut buffer = vec![0u32; window_width * window_height];

    // Convert grayscale to buffer
    for (i, pixel) in gray_img.pixels().enumerate() {
        let x = (i as u32 % width) as usize * scale;
        let y = (i as u32 / width) as usize * scale;
        
        let gray_value = pixel[0] as u32;
        let pixel_color = (0xFF << 24) | (gray_value << 16) | (gray_value << 8) | gray_value;

        for dy in 0..scale {
            for dx in 0..scale {
                let buf_idx = (y + dy) * window_width + (x + dx);
                if buf_idx < buffer.len() {
                    buffer[buf_idx] = pixel_color;
                }
            }
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, window_width, window_height)?;
    }

    Ok(())
}