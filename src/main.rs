use rusthering::Dithering;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let src_path = &args[1];
    let dest_path = &args[2];
    let option = &args[3];

    let mut dither = Dithering::new(src_path);
    let destination_buffer = dither.dithering(option);
    destination_buffer.save(dest_path).unwrap();
}