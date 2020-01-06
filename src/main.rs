use std::{fs, env, cmp, fmt};
use serde::{Deserialize, Serialize};
use serde_json;
use image::{GenericImage, ImageBuffer, Rgb, RgbImage};
use imageproc::{drawing};
use rusttype::{Font, Scale};

#[derive(Debug, Serialize, Deserialize)]
enum Orientation {
    Landscape,
    Portrait,
    LandscapeFlipped,
    PortraitFlipped,
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize)]
struct DisplayInfo {
    name: String,
    framerate: String,
    pos: [i32;2],
    size: [u32;2],
    orientation: Orientation,
    empty: bool
}

fn apply_gradient( img: &mut RgbImage )
{
    let w = img.width();
    let h = img.height();
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgb([(255.0 * x as f32 / w as f32) as u8, (255.0 * y as f32 / h as f32) as u8, 0]);
    }
}

fn apply_patterns( img: &mut RgbImage, display: &DisplayInfo )
{
    let white = Rgb([255u8, 255u8, 255u8]);
    let black = Rgb([0u8, 0u8, 0u8]);
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
    let font_data: &[u8] = include_bytes!("../data/consola.ttf");
    let font = Font::from_bytes(font_data).unwrap();
    drawing::draw_text_mut( img, white, 100, 100, Scale::uniform( 50.0 ), &font, &display.name );

    let ws = display.size[0].to_string();
    let hs = display.size[1].to_string();
    let orientation = display.orientation.to_string();
    let info_str = ["[", &ws, ", ", &hs, "], ", &display.framerate, ", ", &orientation ].concat();
    drawing::draw_text_mut( img, white, 100, 150, Scale::uniform( 25.0 ), &font, &info_str );
}

fn process( json: &String ) {
    let display_list: Vec<DisplayInfo> = serde_json::from_str(json).expect("Failed to parse json.");

    let mut min_x: i32 = std::i32::MAX;
    let mut min_y: i32 = std::i32::MAX;
    let mut max_x: i32 = std::i32::MIN;
    let mut max_y: i32 = std::i32::MIN;

    for display in display_list.iter() {
        let mut horizontal: bool = false;
        match display.orientation {
            Orientation::Landscape | Orientation::LandscapeFlipped => horizontal = true,
            _ => { }
        }
        let w = display.size[if horizontal { 0 } else { 1 }] as i32;
        let h = display.size[if horizontal { 1 } else { 0 }] as i32;
        min_x = cmp::min( min_x, display.pos[0] );
        min_y = cmp::min( min_y, display.pos[1] );
        max_x = cmp::max( max_x, display.pos[0] + w );
        max_y = cmp::max( max_y, display.pos[1] + h );
    }


    let mut canvas = ImageBuffer::new( (max_x - min_x) as u32, (max_y - min_y) as u32);
    for display in display_list.iter() {
        let mut displayimg = ImageBuffer::new( display.size[0], display.size[1] );
        match display.orientation {
            Orientation::Landscape => { },
            Orientation::Portrait => {
                displayimg = image::imageops::rotate90( &displayimg );
            },
            Orientation::LandscapeFlipped => {
                displayimg = image::imageops::rotate180( &displayimg );
            },
            Orientation::PortraitFlipped => {
                displayimg = image::imageops::rotate270( &displayimg );
            },
        }
        if ! display.empty {
            apply_gradient( &mut displayimg );
            apply_patterns( &mut displayimg, &display );
        }
        canvas.copy_from( &displayimg, ( display.pos[0] - min_x ) as u32, (display.pos[1] - min_y ) as u32 );
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
