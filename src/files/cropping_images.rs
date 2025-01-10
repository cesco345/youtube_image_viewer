use image::{DynamicImage, GenericImageView};

fn main() {
    let mut img = image::open("2_dollar.jpg").expect("Failed to open image.");
    let (img_width, img_height) = img.dimensions();
    println!("Image dimensions: {}x{}", img_width, img_height);

    // Define cropping dimensions
    let crop_width = 200;
    let crop_height = 200;

    // Calculate top-left corner for center crop
    let x = (img_width / 2).saturating_sub(crop_width / 2);
    let y = (img_height / 2).saturating_sub(crop_height / 2);

    // Crop and save the image
    let cropped_img: DynamicImage = img.crop(x, y, crop_width, crop_height);
    cropped_img
        .save("cropped_dollar.png")
        .expect("Failed to save cropped image.");
    println!("Image cropped and saved as 'cropped_dollar.png'.");
}