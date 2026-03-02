# qrgen

A fast command-line tool for generating QR codes. Render in the terminal or save as PNG.

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
  -o, --output <FILE>    Output PNG file path. If omitted, render in terminal.
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
