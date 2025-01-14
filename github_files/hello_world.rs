fn main() {
    let image_path = "img_viewer/cli.jpeg";
    
    // Print current directory
    println!("Current directory: {:?}", std::env::current_dir().unwrap());
    
    // Check if file exists and is readable
    let path = Path::new(image_path);
    println!("Image path: {:?}", path.canonicalize().unwrap());
    
    // Try to read the file content first
    let img_result = image::open(path);
    match img_result {
        Ok(img) => {
            println!("Image loaded successfully!");
            println!("Dimensions: {}x{}", img.width(), img.height());
        },
        Err(e) => {
            println!("Failed to load image: {:?}", e);
        }
    }
}