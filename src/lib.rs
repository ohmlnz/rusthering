use image::{open, ImageBuffer, Rgb};
use rand::Rng;
pub struct Dithering {
    selected_image: ImageBuffer<Rgb<u8>, Vec<u8>>,
    destination_buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
    image_width: i32,
    image_height: i32,
}

impl Dithering {
    pub fn new(image_path: &str) -> Dithering {
        let selected_image = open(image_path).unwrap().into_rgb8();
        let destination_buffer = ImageBuffer::new(selected_image.width(), selected_image.height());
        let image_width = selected_image.width() as i32;
        let image_height = selected_image.height() as i32;

        Dithering {
            selected_image,
            destination_buffer,
            image_width,
            image_height,
        }
    }

    pub fn floyd(&mut self) -> &ImageBuffer<Rgb<u8>, Vec<u8>> {
        let multi_level_quantize = |pixel: &Rgb<u8>, levels: f32| {
            let channel_red =
                (((pixel.0[0] as f32 / 255.0) * levels).round() / levels * 255.0).clamp(0.0, 255.0);
            let channel_blue =
                (((pixel.0[1] as f32 / 255.0) * levels).round() / levels * 255.0).clamp(0.0, 255.0);
            let channel_green =
                (((pixel.0[2] as f32 / 255.0) * levels).round() / levels * 255.0).clamp(0.0, 255.0);
            Rgb([channel_red as u8, channel_blue as u8, channel_green as u8])
        };

        for x in 0..self.image_width as u32 {
            for y in 0..self.image_height as u32 {
                let destination_pixel = self.destination_buffer.get_pixel_mut(x, y);
                let original_pixel = self.selected_image.get_pixel(x, y);

                let quantized_pixel = multi_level_quantize(&original_pixel, 4.0);
                let quantization_errors: [i32; 3] = [
                    (original_pixel.0[0] as i32 - quantized_pixel.0[0] as i32),
                    (original_pixel.0[1] as i32 - quantized_pixel.0[1] as i32),
                    (original_pixel.0[2] as i32 - quantized_pixel.0[2] as i32),
                ];

                *destination_pixel = quantized_pixel;

                let mut update_neighboring_pixel =
                    |x_offset: i32, y_offset: i32, error_bias: f32| {
                        if !(x as i32 + x_offset <= 0
                            || y as i32 + y_offset <= 0
                            || x as i32 + x_offset >= self.image_width
                            || y as i32 + y_offset >= self.image_height)
                        {
                            let original_offset_pixel = self.selected_image.get_pixel_mut(
                                (x as i32 + x_offset) as u32,
                                (y as i32 + y_offset) as u32,
                            );

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

        &self.destination_buffer
    }

    pub fn linear_grayscale(&mut self) -> &ImageBuffer<Rgb<u8>, Vec<u8>> {
        let grayscale = |original_pixel: &Rgb<u8>| {
            let grayscale: u8 = (0.2167 * original_pixel.0[0] as f32) as u8
                + (0.7152 * original_pixel.0[1] as f32) as u8
                + (0.0722 * original_pixel.0[2] as f32) as u8;
            Rgb([grayscale, grayscale, grayscale])
        };

        for x in 0..self.image_width as u32 {
            for y in 0..self.image_height as u32 {
                let destination_pixel = self.destination_buffer.get_pixel_mut(x, y);
                let original_pixel = self.selected_image.get_pixel(x, y);

                let quantized_pixel = grayscale(&original_pixel);

                *destination_pixel = quantized_pixel;
            }
        }
        &self.destination_buffer
    }

    pub fn threshold(&mut self, threshold: u8) -> &ImageBuffer<Rgb<u8>, Vec<u8>> {
        let quantize = |original_pixel: &Rgb<u8>| {
            let threshold = if threshold == 0 {
                rand::thread_rng().gen_range(1..255)
            } else {
                127
            };

            if original_pixel.0[0] < threshold {
                return Rgb([0 as u8, 0 as u8, 0 as u8]);
            } else {
                return Rgb([255 as u8, 255 as u8, 255 as u8]);
            }
        };

        for x in 0..self.image_width as u32 {
            for y in 0..self.image_height as u32 {
                let destination_pixel = self.destination_buffer.get_pixel_mut(x, y);
                let original_pixel = self.selected_image.get_pixel(x, y);

                let quantized_pixel = quantize(&original_pixel);

                *destination_pixel = quantized_pixel;
            }
        }
        &self.destination_buffer
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn get_quantized_black_pixel() {
//         let black_pixel = threshold(&Rgb([100, 0, 0]));
//         assert_eq!(black_pixel, Rgb([0, 0, 0]));
//     }

//     #[test]
//     fn get_quantized_white_pixel() {
//         let white_pixel = threshold(&Rgb([128, 0, 0]));
//         assert_eq!(white_pixel, Rgb([255, 255, 255]));
//     }

//     #[test]
//     fn get_grayscale_pixel() {
//         let grayscale_pixel = linear_grayscale(&Rgb([140, 34, 45]));
//         assert_eq!(grayscale_pixel.0[0], grayscale_pixel.0[1]);
//         assert_eq!(grayscale_pixel.0[1], grayscale_pixel.0[2]);
//         assert_eq!(grayscale_pixel.0[0], grayscale_pixel.0[2]);
//     }
// }
