use std::{fs, path::PathBuf, sync::Arc};
use toml::Value;

const CARGO_TOML: &str = "Cargo.toml";

#[derive(Debug)]
pub struct Workspace {
    members: Vec<Arc<str>>,
    pub(super) workspace_path: PathBuf,
}

impl Workspace {
    pub(crate) fn members(
        &self,
    ) -> Result<Vec<(Arc<str>, PathBuf)>, super::workspace_dependency::Error> {
        Ok(self
            .members
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
                    Result::<_, super::workspace_dependency::Error>::Ok(resp)
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

impl TryFrom<PathBuf> for Workspace {
    type Error = super::workspace_dependency::Error;

    fn try_from(workspace_path: PathBuf) -> Result<Self, Self::Error> {
        if !workspace_path.exists() {
            return Err(Self::Error::PathNotFound(workspace_path));
        }
        let cargo_path = workspace_path.join(CARGO_TOML);
        if !cargo_path.exists() {
            return Err(Self::Error::PathNotFound(cargo_path));
        }

        let workspace_cargo = fs::read_to_string(cargo_path)?;
        let workspace_cargo = workspace_cargo.parse::<Value>()?;

        if let Value::Table(table) = &workspace_cargo {
            let mut members = vec![];

            if let Some(workspace) = table.get("workspace") {
                if let Some(Value::Array(members_vec)) = workspace.get("members") {
                    for member in members_vec {
                        if let Value::String(member) = member {
                            members.push(member.as_str().into());
                        }
                    }
                } else {
                    return Err(Self::Error::NoMembersInWorkspace(workspace.clone()));
                }

                Ok(Self {
                    members,
                    workspace_path,
                })
            } else {
                Err(Self::Error::WorkspaceNotFound(workspace_cargo))
            }
        } else {
            Err(Self::Error::WorkspaceUnexpectedValue(workspace_cargo))
        }
    }
}
