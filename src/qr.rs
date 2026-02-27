use std::path::Path;

use anyhow::{Context, Result};
use image::{Luma, imageops::FilterType};
use qrcode::{QrCode, render::unicode};

pub fn render_terminal(content: &str) -> Result<String> {
    let code = QrCode::new(content.as_bytes()).context("failed to generate QR code")?;

    let rendered = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Dark)
        .light_color(unicode::Dense1x2::Light)
        .build();

    Ok(rendered)
}

pub fn write_png(content: &str, output: &Path, size: u32) -> Result<()> {
    let code = QrCode::new(content.as_bytes()).context("failed to generate QR code")?;

    let raw = code.render::<Luma<u8>>().build();
    let image = image::imageops::resize(&raw, size, size, FilterType::Nearest);

    image
        .save(output)
        .with_context(|| format!("failed to write PNG file: {}", output.display()))?;

    Ok(())
}
