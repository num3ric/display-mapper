use std::fs;
use std::env;
use std::mem;
use serde::{Deserialize, Serialize};
use serde_json;
use image::{ImageBuffer};

#[derive(Serialize, Deserialize)]
enum Orientation {
    Landscape,
    Portrait,
    LandscapeFlipped,
    PortraitFlipped,
}

#[derive(Serialize, Deserialize)]
struct Canvas {
    id: u8,
    pos: [u32;2],
    size: [u32;2],
    orientation: Orientation,
}

fn get_gradient( x:u32, y:u32, w:u32, h:u32, orient:&Orientation ) -> [u8;3]
{
    let mut nx = x as f32 / w as f32;
    let mut ny = y as f32 / h as f32;
    match orient {
        Orientation::Landscape => { },
        Orientation::Portrait => {
            mem::swap(&mut nx, &mut ny);
            nx = 1.0 - nx;
        },
        Orientation::LandscapeFlipped => {
            ny = 1.0 - ny;
        },
        Orientation::PortraitFlipped => {
            mem::swap(&mut nx, &mut ny);
            ny = 1.0 - ny;
        },
    }

    [(255.0 * nx) as u8, (255.0 * ny) as u8, 0]
}

fn generate_maps( canvas: &Canvas, filename: &str) {
    let w = canvas.size[0];
    let h = canvas.size[1];
    let mut imgbuf = ImageBuffer::new(w,h);
    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let white : bool = x % 4 == 0 && y % 4 == 0;
        let black : bool = ( x + 2 ) % 4 == 0 && ( y + 2 ) % 4 == 0;
        if white {
            *pixel = image::Rgb([255,255,255]);
        }
        else if black {
            *pixel = image::Rgb([0, 0, 0]);
        }
        else {
            *pixel = image::Rgb(get_gradient(x,y,w,h, &canvas.orientation));
        }   
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
