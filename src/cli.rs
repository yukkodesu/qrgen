use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "qrgen", version, about = "Generate QR code in terminal or PNG")]
pub struct Cli {
    /// QR content text
    pub content: String,

    /// Output PNG file path. If omitted, render in terminal.
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Output image size in pixels
    #[arg(short = 's', long, value_name = "PIXELS", default_value_t = 256)]
    pub size: u32,
}
