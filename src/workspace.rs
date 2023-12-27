use crate::dependency::Dependency;
use anyhow::{anyhow, Error};
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use toml::Value;

const CARGO_TOML: &str = "Cargo.toml";

#[derive(Debug, Default)]
pub struct Workspace {
    pub(crate) _members: Vec<Arc<str>>,
    pub(crate) _dependencies: HashMap<Arc<str>, Dependency>,
    pub(crate) workspace_path: PathBuf,
}

impl Workspace {
    pub(crate) fn members(&self) -> anyhow::Result<Vec<(Arc<str>, PathBuf)>> {
        Ok(self
            ._members
            .iter()
            .flat_map(|member| {
                if member.ends_with('*') {
                    let member_folder = self
                        .workspace_path
                        .join(&member.as_ref()[..member.len() - 2]);
                    let mut resp = Vec::new();
                    for file in fs::read_dir(member_folder)? {
                        let file = file?.path();
                        resp.push((
                            Arc::<str>::from(format!("{}", file.display()).as_str()),
                            file.join(CARGO_TOML),
                        ))
                    }
                    Result::<_, anyhow::Error>::Ok(resp)
                } else {
                    Ok(vec![(
                        member.clone(),
                        self.workspace_path.join(member.as_ref()).join(CARGO_TOML),
                    )])
                }
            })
            .flatten()
            .collect())
    }
}

impl TryFrom<(PathBuf, Value)> for Workspace {
    type Error = Error;
    fn try_from((workspace_path, value): (PathBuf, Value)) -> Result<Self, Self::Error> {
        if let Value::Table(table) = value {
            let mut members = vec![];
            let mut dependencies = HashMap::new();

            if let Some(workspace) = table.get("workspace") {
                if let Some(Value::Array(members_vec)) = workspace.get("members") {
                    for member in members_vec {
                        if let Value::String(member) = member {
                            members.push(member.as_str().into());
                        }
                    }
                } else {
                    return Err(anyhow!("members not found in {workspace:?}"));
                }

                if let Some(Value::Table(depencencies_table)) = workspace.get("dependencies") {
                    for (name, content) in depencencies_table {
                        dependencies.insert(
                            Arc::from(name.as_str()),
                            Dependency::try_from((name, content))?,
                        );
                    }
                }
                Ok(Self {
                    _members: members,
                    _dependencies: dependencies,
                    workspace_path,
                })
            } else {
                Err(anyhow!("workspace not found {table:?}"))
            }
        } else {
            Err(anyhow!("unexpected workspace value {value:?}"))
        }
    }
}

impl TryFrom<PathBuf> for Workspace {
    type Error = Error;

    fn try_from(workspace_path: PathBuf) -> Result<Self, Self::Error> {
        if !workspace_path.exists() {
            return Err(anyhow!("{} was not found", workspace_path.display()));
        }
        let cargo_path = workspace_path.join(CARGO_TOML);
        if !cargo_path.exists() {
            return Err(anyhow!("{cargo_path:?} was not found"));
        }

        let workspace_cargo = fs::read_to_string(cargo_path)?;
        let workspace_cargo = workspace_cargo.parse::<Value>()?;
        Workspace::try_from((workspace_path, workspace_cargo))
    }
}
