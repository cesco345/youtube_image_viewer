use image::{GenericImageView, ImageBuffer, Rgb};

fn main() {
        
    let img = image::open("cock_fight.tiff").expect("Failed to open image.");

    let rgb_img = img.to_rgb8();

    let mut output = ImageBuffer::new(img.width(), img.height());

    // Process each pixel in the original image
    for (x, y, pixel) in rgb_img.enumerate_pixels() {
        // Modify the pixel values
        let new_pixel = Rgb([
            pixel[0].saturating_add(30), // Increase the red channel
            pixel[1],                    // Keep the green channel unchanged
            pixel[2].saturating_sub(30), // Decrease the blue channel
        ]);
        // Place the modified pixel in the output image
        output.put_pixel(x, y, new_pixel);
    }

    output
        .save("color_modified.png")
        .expect("Failed to save the modified image.");

    println!("Image processing complete. Modified image saved as 'color_modified.png'.");
}