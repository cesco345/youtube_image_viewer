use image::{ImageBuffer, Rgb};
use image::imageops;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create Image (1): Create and manipulate image directly in memory
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(300, 300);
    
    // Draw a red square in the middle
    for x in 100..200 {
        for y in 100..200 {
            img.put_pixel(x, y, Rgb([255_u8, 0_u8, 0_u8]));
        }
    }
    img.save("red_square.png")?;
    println!("Created and saved red_square.png");

    // Resized Image : Load from file and modify
    if let Ok(existing_img) = image::open("red_square.png") {
        // Resize the image to half its size
        let resized = imageops::resize(
            &existing_img, 
            existing_img.width() / 2, 
            existing_img.height() / 2,
            imageops::FilterType::Lanczos3
        );
        resized.save("red_square_small.png")?;
        println!("Created resized version: red_square_small.png");
    }

    // Load Image (2): Use file-to-memory loading
    // First read the file into memory
    let file_contents = std::fs::read("red_square.png")?;
    let img_from_memory = image::load_from_memory(&file_contents)?;
    
    // Create a blue version using memory-loaded image
    let mut blue_img = img_from_memory.to_rgb8();
    for pixel in blue_img.pixels_mut() {
        *pixel = Rgb([0_u8, 0_u8, 255_u8]); // Convert to blue
    }
    blue_img.save("blue_square.png")?;
    println!("Created blue version using memory loading");

    // Manipulate Image (3) : Using Cursor for streaming-like operations
    let cursor = Cursor::new(file_contents);  // Reuse the file contents we already loaded
    let img_from_cursor = image::load(cursor, image::ImageFormat::Png)?;
    
    // Create a composite effect using the cursor-loaded image
    let mut composite = img_from_cursor.to_rgb8();
    // Add a green diagonal line
    for i in 0..300 {
        if i < composite.width() && i < composite.height() {
            composite.put_pixel(i, i, Rgb([0_u8, 255_u8, 0_u8]));
        }
    }
    composite.save("composite_square.png")?;
    println!("Created composite version using cursor approach");

    // Print dimensions of all generated images
    println!("\nImage Dimensions:");
    println!("Original: {}x{}", img.width(), img.height());
    println!("Memory-loaded: {}x{}", img_from_memory.width(), img_from_memory.height());
    println!("Cursor-loaded: {}x{}", img_from_cursor.width(), img_from_cursor.height());

    Ok(())
}