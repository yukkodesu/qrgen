mod common;

use qrgen::app::{RunOutput, run_with_args};

use common::unique_tmp_png_path;

#[test]
fn run_terminal_success_path() {
    let result = run_with_args(["qrgen", "https://example.com"]).expect("terminal run should work");
    match result {
        RunOutput::Terminal(rendered) => assert!(!rendered.trim().is_empty()),
        RunOutput::Saved(_) => panic!("expected terminal output"),
        RunOutput::Version(_) => panic!("expected terminal output"),
    }
}

#[test]
fn run_png_success_path_creates_file_with_size() {
    let output = unique_tmp_png_path("m1-success");
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
        RunOutput::Saved(path) => assert_eq!(path, output),
        RunOutput::Terminal(_) => panic!("expected file output"),
        RunOutput::Version(_) => panic!("expected file output"),
    }

    let image = image::open(&output).expect("png should be readable");
    assert_eq!(image.width(), 300);
    assert_eq!(image.height(), 300);

    std::fs::remove_file(&output).expect("cleanup should remove test artifact");
}
