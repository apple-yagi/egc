use std::process::exit;

use clap::Parser;
use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{point, Font, Rect, Scale};

#[derive(Parser)]
struct Args {
    /// 絵文字にする文字列
    first: String,

    /// 文字の色 （デフォルト: pink）
    /// 指定できる色 : pink, yellow, black, red, green, blue
    #[arg(short, long, default_value = "pink")]
    color: String,
}

fn main() {
    let args = Args::parse();
    let text = &args.first;
    let color_result = get_color(&args.color);
    let color = match color_result {
        Ok(color) => color,
        Err(err) => {
            println!("{}", err);
            exit(1)
        }
    };

    let imgx = 500;
    let imgy = 500;

    let mut imgbuf = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(imgx, imgy);

    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = Rgba([255u8, 255u8, 255u8, 0]);
    }

    let font = Vec::from(include_bytes!("../fonts/NotoSansJP-Bold.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let mut scale = Scale {
        x: 500.0,
        y: 1000.0,
    };
    for _ in 0..100 {
        let glyphs: Vec<Rect<i32>> = font
            .layout(text, scale, point(0.0, 0.0))
            .map(|g| g.pixel_bounding_box().unwrap())
            .collect();
        let first = glyphs.first().unwrap().min;
        let last = glyphs.last().unwrap().max;
        let width = last.x - first.x;
        if width > imgx as i32 {
            scale = Scale {
                x: scale.x - 5.0,
                y: scale.y,
            };
        } else {
            break;
        }
    }

    for _ in 0..100 {
        let glyphs: Vec<Rect<i32>> = font
            .layout(text, scale, point(0.0, 0.0))
            .map(|g| g.pixel_bounding_box().unwrap())
            .collect();
        let min_y = glyphs.iter().map(|g| g.min.y).min().unwrap();
        let max_y = glyphs.iter().map(|g| g.max.y).max().unwrap();
        let height = max_y - min_y;
        if height > imgy as i32 {
            scale = Scale {
                x: scale.x,
                y: scale.y - 5.0,
            };
        } else {
            break;
        }
    }

    let point = point(0.0, font.v_metrics(scale).ascent);

    let glyphs: Vec<Rect<i32>> = font
        .layout(text, scale, point)
        .map(|g| g.pixel_bounding_box().unwrap())
        .collect();

    let first = glyphs.first().unwrap().min;
    let last = glyphs.last().unwrap().max;
    let min_y = glyphs.iter().map(|g| g.min.y).min().unwrap();
    let max_y = glyphs.iter().map(|g| g.max.y).max().unwrap();
    let height = max_y - min_y;
    let width = last.x - first.x;
    let center_x = (imgx / 2) as i32 - (width / 2) as i32 - first.x as i32;
    let center_y = (imgy / 2) as i32 - (height / 2) as i32 - min_y as i32;

    draw_text_mut(&mut imgbuf, color, center_x, center_y, scale, &font, text);

    imgbuf.save(text.to_owned() + ".png").unwrap();
    exit(0)
}

fn get_color(color: &str) -> Result<Rgba<u8>, &str> {
    match color {
        "pink" => Ok(Rgba([255u8, 0u8, 255u8, 255u8])),
        "yellow" => Ok(Rgba([255u8, 255u8, 0u8, 255u8])),
        "black" => Ok(Rgba([0u8, 0u8, 0u8, 255u8])),
        "red" => Ok(Rgba([255u8, 0u8, 0u8, 255u8])),
        "green" => Ok(Rgba([0u8, 255u8, 0u8, 255u8])),
        "blue" => Ok(Rgba([0u8, 0u8, 255u8, 255u8])),
        _ => Err("指定できる色 : pink, yellow, black, red, green, blue, orange"),
    }
}
