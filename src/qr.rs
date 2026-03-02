use std::path::Path;

use anyhow::{Context, Result};
use image::{DynamicImage, ImageFormat, Luma, imageops::FilterType};
use qrcode::{QrCode, render::unicode};

use crate::cli::OutputFormat;

pub fn render_terminal(content: &str) -> Result<String> {
    let code = QrCode::new(content.as_bytes()).context("failed to generate QR code")?;

    let rendered = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Dark)
        .light_color(unicode::Dense1x2::Light)
        .build();

    Ok(rendered)
}

pub fn write_image(content: &str, output: &Path, size: u32, format: OutputFormat) -> Result<()> {
    let code = QrCode::new(content.as_bytes()).context("failed to generate QR code")?;

    let raw = code.render::<Luma<u8>>().build();
    let image = image::imageops::resize(&raw, size, size, FilterType::Nearest);
    let dynamic = DynamicImage::ImageLuma8(image);

    let image_format = match format {
        OutputFormat::Avif => ImageFormat::Avif,
        OutputFormat::Png => ImageFormat::Png,
        OutputFormat::Jpeg => ImageFormat::Jpeg,
        OutputFormat::Webp => ImageFormat::WebP,
    };

    dynamic
        .save_with_format(output, image_format)
        .with_context(|| {
            format!(
                "failed to write {} file: {}",
                format.as_str(),
                output.display()
            )
        })?;

    Ok(())
}
