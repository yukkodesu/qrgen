# qrgen

A fast command-line tool for generating QR codes. Render in the terminal or save as image files.

## Install

```bash
cargo install --path .
```

## Usage

### Basic: render QR code in terminal

```bash
qrgen "hello world"
```

### Save as PNG with custom output path

```bash
qrgen "https://example.com" -o ./dist/site.png
```

### Save as WebP/AVIF/JPEG/SVG with `--format`

```bash
qrgen "https://example.com" -o ./dist/site.webp --format webp
qrgen "https://example.com" -o ./dist/site.avif --format avif
qrgen "https://example.com" -o ./dist/site.jpeg --format jpeg
qrgen "https://example.com" -o ./dist/site.svg --format svg
```

### Custom image size

```bash
qrgen "WIFI:T:WPA;S:MyWiFi;P:12345678;;" --size 512
```

### Options

```
Usage: qrgen [OPTIONS] <CONTENT>

Arguments:
  <CONTENT>  QR content text

Options:
  -o, --output <FILE>    Output image file path. If omitted, render in terminal.
      --format <FORMAT>  Output format for file export (requires --output) [possible values: avif, png, jpeg, webp, svg]
  -s, --size <PIXELS>    Output image size in pixels [default: 256]
  -v, --version          Print version information
  -h, --help             Print help (see more with '--help')
```

## Development

```bash
# Run tests
cargo test

# Build release binary
cargo build --release
```

## License

MIT
