use std::{fs, env, mem, cmp};
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
struct Display {
    name: String,
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

fn generate_maps( imgbuf: &mut image::RgbImage, display: &Display )
{
    let offx = display.pos[0];
    let offy = display.pos[1];
    let w = display.size[0];
    let h = display.size[1];
    let i = 10;
    let hi = i/2;
    // Iterate over the coordinates and pixels of the image
    for x in offx..(offx+w) {
        for y in offy..(offy+h) {
            let pixel = imgbuf.get_pixel_mut(x, y);
            let white : bool = x % i == 0 && y % i == 0;
            let black : bool = ( x + hi ) % i == 0 && ( y + hi ) % i == 0;
            if white {
                *pixel = image::Rgb([255,255,255]);
            }
            else if black {
                *pixel = image::Rgb([0, 0, 0]);
            }
            else {
                *pixel = image::Rgb(get_gradient(x-offx,y-offy,w,h, &display.orientation));
            }
        }
    }
}

fn process( json: &String ) {
    let display_list: Vec<Display> = serde_json::from_str(json).expect("Failed to parse json.");

    let mut max_w: u32 = 0;
    let mut max_h: u32 = 0;
    for display in display_list.iter() {
        max_w = cmp::max( max_w, display.pos[0] + display.size[0] );
        max_h = cmp::max( max_h, display.pos[1] + display.size[1] );
    }
    let mut imgbuf = ImageBuffer::new(max_w, max_h);

    for display in display_list.iter() {
        generate_maps( &mut imgbuf, &display );
    }
   
    // write it out to a file
    imgbuf.save("output.png").unwrap();
}

fn main() {
    let path = env::current_dir().unwrap().join("data").join("displays.json");
    match fs::read_to_string( &path ) {
        Ok(json) => process( &json ),
        Err(_) => println!("Failed to load file: {}", path.display()),
    }
    println!("Done!");
}
