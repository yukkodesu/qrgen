use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Avif,
    Png,
    Jpeg,
    Webp,
}

impl OutputFormat {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Avif => "avif",
            Self::Png => "png",
            Self::Jpeg => "jpeg",
            Self::Webp => "webp",
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "qrgen",
    about = "Generate QR code in terminal or image file",
    arg_required_else_help = true,
    after_help = "Examples:\n  qrgen \"https://example.com\"\n  qrgen \"https://example.com\" -o ./dist/site.png\n  qrgen \"https://example.com\" -o ./dist/site.webp --format webp\n  qrgen \"WIFI:T:WPA;S:MyWiFi;P:12345678;;\" --size 512"
)]
pub struct Cli {
    /// QR content text
    #[arg(required_unless_present = "version", value_name = "CONTENT")]
    pub content: Option<String>,

    /// Output image file path. If omitted, render in terminal.
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Output format for file export (requires --output)
    #[arg(long, value_enum, value_name = "FORMAT")]
    pub format: Option<OutputFormat>,

    /// Output image size in pixels
    #[arg(short = 's', long, value_name = "PIXELS", default_value_t = 256)]
    pub size: u32,

    /// Print version information
    #[arg(short = 'v', long = "version", default_value_t = false)]
    pub version: bool,
}
