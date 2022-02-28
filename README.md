## Rusthering

A basic dithering CLI written in Rust

```console

USAGE:
    rusthering           <INPUT> <OUTPUT> [OPTIONS] 

ARGS:
    <SRC_PATH>           Path to the input image
    <DEST_PATH>          Path to the output image

OPTIONS:
    --grayscale          Produces a 8-bit grayscale output
    --dithering          Produces a 1-bit grayscale dithered output
    --random             Produces a random grayscale 1-bit dithered output
    --floyd              Produces a colored dithered output using the Floyd–Steinberg algorithm

```
