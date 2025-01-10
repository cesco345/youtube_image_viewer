use image::DynamicImage;
use minifb::{Key, Window, WindowOptions};
use std::io::Write;

struct ImageState {
    image: DynamicImage,
    rotation: i32,
    scale: f32,
    position: (i32, i32),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = ImageState {
        image: image::open("optical.png")?,
        rotation: 0,
        scale: 1.0,
        position: (0, 0),
    };
    
    let mut window = Window::new(
        "Enhanced Image Viewer",
        800,
        600,
        WindowOptions {
            resize: true,
            scale: minifb::Scale::X1,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )?;

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut buffer: Vec<u32> = vec![0; 800 * 600];

    println!("\n=== Enhanced Image Viewer ===");
    println!("Controls:");
    println!("← → Arrow keys - Rotate image");
    println!("+ - Keys - Zoom in/out");
    println!("IJKL - Pan image (I=up, J=left, K=down, L=right)");
    println!("R - Reset view");
    println!("Space - Save current state");
    println!("Q - Quit");

    while window.is_open() && !window.is_key_down(Key::Q) {
        // Rotation
        if window.is_key_pressed(Key::Right, minifb::KeyRepeat::No) {
            state.rotation = (state.rotation + 90) % 360;
        }
        if window.is_key_pressed(Key::Left, minifb::KeyRepeat::No) {
            state.rotation = (state.rotation - 90 + 360) % 360;
        }

        // Panning with IJKL keys
        if window.is_key_down(Key::I) { state.position.1 -= 5; }
        if window.is_key_down(Key::K) { state.position.1 += 5; }
        if window.is_key_down(Key::J) { state.position.0 -= 5; }
        if window.is_key_down(Key::L) { state.position.0 += 5; }

        // Zooming
        if window.is_key_down(Key::Equal) {  // Plus key
            state.scale *= 1.1;
        }
        if window.is_key_down(Key::Minus) {
            state.scale /= 1.1;
        }

        // Reset
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            state.rotation = 0;
            state.scale = 1.0;
            state.position = (0, 0);
        }

        // Save (Space bar)
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            let filename = format!("image_rot{}_scale{}.png", 
                state.rotation,
                (state.scale * 100.0) as i32
            );
            state.image.save(&filename)?;
            println!("Saved as: {}", filename);
        }

        update_buffer(&state, &mut buffer, 800, 600);
        window.update_with_buffer(&buffer, 800, 600)?;

        // Display current state
        print!("\rRotation: {}° | Zoom: {:.1}x | Position: {:?}    ", 
            state.rotation, state.scale, state.position
        );
        std::io::stdout().flush()?;
    }

    Ok(())
}

fn update_buffer(state: &ImageState, buffer: &mut Vec<u32>, width: usize, height: usize) {
    buffer.fill(0);  // Clear with black background

    let scaled_width = (width as f32 * state.scale) as u32;
    let scaled_height = (height as f32 * state.scale) as u32;

    let mut processed_img = state.image.clone();
    
    // Apply transformations
    match state.rotation {
        90 => processed_img = processed_img.rotate90(),
        180 => processed_img = processed_img.rotate180(),
        270 => processed_img = processed_img.rotate270(),
        _ => {}
    }

    let resized = processed_img.resize(
        scaled_width,
        scaled_height,
        image::imageops::FilterType::Triangle
    );

    let rgba = resized.to_rgba8();
    
    // Apply position offset when copying to buffer
    for y in 0..height {
        for x in 0..width {
            let src_x = x as i32 - state.position.0;
            let src_y = y as i32 - state.position.1;
            
            if src_x >= 0 && src_x < rgba.width() as i32 && 
               src_y >= 0 && src_y < rgba.height() as i32 {
                let pixel = rgba.get_pixel(src_x as u32, src_y as u32);
                let idx = y * width + x;
                if idx < buffer.len() {
                    buffer[idx] = (pixel[3] as u32) << 24 | 
                                (pixel[0] as u32) << 16 | 
                                (pixel[1] as u32) << 8  | 
                                (pixel[2] as u32);
                }
            }
        }
    }
}