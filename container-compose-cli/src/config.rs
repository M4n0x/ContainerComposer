use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContainerComposeConfig {
    #[serde(default = "default_version")]
    pub version: String,
    pub services: HashMap<String, Service>,
    #[serde(default)]
    pub volumes: HashMap<String, Volume>,
    #[serde(default)]
    pub networks: HashMap<String, Network>,
}

fn default_version() -> String {
    "1.0".to_string()
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Service {
    pub image: String,
    #[serde(default)]
    pub ports: Vec<String>,
    #[serde(default)]
    pub volumes: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_environment")]
    pub environment: Vec<String>,
    #[serde(default)]
    pub depends_on: Vec<String>,
    pub command: Option<Vec<String>>,
    pub working_dir: Option<String>,
}

// Custom deserializer for environment that handles both array and object formats
fn deserialize_environment<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        // Array format: ["KEY=value", "KEY2=value2"]
        Value::Sequence(seq) => {
            let mut env_vars = Vec::new();
            for item in seq {
                if let Some(env_str) = item.as_str() {
                    env_vars.push(env_str.to_string());
                }
            }
            Ok(env_vars)
        }
        // Object format: {KEY: value, KEY2: value2}
        Value::Mapping(map) => {
            let mut env_vars = Vec::new();
            for (key, value) in map {
                if let (Some(key_str), Some(value_str)) = (key.as_str(), value.as_str()) {
                    env_vars.push(format!("{key_str}={value_str}"));
                }
            }
            Ok(env_vars)
        }
        // Single string (edge case)
        Value::String(s) => Ok(vec![s]),
        // Default to empty
        _ => Ok(Vec::new()),
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Volume {
    #[serde(default)]
    pub driver: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Network {
    #[serde(default = "default_driver")]
    pub driver: String,
}

fn default_driver() -> String {
    "bridge".to_string()
}

impl ContainerComposeConfig {
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: ContainerComposeConfig = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
    pub fn validate(&self) -> anyhow::Result<()> {
        // Check if all services have valid images
        for (name, service) in &self.services {
            if service.image.is_empty() {
                return Err(anyhow::anyhow!("Service '{}' has no image specified", name));
            }
        }

        // Check dependencies exist
        for (name, service) in &self.services {
            for dep in &service.depends_on {
                if !self.services.contains_key(dep) {
                    return Err(anyhow::anyhow!(
                        "Service '{}' depends on '{}' which doesn't exist",
                        name,
                        dep
                    ));
                }
            }
        }

        Ok(())
    }
}
