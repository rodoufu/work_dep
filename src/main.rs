use crate::{args::Args, dependency::Version, workspace::Workspace};
use anyhow::anyhow;
use clap::Parser;
use package::Package;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
    sync::Arc,
};
use toml::Value;

mod args;
mod dependency;
mod package;
mod workspace;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("checking dependencies for: {}", args.project_path);
    let project_path = Path::new(args.project_path.as_ref());
    if !project_path.exists() {
        return Err(anyhow!("{} was not found", args.project_path));
    }
    let cargo_path = project_path.join("Cargo.toml");
    if !cargo_path.exists() {
        return Err(anyhow!("{cargo_path:?} was not found"));
    }

    let workspace_cargo = fs::read_to_string(cargo_path)?;
    let workspace_cargo = workspace_cargo.parse::<Value>()?;
    let workspace_cargo = Workspace::try_from(workspace_cargo)?;

    let mut dependency_not_in_workspace: HashMap<_, HashSet<_>> = HashMap::new();
    let mut packages = HashMap::new();

    for member in workspace_cargo.members {
        let member: Arc<str> = Arc::from(member.as_str());
        let member_path = project_path.join(member.as_ref()).join("Cargo.toml");
        if !member_path.exists() {
            return Err(anyhow!("{member_path:?} was not found"));
        }

        let member_cargo = fs::read_to_string(member_path)?;
        let member_cargo = member_cargo.parse::<Value>()?;
        let member_cargo = Package::try_from(member_cargo)?;
        packages.insert(member.clone(), member_cargo.clone());

        for (dependency_str, dep) in member_cargo.dependencies {
            if matches!(dep.version, Version::Workspace) || matches!(dep.version, Version::Path(_))
            {
                continue;
            }
            dependency_not_in_workspace
                .entry(dependency_str)
                .or_default()
                .insert(member.clone());
        }
        for (dependency_str, dep) in member_cargo.dev_dependencies {
            if matches!(dep.version, Version::Workspace) {
                continue;
            }
            dependency_not_in_workspace
                .entry(dependency_str)
                .or_default()
                .insert(member.clone());
        }
    }

    for (dependency, _) in dependency_not_in_workspace
        .iter()
        .filter(|(_, v)| v.len() > 1)
    {
        let package_version = packages
            .iter()
            .filter_map(|(package_name, v)| {
                match (
                    v.dependencies.get(dependency),
                    v.dev_dependencies.get(dependency),
                ) {
                    (None, Some(dep)) => Some((package_name, &dep.version)),
                    (Some(dep), None) => Some((package_name, &dep.version)),
                    (Some(dep), Some(_)) => Some((package_name, &dep.version)),
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        println!("{dependency} could be in the workspace, it is used by\n{package_version:#?}");
    }

    Ok(())
}
