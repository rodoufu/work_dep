use crate::dependency::Dependency;
use anyhow::{anyhow, Error};
use std::{collections::HashMap, sync::Arc};
use toml::Value;

#[derive(Debug, Clone)]
pub(crate) struct Package {
    pub(crate) _name: Arc<str>,
    pub(crate) dependencies: HashMap<Arc<str>, Dependency>,
    pub(crate) dev_dependencies: HashMap<Arc<str>, Dependency>,
}

impl TryFrom<Value> for Package {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Table(table) = value {
            let name;
            let mut dependencies = HashMap::new();
            let mut dev_dependencies = HashMap::new();

            if let Some(Value::Table(package_tab)) = table.get("package") {
                if let Some(Value::String(name_str)) = package_tab.get("name") {
                    name = Arc::from(name_str.as_str());
                } else {
                    return Err(anyhow!("name not found in {package_tab:?}"));
                }
            } else {
                return Err(anyhow!("package not found in {table:?}"));
            }

            if let Some(Value::Table(dependencies_table)) = table.get("dependencies") {
                for (name, content) in dependencies_table {
                    dependencies.insert(
                        Arc::from(name.as_str()),
                        Dependency::try_from((name, content))?,
                    );
                }
            }

            if let Some(Value::Table(dependencies_table)) = table.get("dev-dependencies") {
                for (name, content) in dependencies_table {
                    dev_dependencies.insert(
                        Arc::from(name.as_str()),
                        Dependency::try_from((name, content))?,
                    );
                }
            }

            Ok(Self {
                _name: name,
                dependencies,
                dev_dependencies,
            })
        } else {
            Err(anyhow!("unexpected package value {value:?}"))
        }
    }
}
