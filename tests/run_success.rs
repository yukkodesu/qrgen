mod common;

use qrgen::app::{RunOutput, run_with_args};

use common::unique_tmp_image_path;

#[test]
fn run_terminal_success_path() {
    let result = run_with_args(["qrgen", "https://example.com"]).expect("terminal run should work");
    match result {
        RunOutput::Terminal(rendered) => assert!(!rendered.trim().is_empty()),
        RunOutput::Saved { .. } => panic!("expected terminal output"),
        RunOutput::Version(_) => panic!("expected terminal output"),
    }
}

#[test]
fn run_png_success_path_creates_file_with_size() {
    let output = unique_tmp_image_path("m4-png-success", "png");
    let output_arg = output.to_string_lossy().into_owned();
    let args = [
        "qrgen",
        "https://example.com",
        "--output",
        &output_arg,
        "--size",
        "300",
    ];

    let result = run_with_args(args).expect("png run should work");
    match result {
        RunOutput::Saved { path, format } => {
            assert_eq!(path, output);
            assert_eq!(format.as_str(), "png");
        }
        RunOutput::Terminal(_) => panic!("expected file output"),
        RunOutput::Version(_) => panic!("expected file output"),
    }

    let image = image::open(&output).expect("png should be readable");
    assert_eq!(image.width(), 300);
    assert_eq!(image.height(), 300);

    std::fs::remove_file(&output).expect("cleanup should remove test artifact");
}

#[test]
fn run_webp_success_path_creates_file() {
    let output = unique_tmp_image_path("m4-webp-success", "webp");
    let output_arg = output.to_string_lossy().into_owned();
    let args = [
        "qrgen",
        "https://example.com",
        "--output",
        &output_arg,
        "--format",
        "webp",
    ];

    let result = run_with_args(args).expect("webp run should work");
    match result {
        RunOutput::Saved { path, format } => {
            assert_eq!(path, output);
            assert_eq!(format.as_str(), "webp");
        }
        RunOutput::Terminal(_) => panic!("expected file output"),
        RunOutput::Version(_) => panic!("expected file output"),
    }

    let image = image::open(&output).expect("webp should be readable");
    assert!(image.width() > 0);
    assert!(image.height() > 0);

    std::fs::remove_file(&output).expect("cleanup should remove test artifact");
}
