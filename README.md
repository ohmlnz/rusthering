## Rusthering

# A basic dithering CLI written in Rust (following the Floyd–Steinberg algorithm)

```console

USAGE:
    rusthering             <INPUT> --output <OUTPUT>
ARGS:
    <INPUT>                Path to the input file       
OPTIONS:
    -o, --output           Path to the output file
    -d, --dithered         Produce a 4-bit dithered output
    -g, --grayscale        Produce a 8-bit grayscale output
    -q, --quantize         Produce a 1-bit quantized output
    -h, --help             Print help information
    -v, --version          Print version information

```