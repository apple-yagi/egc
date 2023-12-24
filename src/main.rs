use image::{ImageBuffer, Rgba};

fn main() {
    let imgx = 200;
    let imgy = 200;

    let mut imgbuf = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(imgx, imgy);

    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = Rgba::<u8>::from([255, 255, 255, 0]);
    }

    imgbuf.save("emoji.png").unwrap();
}
