use image::io::Reader as ImageReader;
use std::fs;
use std::io::Cursor;

fn main() -> Result<(), image::ImageError> {
    // Ensure the directory for saving exists (in this case, saving in the current working directory)
    fs::create_dir_all("./").map_err(|e| {
        println!("Error creating directory: {}", e);
        e
    })?;

    // Method 1: Load from file
    let img = ImageReader::open("sol.png")?.decode()?;
    println!("First image loaded from a file successfully!");
    println!("Image dimensions: {}x{}", img.width(), img.height());
    
    // Method 2: Load from memory
    let image_bytes = include_bytes!("sol.png");
    let img2 = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()?
        .decode()?;
    println!("Second image loaded from memory successfully!");
    println!("Memory loaded image dimensions: {}x{}", img2.width(), img2.height());
    
    // Save both images to verify they loaded correctly
    img.save("./img_file.png")?;
    img2.save("./img_memory.png")?;
    println!("Both images saved successfully!");
    
    Ok(())
}