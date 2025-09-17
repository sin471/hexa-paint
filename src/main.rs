use image;
use std::io::{self, Write};

fn get_coordinates(pixel_count: u32, width: u32) -> (u32, u32) {
    let block_index = (pixel_count % width) / 4 + 1;
    let x = block_index * 4 - (pixel_count % width % 4) - 1;
    let y = pixel_count / width;
    (x, y)
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <output-filename> <width> <height>", args[0]);
        std::process::exit(1);
    }
    let output_filename = &args[1];
    let width: u32 = args[2].parse().expect("Invalid width");
    let height: u32 = args[3].parse().expect("Invalid height");
    print!("Enter Hex values: ");
    io::stdout().flush().unwrap();
    let mut hexes = String::new();
    io::stdin().read_line(&mut hexes).expect("Failed to read line");
    let hexes = hexes.trim().to_string();
    
    let mut img = image::RgbImage::new(width, height);
    if (width * height) as usize != hexes.len() * 4 {
        eprintln!("Error: width * height must equal the number of hex values provided.");
        std::process::exit(1);
    }
    
    let mut pixel_count =0;
    for hex in hexes.chars() {
        let mut hex_num = hex.to_digit(16).expect("Invalid hex character") as u8;
        for _ in 0..=3 {
            let pixel = if hex_num & 0b0001 != 0 {
                image::Rgb([0, 0, 0])
            } else {
                image::Rgb([255, 255, 255])
            };
            let (x,y) = get_coordinates(pixel_count, width);
            img.put_pixel(x, y, pixel);
            hex_num >>= 1;
            pixel_count += 1;
        }
    }
    img.save(output_filename).expect("Failed to save image");
}
