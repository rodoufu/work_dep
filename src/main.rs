use crate::{args::Args, dependency::Version, workspace::Workspace};
use anyhow::anyhow;
use clap::Parser;
use package::Package;
use std::{
    collections::{HashMap, HashSet},
    fs,
};
use toml::Value;

mod args;
mod dependency;
mod package;
mod workspace;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!(
        "checking dependencies for: {}",
        args.workspace_path.display()
    );
    let workspace_cargo = Workspace::try_from(args.workspace_path)?;

    let mut dependency_not_in_workspace: HashMap<_, HashSet<_>> = HashMap::new();
    let mut packages = HashMap::new();

    for (member, member_path) in workspace_cargo.members()? {
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
