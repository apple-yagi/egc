use std::env;

use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{point, Font, Rect, Scale};

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = &args[2];

    let imgx = 500;
    let imgy = 500;

    let mut imgbuf = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(imgx, imgy);

    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = Rgba([255u8, 255u8, 255u8, 0]);
    }

    let font = Vec::from(include_bytes!("../fonts/NotoSansJP-Bold.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let mut scale = Scale { x: 350.0, y: 350.0 };
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
                x: scale.x * 0.9,
                y: scale.y,
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
    let center_x = (imgx / 2) - (width / 2) as u32 - first.x as u32;
    let center_y = (imgy / 2) - (height / 2) as u32 - min_y as u32;

    draw_text_mut(
        &mut imgbuf,
        Rgba([255, 255, 255, 255]),
        center_x as i32,
        center_y as i32,
        scale,
        &font,
        text,
    );

    imgbuf.save(text.to_owned() + ".png").unwrap();
}
