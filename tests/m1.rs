use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use clap::Parser;
use qrgen::{
    app::{RunOutput, run_with_args},
    cli::Cli,
};

#[test]
fn parses_defaults() {
    let cli = Cli::try_parse_from(["qrgen", "hello"]).expect("should parse default args");
    assert_eq!(cli.content, "hello");
    assert_eq!(cli.size, 256);
    assert!(cli.output.is_none());
}

#[test]
fn parses_output_and_size() {
    let cli = Cli::try_parse_from(["qrgen", "hello", "-o", "out.png", "--size", "512"])
        .expect("should parse output + size");
    assert_eq!(cli.size, 512);
    assert_eq!(cli.output, Some(PathBuf::from("out.png")));
}

#[test]
fn run_terminal_success_path() {
    let result = run_with_args(["qrgen", "https://example.com"]).expect("terminal run should work");
    match result {
        RunOutput::Terminal(rendered) => assert!(!rendered.trim().is_empty()),
        RunOutput::Saved(_) => panic!("expected terminal output"),
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
    }

    let image = image::open(&output).expect("png should be readable");
    assert_eq!(image.width(), 300);
    assert_eq!(image.height(), 300);

    std::fs::remove_file(&output).expect("cleanup should remove test artifact");
}

#[test]
fn run_fails_on_empty_content() {
    let err = run_with_args(["qrgen", "   "]).expect_err("empty content should fail");
    assert!(err.to_string().contains("input content is empty"));
}

#[test]
fn run_fails_on_invalid_size() {
    let err =
        run_with_args(["qrgen", "hello", "--size", "0"]).expect_err("size=0 should be rejected");
    assert!(err.to_string().contains("invalid --size value"));
}

#[test]
fn run_fails_when_output_dir_missing() {
    let missing_dir = unique_tmp_png_path("missing-dir");
    let output = missing_dir.join("qr.png");
    let output_arg = output.to_string_lossy().into_owned();

    let err = run_with_args(["qrgen", "hello", "--output", &output_arg])
        .expect_err("missing output directory should fail");
    assert!(err.to_string().contains("output directory does not exist"));
}

fn unique_tmp_png_path(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("qrgen-{prefix}-{nanos}.png"))
}
