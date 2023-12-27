use clap::Parser;
use std::path::PathBuf;

/// Checks the dependencies of member projects of a Rust workspace looking for shared ones that could be added to the workspace dependencies.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Path to the workspace it should evaluate the member projects and check the dependencies
    #[arg(short, long)]
    pub(crate) workspace_path: PathBuf,
}
