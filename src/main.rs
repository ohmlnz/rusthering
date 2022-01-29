use image::{open, ImageBuffer, Rgb};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_path = &args[1];
    let destination_path = &args[2];

    let selected_image = open(image_path).unwrap().into_rgb8();
    let grayscaled = grayscale(selected_image);
    let quantized = quantize(grayscaled);
    quantized.save(destination_path).unwrap();
}

fn grayscale(mut image: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut buffer = image::ImageBuffer::new(image.width(), image.height());

    for (x, y, og_pixel) in image.enumerate_pixels_mut() {
        let pixel = buffer.get_pixel_mut(x, y);
        let grayscale: u8 = (0.2167 * og_pixel.0[0] as f32) as u8
            + (0.7152 * og_pixel.0[1] as f32) as u8
            + (0.0722 * og_pixel.0[2] as f32) as u8;
        *pixel = image::Rgb([grayscale, grayscale, grayscale]);
    }

    buffer
}

fn quantize(mut image: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut buffer = image::ImageBuffer::new(image.width(), image.height());

    for (x, y, og_pixel) in image.enumerate_pixels_mut() {
        let pixel = buffer.get_pixel_mut(x, y);

        if og_pixel.0[0] < 128 {
            *pixel = image::Rgb([0 as u8, 0 as u8, 0 as u8]);
        } else {
            *pixel = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
        }
    }

    buffer
}
