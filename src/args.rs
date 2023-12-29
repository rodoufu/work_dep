use clap::Parser;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Default, Clone)]
pub(crate) enum OutputFormat {
    #[default]
    Text,
    #[allow(clippy::upper_case_acronyms)]
    JSON,
}

impl FromStr for OutputFormat {
    type Err = super::workspace_dependency::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::JSON),
            other => Err(Self::Err::InvalidOutputFormat(other.into())),
        }
    }
}

/// Checks the dependencies of member projects of a Rust workspace looking for shared ones that could be added to the workspace dependencies.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Path to the workspace it should evaluate the member projects and check the dependencies
    #[arg(short, long)]
    pub(crate) workspace_path: PathBuf,

    /// Indicates the format to print the response
    #[arg(short, long, default_value = "text")]
    pub(crate) output_format: OutputFormat,
}
