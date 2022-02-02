use rusthering::dithering;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_path = &args[1];
    let destination_path = &args[2];
    let destination_buffer = dithering(image_path);
    destination_buffer.save(destination_path).unwrap();
}
