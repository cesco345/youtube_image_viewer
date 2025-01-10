use image::{GenericImageView, ImageBuffer, Rgb, RgbImage};
use std::env;

fn adjust_colors(img: &RgbImage, red_adjust: i16, blue_adjust: i16) -> RgbImage {
    let mut output = ImageBuffer::new(img.width(), img.height());
    
    for (x, y, pixel) in img.enumerate_pixels() {
        let new_pixel = Rgb([
            (pixel[0] as i16 + red_adjust).clamp(0, 255) as u8, // Adjust red
            pixel[1],                                           // Keep green unchanged
            (pixel[2] as i16 + blue_adjust).clamp(0, 255) as u8, // Adjust blue
        ]);
        output.put_pixel(x, y, new_pixel);
    }
    output
}

fn convert_to_grayscale(img: &RgbImage) -> RgbImage {
    let mut output = ImageBuffer::new(img.width(), img.height());
    
    for (x, y, pixel) in img.enumerate_pixels() {
        let grayscale_value = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;
        output.put_pixel(x, y, Rgb([grayscale_value, grayscale_value, grayscale_value]));
    }
    output
}

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <input_image> <red_adjustment> <blue_adjustment>", args[0]);
        eprintln!("Add 'grayscale' as an optional fourth argument to convert the image to grayscale.");
        return;
    }
    
    let input_path = &args[1];
    let red_adjust: i16 = args[2].parse().unwrap_or(0);
    let blue_adjust: i16 = args[3].parse().unwrap_or(0);
    let grayscale = args.get(4).map(|s| s == "grayscale").unwrap_or(false);

    // Load the input image
    let img = match image::open(input_path) {
        Ok(img) => img.to_rgb8(),
        Err(e) => {
            eprintln!("Failed to open image {}: {}", input_path, e);
            return;
        }
    };

    // Process the image
    let output = if grayscale {
        convert_to_grayscale(&img)
    } else {
        adjust_colors(&img, red_adjust, blue_adjust)
    };

    // Save the output image
    let output_path = if grayscale {
        "output_grayscale.png"
    } else {
        "output_color_adjusted.png"
    };

    if let Err(e) = output.save(output_path) {
        eprintln!("Failed to save the image: {}", e);
        return;
    }

    println!(
        "Image processing complete. Modified image saved as '{}'.",
        output_path
    );
}
