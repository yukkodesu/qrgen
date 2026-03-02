use anyhow::Result;
use clap::{Error as ClapError, error::ErrorKind};

use qrgen::app::{RunOutput, run_with_args};

fn main() {
    if let Err(err) = run() {
        if let Some(clap_err) = err.downcast_ref::<ClapError>() {
            let _ = clap_err.print();
            let code = match clap_err.kind() {
                ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand => 0,
                _ => clap_err.exit_code(),
            };
            std::process::exit(code);
        }

        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let output = run_with_args(std::env::args_os())?;
    match output {
        RunOutput::Terminal(rendered) => println!("{rendered}"),
        RunOutput::Saved { path, format } => {
            println!(
                "Saved QR code {} to {}",
                format.as_str().to_uppercase(),
                path.display()
            )
        }
        RunOutput::Version(version) => println!("{version}"),
    }
    Ok(())
}
