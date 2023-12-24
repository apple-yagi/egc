use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

fn main() {
    let imgx = 200;
    let imgy = 200;

    let mut imgbuf = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(imgx, imgy);

    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = Rgba([255u8, 255u8, 255u8, 0]);
    }

    let font = Vec::from(include_bytes!("../fonts/NotoSansJP-Bold.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let scale = Scale { x: 41.5, y: 200.0 };

    let text = "完全に理解した";
    draw_text_mut(
        &mut imgbuf,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        0,
        0,
        scale,
        &font,
        text,
    );

    imgbuf.save("emoji.png").unwrap();
}
