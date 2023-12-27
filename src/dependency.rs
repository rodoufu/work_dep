use serde::Serialize;
use std::{path::PathBuf, sync::Arc};
use toml::Value;

/// Types of versions in a TOML file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) enum Version {
    /// Workspace dependency
    Workspace,
    /// Version number
    Version(Arc<str>),
    /// Points to a git repository and a branch
    GitBranch((Arc<str>, Arc<str>)),
    /// Points to a relative path in the file system
    Path(PathBuf),
}

#[derive(Debug, Clone)]
pub(crate) struct Dependency {
    pub(super) version: Version,
}

impl Default for Dependency {
    fn default() -> Self {
        Self {
            version: Version::Workspace,
        }
    }
}

impl TryFrom<&toml::Value> for Dependency {
    type Error = super::workspace_dependency::Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(value_str) => Ok(Self {
                version: Version::Version(Arc::from(value_str.as_str())),
            }),
            Value::Table(table) => {
                let mut version = None;
                if let Some(Value::Boolean(workspace_bool)) = table.get("workspace") {
                    if *workspace_bool {
                        version = Some(Version::Workspace);
                    }
                }
                if let Some(Value::String(version_str)) = table.get("version") {
                    version = Some(Version::Version(version_str.as_str().into()));
                }
                if let Some(Value::String(path_str)) = table.get("path") {
                    version = Some(Version::Path(path_str.as_str().into()));
                }
                if let (Some(Value::String(git_str)), Some(Value::String(branch_str))) =
                    (table.get("git"), table.get("branch"))
                {
                    version = Some(Version::GitBranch((
                        git_str.as_str().into(),
                        branch_str.as_str().into(),
                    )));
                }

                if let Some(version) = version {
                    Ok(Self { version })
                } else {
                    Err(Self::Error::InvalidVersion(value.clone()))
                }
            }
            _ => Err(Self::Error::DependencyUnexpectedValue(value.clone())),
        }
    }
}
