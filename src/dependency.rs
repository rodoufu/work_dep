use anyhow::{anyhow, Error};
use std::sync::Arc;
use toml::Value;

/// Types of versions in a TOML file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Version {
    /// Workspace dependency
    Workspace,
    /// Version number
    Value(Arc<str>),
    /// Points to a git repository and a branch
    GitBranch((Arc<str>, Arc<str>)),
    /// Points to a relative path in the file system
    Path(Arc<str>),
}

#[derive(Debug, Clone)]
pub(crate) struct Dependency {
    pub(crate) _name: Arc<str>,
    pub(crate) version: Version,
}

impl Default for Dependency {
    fn default() -> Self {
        Self {
            _name: Arc::from(""),
            version: Version::Workspace,
        }
    }
}

impl TryFrom<(&String, &toml::Value)> for Dependency {
    type Error = Error;

    fn try_from((name, value): (&String, &Value)) -> Result<Self, Self::Error> {
        match value {
            Value::String(value_str) => Ok(Self {
                _name: Arc::from(name.as_str()),
                version: Version::Value(Arc::from(value_str.as_str())),
            }),
            Value::Table(table) => {
                let mut version = None;
                if let Some(Value::Boolean(workspace_bool)) = table.get("workspace") {
                    if *workspace_bool {
                        version = Some(Version::Workspace);
                    }
                }
                if let Some(Value::String(version_str)) = table.get("version") {
                    version = Some(Version::Value(version_str.as_str().into()));
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
                    Ok(Self {
                        _name: Arc::from(name.as_str()),
                        version,
                    })
                } else {
                    Err(anyhow!("invalid version {table:?}"))
                }
            }
            _ => Err(anyhow!("unexpected value for Dependency {value:?}")),
        }
    }
}
