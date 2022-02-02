use clap::{App, Arg, arg};
use rusthering::dithering;
use std::env;

fn main() {
    let matches = App::new("Rusthering")
      .version("0.1.0")
      .arg(
        Arg::new("ouput")
            .short('o')
      )
      .arg(
        Arg::new("dithered")
            .short('d')
      )
      .arg(
        Arg::new("grayscale")
            .short('g')
      )
      .arg(
        Arg::new("quantize")
            .short('q')
      )
      .arg(
        Arg::new("help")
            .short('h')
            .help("\n\
              See below the list of options available:\n\
              -o, --output           Path to the output file (optional, will default to your current directory with the following format: [image_name]_modified.[extension])\n\
              -d  --dithered         Produce a 4-bit dithered output\n\
              -g  --grayscale        Produce a 8-bit grayscale output\n\
              -q  --quantize         Produce a 1-bit quantized output\n\
              -h, --help             Print help information\n\
              -v, --version          Print version information\n\
            ")
      )
      .arg(
        arg!(-v --version "The CLI is currently on version 0.1.0"),
      )
      .get_matches();
    
    // match matches {
    //   m.is_present => println!("WORKS");
    //   _ => {}
    // }
    let args: Vec<String> = env::args().collect();
    let image_path = &args[1];
    let destination_path = &args[2];
    let destination_buffer = dithering(image_path);
    destination_buffer.save(destination_path).unwrap();
}
