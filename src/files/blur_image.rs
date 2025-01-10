use image::{GenericImageView, ImageBuffer, Rgb};

fn main() {
    let img = image::open("lira.jpg").unwrap();
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    
    let mut output = ImageBuffer::new(width, height);
    
    // Simple box blur
    for x in 1..width-1 {
        for y in 1..height-1 {
            let mut r_sum = 0u32;
            let mut g_sum = 0u32;
            let mut b_sum = 0u32;
            
            // 3x3 kernel
            for dx in [-1, 0, 1].iter() {
                for dy in [-1, 0, 1].iter() {
                    let pixel = rgb_img.get_pixel((x as i32 + dx) as u32, 
                                                (y as i32 + dy) as u32);
                    r_sum += pixel[0] as u32;
                    g_sum += pixel[1] as u32;
                    b_sum += pixel[2] as u32;
                }
            }
            
            output.put_pixel(x, y, Rgb([
                (r_sum / 9) as u8,
                (g_sum / 9) as u8,
                (b_sum / 9) as u8
            ]));
        }
    }
    
    output.save("blurred.png").unwrap();
}