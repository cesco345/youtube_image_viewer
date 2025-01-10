use image::{DynamicImage, GenericImageView};
use std::error::Error;
use image::imageops::FilterType;


fn main() -> Result<(), Box<dyn Error>> {
    // Load the input image
    let img = image::open("2_dollar.jpg").expect("Failed to open input image.");
    let (img_width, img_height) = img.dimensions();
    println!("Original Image Dimensions: {}x{}", img_width, img_height);

    // 1. Resize Exact (Does not preserve aspect ratio)
    resize_exact(&img)?;

    // 2. Create Thumbnail (Preserves aspect ratio)
    create_thumbnail(&img)?;

    // 3. Create Thumbnail Exact (Does not preserve aspect ratio)
    create_thumbnail_exact(&img)?;

    // 4. Resize to Fill (Preserves aspect ratio, then crops)
    resize_to_fill(&img)?;

    Ok(())
}

/// 1. Resize Exact
fn resize_exact(img: &DynamicImage) -> Result<(), Box<dyn Error>> {
    let new_width = 800;
    let new_height = 600;
    let resized = img.resize_exact(new_width, new_height, FilterType::Nearest);
    resized.save("resized_exact.png")?;
    println!(
        "Resized Exact: {}x{} -> {}x{}",
        img.width(),
        img.height(),
        new_width,
        new_height
    );
    Ok(())
}

/// 2. Create Thumbnail
fn create_thumbnail(img: &DynamicImage) -> Result<(), Box<dyn Error>> {
    let max_width = 200;
    let max_height = 200;
    let thumbnail = img.thumbnail(max_width, max_height);
    thumbnail.save("thumbnail.png")?;
    println!(
        "Thumbnail (Preserved Aspect Ratio): {}x{} -> {}x{}",
        img.width(),
        img.height(),
        thumbnail.width(),
        thumbnail.height()
    );
    Ok(())
}

/// 3. Create Thumbnail Exact
fn create_thumbnail_exact(img: &DynamicImage) -> Result<(), Box<dyn Error>> {
    let thumb_width = 200;
    let thumb_height = 200;
    let thumbnail_exact = img.thumbnail_exact(thumb_width, thumb_height);
    thumbnail_exact.save("thumbnail_exact.png")?;
    println!(
        "Thumbnail Exact (No Aspect Ratio): {}x{} -> {}x{}",
        img.width(),
        img.height(),
        thumb_width,
        thumb_height
    );
    Ok(())
}

/// 4. Resize to Fill
fn resize_to_fill(img: &DynamicImage) -> Result<(), Box<dyn Error>> {
    let fill_width = 300;
    let fill_height = 300;
    let resized_fill = img.resize_to_fill(fill_width, fill_height, FilterType::Lanczos3);
    resized_fill.save("resize_to_fill.png")?;
    println!(
        "Resized to Fill (Preserved Aspect Ratio + Cropped): {}x{} -> {}x{}",
        img.width(),
        img.height(),
        fill_width,
        fill_height
    );
    Ok(())
}