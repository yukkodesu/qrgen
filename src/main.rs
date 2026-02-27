use anyhow::Result;

use qrgen::app::{RunOutput, run_with_args};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let output = run_with_args(std::env::args_os())?;
    match output {
        RunOutput::Terminal(rendered) => println!("{rendered}"),
        RunOutput::Saved(path) => println!("Saved QR code PNG to {}", path.display()),
    }
    Ok(())
}
