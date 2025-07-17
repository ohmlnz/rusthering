# Rusthering

```
  ____            _   _               _             
 |  _ \ _   _ ___| |_| |__   ___ _ __(_)_ __   __ _ 
 | |_) | | | / __| __| '_ \ / _ \ '__| | '_ \ / _` |
 |  _ <| |_| \__ \ |_| | | |  __/ |  | | | | | (_| |
 |_| \_\\__,_|___/\__|_| |_|\___|_|  |_|_| |_|\__, |
                                              |___/ 
   
```

A fast, simple CLI tool for applying various dithering and grayscale effects to images, written in Rust.

---

## Features
- **8-bit grayscale conversion**
- **1-bit threshold dithering** (classic black & white)
- **Randomized dithering**
- **Floyd–Steinberg color dithering**
- Fast and easy to use from the command line

---

## Usage

```console
USAGE:
    rusthering <SRC_PATH> <DEST_PATH> [OPTIONS]

ARGS:
    <SRC_PATH>    Path to the input image
    <DEST_PATH>   Path to the output image

OPTIONS:
    --grayscale   Produces an 8-bit grayscale output
    --dithering   Produces a 1-bit (black & white) dithered output
    --random      Produces a randomized 1-bit dithered output
    --floyd       Produces a color dithered output using the Floyd–Steinberg algorithm
```

### Example

```console
rusthering input.png output.png --floyd
```

---

## About
- See [Floyd–Steinberg dithering](https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering) for more on the algorithm.
- Built with ❤️ in Rust.
