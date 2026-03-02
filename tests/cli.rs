use std::path::PathBuf;

use clap::Parser;
use qrgen::{
    app::{RunOutput, run_with_args},
    cli::Cli,
};

#[test]
fn parses_defaults() {
    let cli = Cli::try_parse_from(["qrgen", "hello"]).expect("should parse default args");
    assert_eq!(cli.content, Some("hello".to_string()));
    assert_eq!(cli.size, 256);
    assert!(cli.output.is_none());
    assert!(cli.format.is_none());
}

#[test]
fn parses_output_and_size() {
    let cli = Cli::try_parse_from(["qrgen", "hello", "-o", "out.png", "--size", "512"])
        .expect("should parse output + size");
    assert_eq!(cli.size, 512);
    assert_eq!(cli.output, Some(PathBuf::from("out.png")));
}

#[test]
fn parses_format() {
    let cli = Cli::try_parse_from(["qrgen", "hello", "-o", "out.webp", "--format", "webp"])
        .expect("should parse format");
    assert_eq!(cli.output, Some(PathBuf::from("out.webp")));
    assert_eq!(
        cli.format.map(|format| format.as_str().to_string()),
        Some("webp".to_string())
    );
}

#[test]
fn no_args_shows_help_with_examples() {
    let err = Cli::try_parse_from(["qrgen"]).expect_err("missing args should show help");
    let text = err.to_string();
    assert!(text.contains("Usage:"));
    assert!(text.contains("Examples:"));
}

#[test]
fn help_includes_version_short_flag() {
    let err = Cli::try_parse_from(["qrgen", "-h"]).expect_err("help should short-circuit");
    let text = err.to_string();
    assert!(text.contains("-v, --version"));
    assert!(text.contains("--format <FORMAT>"));
    assert!(text.contains("Examples:"));
}

#[test]
fn short_v_prints_version() {
    let result = run_with_args(["qrgen", "-v"]).expect("version flag should succeed");
    match result {
        RunOutput::Version(version) => assert!(version.contains(env!("CARGO_PKG_VERSION"))),
        _ => panic!("expected version output"),
    }
}
