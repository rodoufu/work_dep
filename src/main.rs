use crate::{args::Args, workspace::Workspace, workspace_dependency::WorkspaceDependency};
use clap::Parser;

mod args;
mod dependency;
mod package;
mod workspace;
mod workspace_dependency;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let workspace_cargo = Workspace::try_from(args.workspace_path)?;

    let workspace_dependency = WorkspaceDependency::try_from(workspace_cargo)?;

    match args.output_format {
        args::OutputFormat::Text => {
            println!(
                "checking dependencies for: {}",
                workspace_dependency.workspace_path.display(),
            );

            for (dependency, package_version) in workspace_dependency.dependency_project_and_version
            {
                println!(
                    "{dependency} could be in the workspace, it is used by\n{package_version:#?}"
                );
            }
        }
        args::OutputFormat::JSON => {
            println!("{}", serde_json::to_string(&workspace_dependency)?);
        }
    }

    Ok(())
}
