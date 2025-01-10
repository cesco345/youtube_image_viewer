use image::DynamicImage;
use minifb::{Key, Window, WindowOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the original image
    let mut img = image::open("sol.png")?;
    let mut current_rotation = 0;
    
    // Create a window with dimensions matching the image
    let mut window = Window::new(
        "Image Rotation",
        img.width() as usize,
        img.height() as usize,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )?;

    // Set up buffer for window display
    let mut buffer: Vec<u32> = vec![0; img.width() as usize * img.height() as usize];

    println!("Controls:");
    println!("'R' - Rotate 90° clockwise");
    println!("'L' - Rotate 90° counterclockwise");
    println!("'S' - Save current rotation");
    println!("'Q' - Quit");

    // Update buffer with image data
    update_buffer(&img, &mut buffer);

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Q) {
        // Handle rotation controls
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            img = img.rotate90();
            current_rotation = (current_rotation + 90) % 360;
            println!("Rotated clockwise 90° - Current rotation: {}°", current_rotation);
            
            // Create new buffer with rotated dimensions
            buffer = vec![0; img.width() as usize * img.height() as usize];
            update_buffer(&img, &mut buffer);
        }

        if window.is_key_pressed(Key::L, minifb::KeyRepeat::No) {
            img = img.rotate270();
            current_rotation = (current_rotation - 90 + 360) % 360;
            println!("Rotated counterclockwise 90° - Current rotation: {}°", current_rotation);
            
            // Create new buffer with rotated dimensions
            buffer = vec![0; img.width() as usize * img.height() as usize];
            update_buffer(&img, &mut buffer);
        }

        if window.is_key_pressed(Key::S, minifb::KeyRepeat::No) {
            let filename = format!("rotated_{}_degrees.png", current_rotation);
            img.save(&filename)?;
            println!("Saved image as: {}", filename);
        }

        // Update window with current buffer
        window.update_with_buffer(&buffer, img.width() as usize, img.height() as usize)?;
    }

    Ok(())
}

// Helper function to update the buffer with image data
fn update_buffer(img: &DynamicImage, buffer: &mut Vec<u32>) {
    let rgba_img = img.to_rgba8();
    
    for (i, pixel) in rgba_img.pixels().enumerate() {
        let r = pixel[0] as u32;
        let g = pixel[1] as u32;
        let b = pixel[2] as u32;
        let a = pixel[3] as u32;
        
        // Convert RGBA to a single u32 value (ARGB format for minifb)
        buffer[i] = (a << 24) | (r << 16) | (g << 8) | b;
    }
}