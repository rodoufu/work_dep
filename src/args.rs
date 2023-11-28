use clap::Parser;
use std::sync::Arc;

/// Checks the dependencies of the subprojects of a Rust workspace.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// Path to the project to get the workspace and check the dependencies
    #[arg(short, long)]
    pub(crate) project_path: Arc<str>,
}
