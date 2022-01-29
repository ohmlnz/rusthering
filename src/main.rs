use image::open;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let current_file = &args[1];
    let destination_file = &args[2];

    grayscale(current_file, destination_file);
}

fn grayscale(current_file: &String, destination_file: &String) {
    let mut og_img = open(current_file).unwrap().into_rgb8();
    let mut dest_img = image::ImageBuffer::new(og_img.width(), og_img.height());

    for (x, y, og_pixel) in og_img.enumerate_pixels_mut() {
        let pixel = dest_img.get_pixel_mut(x, y);
        let image::Rgb(_data) = *pixel;
        let grayscale: u8 = (0.2167 * og_pixel.0[0] as f32) as u8
            + (0.7152 * og_pixel.0[1] as f32) as u8
            + (0.0722 * og_pixel.0[2] as f32) as u8;
        *pixel = image::Rgb([grayscale, grayscale, grayscale]);
    }

    dest_img.save(destination_file).unwrap();
}

fn quantize() {}
