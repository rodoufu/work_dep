use crate::dependency::Dependency;
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use toml::Value;

#[derive(Debug, Clone)]
pub(crate) struct Package {
    pub(super) name: Arc<str>,
    pub(super) dependencies: HashMap<Arc<str>, Dependency>,
    pub(super) dev_dependencies: HashMap<Arc<str>, Dependency>,
}

impl TryFrom<PathBuf> for Package {
    type Error = super::workspace_dependency::Error;

    fn try_from(member_path: PathBuf) -> Result<Self, Self::Error> {
        let member_cargo = fs::read_to_string(member_path)?;
        let member_cargo = member_cargo.parse::<Value>()?;

        if let Value::Table(table) = member_cargo {
            let name;
            let mut dependencies = HashMap::new();
            let mut dev_dependencies = HashMap::new();

            if let Some(Value::Table(package_tab)) = table.get("package") {
                if let Some(Value::String(name_str)) = package_tab.get("name") {
                    name = Arc::from(name_str.as_str());
                } else {
                    return Err(Self::Error::PropertyNotFound(
                        "name".into(),
                        Value::Table(package_tab.clone()),
                    ));
                }
            } else {
                return Err(Self::Error::PropertyNotFound(
                    "package".into(),
                    Value::Table(table),
                ));
            }

            if let Some(Value::Table(dependencies_table)) = table.get("dependencies") {
                for (name, content) in dependencies_table {
                    dependencies.insert(Arc::from(name.as_str()), Dependency::try_from(content)?);
                }
            }

            if let Some(Value::Table(dependencies_table)) = table.get("dev-dependencies") {
                for (name, content) in dependencies_table {
                    dev_dependencies
                        .insert(Arc::from(name.as_str()), Dependency::try_from(content)?);
                }
            }

            Ok(Self {
                name,
                dependencies,
                dev_dependencies,
            })
        } else {
            Err(Self::Error::PackageUnexpectedValue(member_cargo))
        }
    }
}
