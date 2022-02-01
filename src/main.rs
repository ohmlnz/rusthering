use image::{open, ImageBuffer, Rgb};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_path = &args[1];
    let destination_path = &args[2];

    let mut selected_image = open(image_path).unwrap().into_rgb8();
    let mut buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(selected_image.width(), selected_image.height());
    let image_width = selected_image.width() as i32;
    let image_height = selected_image.height() as i32;

    for x in 0..image_width as u32 {
        for y in 0..image_height as u32 {
            let pixel = buffer.get_pixel_mut(x, y);
            let op_pixel = grayscale(selected_image.get_pixel(x, y));
            let qp_pixel = quantize(&op_pixel);

            let errors: [i32; 3] = [
                (op_pixel.0[0] as i32 - qp_pixel.0[0] as i32),
                (op_pixel.0[1] as i32 - qp_pixel.0[1] as i32),
                (op_pixel.0[2] as i32 - qp_pixel.0[2] as i32),
            ];

            *pixel = qp_pixel;

            let mut update_pixel = |x_offset: i32, y_offset: i32, error_bias: f32| {
                if !(x as i32 + x_offset <= 0
                    || y as i32 + y_offset <= 0
                    || x as i32 + x_offset >= image_width
                    || y as i32 + y_offset >= image_height)
                {
                    let new_pixel = selected_image
                        .get_pixel_mut((x as i32 + x_offset) as u32, (y as i32 + y_offset) as u32);

                    let mut k: [i32; 3] = [
                        new_pixel.0[0] as i32,
                        new_pixel.0[1] as i32,
                        new_pixel.0[2] as i32,
                    ];

                    k[0] += ((errors[0] as f32) * error_bias) as i32;
                    k[1] += ((errors[1] as f32) * error_bias) as i32;
                    k[2] += ((errors[2] as f32) * error_bias) as i32;

                    *new_pixel = Rgb([
                        k[0].clamp(0, 255) as u8,
                        k[1].clamp(0, 255) as u8,
                        k[2].clamp(0, 255) as u8,
                    ]);
                }
            };

            update_pixel(1, 0, 7.0 / 16.0);
            update_pixel(-1, 1, 3.0 / 16.0);
            update_pixel(0, 1, 5.0 / 16.0);
            update_pixel(1, 1, 1.0 / 16.0);
        }
    }

    buffer.save(destination_path).unwrap();
}

fn grayscale(pixel: &Rgb<u8>) -> Rgb<u8> {
    let grayscale: u8 = (0.2167 * pixel.0[0] as f32) as u8
        + (0.7152 * pixel.0[1] as f32) as u8
        + (0.0722 * pixel.0[2] as f32) as u8;
    Rgb([grayscale, grayscale, grayscale])
}

fn quantize(pixel: &Rgb<u8>) -> Rgb<u8> {
    if pixel.0[0] < 128 {
        return Rgb([0 as u8, 0 as u8, 0 as u8]);
    } else {
        return Rgb([255 as u8, 255 as u8, 255 as u8]);
    }
}
