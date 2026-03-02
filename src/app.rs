use std::{ffi::OsString, path::PathBuf};

use anyhow::{Context, Result, bail};
use clap::Parser;

use crate::{
    cli::{Cli, OutputFormat},
    qr,
};

#[derive(Debug)]
pub enum RunOutput {
    Terminal(String),
    Saved { path: PathBuf, format: OutputFormat },
    Version(String),
}

pub fn run_with_args<I, T>(args: I) -> Result<RunOutput>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let cli = Cli::try_parse_from(args)?;
    process_cli(cli)
}

pub fn process_cli(cli: Cli) -> Result<RunOutput> {
    if cli.version {
        return Ok(RunOutput::Version(format!(
            "{} {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )));
    }

    let content = cli.content.as_deref().unwrap_or("").trim();
    if content.is_empty() {
        bail!("input content is empty. Please provide non-empty text.");
    }

    if cli.size == 0 {
        bail!("invalid --size value: must be greater than 0.");
    }

    if cli.output.is_none() && cli.format.is_some() {
        bail!("--format requires --output. Please provide an output file path.");
    }

    match cli.output {
        Some(path) => {
            if let Some(parent) = path.parent()
                && !parent.as_os_str().is_empty()
                && !parent.exists()
            {
                bail!(
                    "output directory does not exist: {}. Please create it first.",
                    parent.display()
                );
            }

            let format = cli.format.unwrap_or(OutputFormat::Png);
            qr::write_image(content, &path, cli.size, format).with_context(|| {
                format!(
                    "could not generate {} at {}",
                    format.as_str().to_uppercase(),
                    path.display()
                )
            })?;
            Ok(RunOutput::Saved { path, format })
        }
        None => {
            let output = qr::render_terminal(content)?;
            Ok(RunOutput::Terminal(output))
        }
    }
}
