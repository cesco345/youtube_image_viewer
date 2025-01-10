use std::io::prelude::*;
use std::io::{self, SeekFrom, Cursor};
use std::fs::File;

// Function to write bytes and print positions
fn write_ten_bytes_at_end<W: Write + Seek>(writer: &mut W) -> io::Result<()> {
    // Get current position before seeking
    let initial_pos = writer.stream_position()?;
    println!("Initial position: {}", initial_pos);

    // Seek 10 bytes from the end
    let new_pos = writer.seek(SeekFrom::End(-10))?;
    println!("After seeking -10 from end, new position: {}", new_pos);

    // Write bytes and track positions
    for i in 0..10 {
        writer.write(&[i])?;
        let current_pos = writer.stream_position()?;
        println!("Wrote byte {} at position {}", i, current_pos);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    println!("\n=== Testing with in-memory Cursor ===");
    // Create a cursor with 15 zeros
    let mut buff = Cursor::new(vec![0; 15]);
    println!("Created cursor with buffer size: {}", buff.get_ref().len());
    
    // Write bytes using our function
    write_ten_bytes_at_end(&mut buff)?;
    
    // Show the final buffer contents
    println!("\nFinal buffer contents: {:?}", buff.get_ref());
    println!("Last 10 bytes: {:?}", &buff.get_ref()[5..15]);

    println!("\n=== Testing with File ===");
    // Now try with a real file
    let mut file = File::create("foo.txt")?;
    
    // Write some initial data
    file.write_all(&[1; 15])?;
    println!("Created file with 15 bytes of initial data");
    
    // Seek back to start to write our test bytes
    file.seek(SeekFrom::Start(0))?;
    write_ten_bytes_at_end(&mut file)?;
    
    // Read and display file contents
    let contents = std::fs::read("foo.txt")?;
    println!("\nFile contents: {:?}", contents);
    println!("Last 10 bytes: {:?}", &contents[5..15]);

    Ok(())
}

#[test]
fn test_writes_bytes() {
    let mut buff = Cursor::new(vec![0; 15]);
    write_ten_bytes_at_end(&mut buff).unwrap();
    assert_eq!(&buff.get_ref()[5..15], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}