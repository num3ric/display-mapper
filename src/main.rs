use std::{fs, env, mem, cmp};
use serde::{Deserialize, Serialize};
use serde_json;
use image::{GenericImage, SubImage, ImageBuffer, Rgb, RgbImage};
use imageproc::{drawing};
use rusttype::{Font, Scale};

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

fn apply_gradient( img: &mut RgbImage, display: &Display )
{
    let w = display.size[0];
    let h = display.size[1];
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgb(get_gradient(x,y,w,h, &display.orientation));
    }
}

fn apply_patterns<'a>( img: &mut RgbImage, text: &'a str )
{
    let white = Rgb([255u8, 255u8, 255u8]);
    let black = Rgb([0u8, 0u8, 0u8]);

    let scale = Scale::uniform( 100.0 );
    let font_data: &[u8] = include_bytes!("../data/consola.ttf");
    let font = Font::from_bytes(font_data).unwrap();

    for x in ( 0..img.width() ).step_by( 10 ) {
        for y in ( 0..img.height() ).step_by( 10 ) {
            let pixel = img.get_pixel_mut(x, y);
            *pixel = white;
        }
    }

    for x in ( 5..img.width() ).step_by( 10 ) {
        for y in ( 5..img.height() ).step_by( 10 ) {
            let pixel = img.get_pixel_mut(x, y);
            *pixel = black;
        }
    }

    drawing::draw_text_mut( img, white, 100, 100, scale, &font, text );
}

fn process( json: &String ) {
    let display_list: Vec<Display> = serde_json::from_str(json).expect("Failed to parse json.");

    let mut max_w: u32 = 0;
    let mut max_h: u32 = 0;
    for display in display_list.iter() {
        max_w = cmp::max( max_w, display.pos[0] + display.size[0] );
        max_h = cmp::max( max_h, display.pos[1] + display.size[1] );
    }
    let mut canvas = ImageBuffer::new(max_w, max_h);

    for display in display_list.iter() {
        let mut subimagebuf = ImageBuffer::new( display.size[0], display.size[1] );
        apply_gradient( &mut subimagebuf, &display );
        apply_patterns( &mut subimagebuf, &display.name );
        canvas.copy_from( &subimagebuf, display.pos[0], display.pos[1] );
    }
    // write it out to a file
    canvas.save("output.png").unwrap();
}

fn main() {
    let path = env::current_dir().unwrap().join("data").join("displays.json");
    match fs::read_to_string( &path ) {
        Ok(json) => process( &json ),
        Err(_) => println!("Failed to load file: {}", path.display()),
    }
    println!("Done!");
}
