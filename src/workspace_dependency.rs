use crate::{dependency::Version, package::Package, workspace::Workspace};
use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    io,
    path::PathBuf,
    sync::Arc,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("members not found in workspace {0}")]
    NoMembersInWorkspace(toml::Value),
    #[error("workspace tag not found in {0}")]
    WorkspaceNotFound(toml::Value),
    #[error("unexpected workspace value {0}")]
    WorkspaceUnexpectedValue(toml::Value),
    #[error("{0} was not found")]
    PathNotFound(PathBuf),
    #[error("{0}")]
    IoError(#[from] io::Error),
    #[error("{0}")]
    TomlParse(#[from] toml::de::Error),
    #[error("invalid version {0}")]
    InvalidVersion(toml::Value),
    #[error("unexpected value for Dependency {0}")]
    DependencyUnexpectedValue(toml::Value),
    #[error("property {0} not found in {1}")]
    PropertyNotFound(Arc<str>, toml::Value),
    #[error("unexpected value for Package {0}")]
    PackageUnexpectedValue(toml::Value),
    #[error("invalid output format {0}")]
    InvalidOutputFormat(Arc<str>),
}

#[derive(Debug, Serialize)]
pub struct WorkspaceDependency {
    pub(super) workspace_path: PathBuf,
    pub(super) dependency_project_and_version: HashMap<Arc<str>, HashMap<Arc<str>, Version>>,
}

impl TryFrom<Workspace> for WorkspaceDependency {
    type Error = Error;

    fn try_from(value: Workspace) -> Result<Self, Self::Error> {
        let mut dependency_not_in_workspace: HashMap<_, HashSet<_>> = HashMap::new();
        let mut packages = HashMap::new();

        for (member, member_path) in value.members()? {
            if !member_path.exists() {
                return Err(Self::Error::PathNotFound(member_path));
            }

            let member_cargo = Package::try_from(member_path)?;
            packages.insert(member.clone(), member_cargo.clone());

            for (dependency_str, dep) in member_cargo.dependencies {
                if matches!(dep.version, Version::Workspace)
                    || matches!(dep.version, Version::Path(_))
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

        let mut dependency_project_and_version = HashMap::new();
        for (dependency, _) in dependency_not_in_workspace
            .iter()
            .filter(|(_, v)| v.len() > 1)
        {
            let package_version = packages
                .iter()
                .filter_map(|(_package_name, v)| {
                    match (
                        v.dependencies.get(dependency),
                        v.dev_dependencies.get(dependency),
                    ) {
                        (None, Some(dep)) => Some((v.name.clone(), dep.version.clone())),
                        (Some(dep), None) => Some((v.name.clone(), dep.version.clone())),
                        (Some(dep), Some(_)) => Some((v.name.clone(), dep.version.clone())),
                        _ => None,
                    }
                })
                .collect();

            dependency_project_and_version.insert(dependency.clone(), package_version);
        }

        Ok(Self {
            workspace_path: value.workspace_path,
            dependency_project_and_version,
        })
    }
}
