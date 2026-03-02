mod common;

use qrgen::app::run_with_args;

use common::unique_tmp_image_path;

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
    let missing_dir = unique_tmp_image_path("missing-dir", "png");
    let output = missing_dir.join("qr.png");
    let output_arg = output.to_string_lossy().into_owned();

    let err = run_with_args(["qrgen", "hello", "--output", &output_arg])
        .expect_err("missing output directory should fail");
    assert!(err.to_string().contains("output directory does not exist"));
}

#[test]
fn run_fails_when_format_without_output() {
    let err = run_with_args(["qrgen", "hello", "--format", "webp"])
        .expect_err("format requires output path");
    assert!(err.to_string().contains("--format requires --output"));
}

#[test]
fn run_fails_for_invalid_format_value() {
    let output = unique_tmp_image_path("m4-invalid-format", "png");
    let output_arg = output.to_string_lossy().into_owned();
    let err = run_with_args(["qrgen", "hello", "--output", &output_arg, "--format", "gif"])
        .expect_err("invalid format should fail");
    assert!(err.to_string().contains("invalid value"));
}
