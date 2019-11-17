use std::fs;
use std::env;
use serde::{Deserialize, Serialize};
use serde_json;
use image::{ImageBuffer};

#[derive(Serialize, Deserialize)]
struct Canvas {
    id: u8,
    pos: [u32;2],
	size: [u32;2],
}

fn generate_maps( canvas: &Canvas, filename: &str) {
    let w = canvas.size[0];
    let h = canvas.size[1];
    let mut imgbuf = ImageBuffer::new(w,h);
    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (255.0 * x as f32 / w as f32) as u8;
        let g = (255.0 * y as f32 / h as f32) as u8;
        *pixel = image::Rgb([r, g, 0]);
    }

    // write it out to a file
    imgbuf.save(filename).unwrap();
}

fn process( json: &String ) {
    let c: Canvas = serde_json::from_str(json).expect("Failed to parse json.");
    generate_maps( &c, "output.png" );
}

fn main() {
    let path = env::current_dir().unwrap().join("data").join("displays.json");
    match fs::read_to_string( &path ) {
        Ok(json) => process( &json ),
        Err(_) => println!("Failed to load file: {}", path.display()),
    }
}
