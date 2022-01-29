use image::{open, Rgb};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_path = &args[1];
    let destination_path = &args[2];

    let mut selected_image = open(image_path).unwrap().into_rgb8();
    let mut buffer = image::ImageBuffer::new(selected_image.width(), selected_image.height());

    for (x, y, og_pixel) in selected_image.enumerate_pixels_mut() {
        let pixel = buffer.get_pixel_mut(x, y);
        *pixel = grayscale(og_pixel);
        *pixel = quantize(pixel);
    }

    buffer.save(destination_path).unwrap();
}

fn grayscale(pixel: &mut Rgb<u8>) -> Rgb<u8> {
    let grayscale: u8 = (0.2167 * pixel.0[0] as f32) as u8
        + (0.7152 * pixel.0[1] as f32) as u8
        + (0.0722 * pixel.0[2] as f32) as u8;
    image::Rgb([grayscale, grayscale, grayscale])
}

fn quantize(pixel: &mut Rgb<u8>) -> Rgb<u8> {
    if pixel.0[0] < 128 {
        return image::Rgb([0 as u8, 0 as u8, 0 as u8]);
    } else {
        return image::Rgb([255 as u8, 255 as u8, 255 as u8]);
    }
}

fn dither() {}
