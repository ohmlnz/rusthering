use image::{open, Rgb};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_path = &args[1];
    let destination_path = &args[2];

    let mut selected_image = open(image_path).unwrap().into_rgb8();
    let mut buffer = image::ImageBuffer::new(selected_image.width(), selected_image.height());
    let image_width = selected_image.width() as i32;
    let image_height = selected_image.height() as i32;

    for (x, y, og_pixel) in selected_image.enumerate_pixels_mut() {
        let pixel = buffer.get_pixel_mut(x, y);
        let op_pixel = grayscale(og_pixel);
        let qp_pixel = quantize(pixel);

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
                let new_pixel = buffer
                    .get_pixel_mut((x as i32 + x_offset) as u32, (y as i32 + y_offset) as u32);

                let pixel_values: [i32; 3] = [
                    (new_pixel.0[0] as i32 + ((errors[0] as f32) * error_bias) as i32),
                    (new_pixel.0[1] as i32 + ((errors[1] as f32) * error_bias) as i32),
                    (new_pixel.0[2] as i32 + ((errors[2] as f32) * error_bias) as i32),
                ];
                *new_pixel = image::Rgb([
                    pixel_values[0].clamp(0, 255) as u8,
                    pixel_values[1].clamp(0, 255) as u8,
                    pixel_values[2].clamp(0, 255) as u8,
                ]);
            }
        };

        update_pixel(1, 0, 7.0 / 16.0);
        update_pixel(-1, 1, 3.0 / 16.0);
        update_pixel(0, 1, 5.0 / 16.0);
        update_pixel(1, 1, 1.0 / 16.0);
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
