use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub cms: Cms,
    pub instance: Instance,
    pub profile: Profile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cms {
    pub name: String,
    pub base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub id: String,
    pub token: String,
    pub requested_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Config> {
    let contents = fs::read_to_string(&path)
        .with_context(|| format!("failed to read {:?}", path.as_ref()))?;

    let config: Config = toml::from_str(&contents)
        .with_context(|| format!("failed to parse {:?}", path.as_ref()))?;

    validate(&config)?;

    Ok(config)
}

fn validate(config: &Config) -> Result<()> {
    anyhow::ensure!(
        !config.instance.id.trim().is_empty(),
        "instance id cannot be empty"
    );

    anyhow::ensure!(
        !config.cms.base_url.trim().is_empty(),
        "cms base_url cannot be empty"
    );

    anyhow::ensure!(
        config.instance.requested_port > 0,
        "port must be greater than 0"
    );

    Ok(())
}
