use image::{open, ImageBuffer, Rgb};

pub fn dithering(image_path: &str) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut selected_image = open(image_path).unwrap().into_rgb8();
    let mut destination_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(selected_image.width(), selected_image.height());
    let image_width = selected_image.width() as i32;
    let image_height = selected_image.height() as i32;

    for x in 0..image_width as u32 {
        for y in 0..image_height as u32 {
            let destination_pixel = destination_buffer.get_pixel_mut(x, y);
            let original_pixel = selected_image.get_pixel(x, y);
            let quantized_pixel = multi_level_quantization(&original_pixel);

            let quantization_errors: [i32; 3] = [
                (original_pixel.0[0] as i32 - quantized_pixel.0[0] as i32),
                (original_pixel.0[1] as i32 - quantized_pixel.0[1] as i32),
                (original_pixel.0[2] as i32 - quantized_pixel.0[2] as i32),
            ];

            *destination_pixel = quantized_pixel;

            let mut update_neighboring_pixel = |x_offset: i32, y_offset: i32, error_bias: f32| {
                if !(x as i32 + x_offset <= 0
                    || y as i32 + y_offset <= 0
                    || x as i32 + x_offset >= image_width
                    || y as i32 + y_offset >= image_height)
                {
                    let original_offset_pixel = selected_image
                        .get_pixel_mut((x as i32 + x_offset) as u32, (y as i32 + y_offset) as u32);

                    let channel_red = original_offset_pixel.0[0] as i32
                        + ((quantization_errors[0] as f32) * error_bias) as i32;
                    let channel_blue = original_offset_pixel.0[1] as i32
                        + ((quantization_errors[1] as f32) * error_bias) as i32;
                    let channel_green = original_offset_pixel.0[2] as i32
                        + ((quantization_errors[2] as f32) * error_bias) as i32;

                    *original_offset_pixel = Rgb([
                        channel_red.clamp(0, 255) as u8,
                        channel_blue.clamp(0, 255) as u8,
                        channel_green.clamp(0, 255) as u8,
                    ]);
                }
            };

            update_neighboring_pixel(1, 0, 7.0 / 16.0);
            update_neighboring_pixel(-1, 1, 3.0 / 16.0);
            update_neighboring_pixel(0, 1, 5.0 / 16.0);
            update_neighboring_pixel(1, 1, 1.0 / 16.0);
        }
    }

    destination_buffer
}

fn _grayscale(pixel: &Rgb<u8>) -> Rgb<u8> {
    let grayscale: u8 = (0.2167 * pixel.0[0] as f32) as u8
        + (0.7152 * pixel.0[1] as f32) as u8
        + (0.0722 * pixel.0[2] as f32) as u8;
    Rgb([grayscale, grayscale, grayscale])
}

fn multi_level_quantization(pixel: &Rgb<u8>) -> Rgb<u8> {
    let levels = 1.0;
    let channel_red =
        (((pixel.0[0] as f32 / 255.0) * levels).round() / levels * 255.0).clamp(0.0, 255.0);
    let channel_blue =
        (((pixel.0[1] as f32 / 255.0) * levels).round() / levels * 255.0).clamp(0.0, 255.0);
    let channel_green =
        (((pixel.0[2] as f32 / 255.0) * levels).round() / levels * 255.0).clamp(0.0, 255.0);
    Rgb([channel_red as u8, channel_blue as u8, channel_green as u8])
}

fn _single_bit_quantization(pixel: &Rgb<u8>) -> Rgb<u8> {
    if pixel.0[0] < 127 {
        return Rgb([0 as u8, 0 as u8, 0 as u8]);
    } else {
        return Rgb([255 as u8, 255 as u8, 255 as u8]);
    }
}
