use crate::dependency::Dependency;
use anyhow::{anyhow, Error};
use std::{collections::HashMap, sync::Arc};
use toml::Value;

#[derive(Debug, Default)]
pub struct Workspace {
    pub(crate) members: Vec<String>,
    pub(crate) _dependencies: HashMap<Arc<str>, Dependency>,
}

impl TryFrom<Value> for Workspace {
    type Error = Error;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Table(table) = value {
            let mut members = vec![];
            let mut dependencies = HashMap::new();

            if let Some(workspace) = table.get("workspace") {
                if let Some(Value::Array(members_vec)) = workspace.get("members") {
                    for member in members_vec {
                        if let Value::String(member) = member {
                            members.push(member.into());
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
                    members,
                    _dependencies: dependencies,
                })
            } else {
                Err(anyhow!("workspace not found {table:?}"))
            }
        } else {
            Err(anyhow!("unexpected workspace value {value:?}"))
        }
    }
}
