use rusthering::Dithering;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_path = &args[1];
    let destination_path = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
        + "/dithered.jpg";

    let mut dither = Dithering::new(image_path);
    let destination_buffer = dither.dithering(1);
    destination_buffer.save(destination_path).unwrap();
}
