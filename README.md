# vid2gif

`vid2gif` is a simple command-line tool written in Rust that converts videos into GIFs using `ffmpeg`. It supports all video formats that `ffmpeg` can read and can optionally produce high-quality optimized GIFs.

---

## Features

* Convert videos to GIF quickly.
* Automatically generates a GIF with default settings.
* Optional high-quality conversion using a palette for better colors.
* Custom output filename support.
* Works with any video format supported by `ffmpeg`.

---

## Requirements

* [Rust](https://www.rust-lang.org/tools/install) (for building from source)
* [ffmpeg](https://ffmpeg.org/download.html) installed and available in your system `PATH`.

---

## Installation

Clone the repository and build the project using Cargo:

```bash
git clone <your-repo-url>
cd vid2gif
cargo build --release
```

The binary will be located in `target/release/vid2gif`.

---

## Usage

Basic conversion (input video → `input.gif`):

```bash
vid2gif -i input.mp4
```

Custom output filename:

```bash
vid2gif -i input.mp4 -o output.gif
```

High-quality optimized GIF:

```bash
vid2gif -i input.mp4 -o fancy.gif --optimize
```

---

## Options

| Flag           | Description                                                        |
| -------------- | ------------------------------------------------------------------ |
| `-i, --input`  | Input video file (required)                                        |
| `-o, --output` | Output GIF file (optional; defaults to input filename with `.gif`) |
| `--optimize`   | Use palette-based high-quality conversion (optional)               |

---

## How It Works

* **Default Conversion:** Uses `ffmpeg` to convert the full video to GIF at 10 fps, scaled to 480px width.
* **Optimized Conversion (`--optimize`):** Generates a palette from the video, then converts using the palette for better color fidelity at 15 fps and 800px width.

---

## Notes

* The program relies on `ffmpeg` — ensure it is installed and accessible from your command line.
* GIFs are limited to 256 colors, so high-quality conversion uses a palette to preserve as much color detail as possible.
* Input file types are unrestricted as long as `ffmpeg` can decode them.

---

## License

This project is licensed under the **GPLv3 License**.
© 2025 LinuxFox

You can view the full license [here](https://www.gnu.org/licenses/gpl-3.0.en.html).

