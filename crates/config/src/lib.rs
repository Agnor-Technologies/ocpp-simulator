use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmsConfig {
    pub name: String,
    pub base_url: String,
    pub http_auth: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub name: String,
    pub vendor: String,
    pub model: String,
    pub max_offer: u32,
    pub min_to_start: u32,
    pub metervalues_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceConfig {
    pub id: String,
    pub cms: String,
    pub profile: String,
    pub token: String,
    pub requested_port: u16,
}

#[derive(Debug, Clone)]
pub struct ResolvedInstance {
    pub instance: InstanceConfig,
    pub cms: CmsConfig,
    pub profile: ProfileConfig,
}

#[derive(Debug)]
pub struct ConfigManager {
    pub cms: HashMap<String, CmsConfig>,
    pub profiles: HashMap<String, ProfileConfig>,
    pub instances: HashMap<String, InstanceConfig>,
}

fn load_directory<T>(
    path: &Path,
) -> Result<HashMap<String, T>>
where
T: for<'de> Deserialize<'de>,
{
    let mut map = HashMap::new();

    if !path.exists() {
        println!("Directory does not exist!");
        return Ok(map);
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("toml") {
            continue;
        }

        let contents = fs::read_to_string(&path)?;

        let value: T = toml::from_str(&contents)?;

        let key = path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();

        map.insert(key, value);
    }

    Ok(map)
}

impl ConfigManager {
    fn validate(&self) -> Result<()> {
        for instance in self.instances.values() {
            if !self.cms.contains_key(&instance.cms) {
                bail!(
                    "Instance '{}' references unknown CMS '{}'",
                    instance.id,
                    instance.cms
                );
            }
        }

        for instance in self.instances.values() {
            if !self.profiles.contains_key(&instance.profile) {
                bail!(
                    "Instance '{}' references unknown profile '{}'",
                    instance.id,
                    instance.profile
                );
            }
        }

        let mut ports = HashSet::new();

        for instance in self.instances.values() {
            if !ports.insert(instance.requested_port) {
                bail!(
                    "Duplicate requested port {}",
                    instance.requested_port
                );
            }
        }

        Ok(())
    }

    pub fn resolve_all(
        &self,
    ) -> Result<Vec<ResolvedInstance>> {
        self.instances
            .values()
            .map(|instance| {
                self.resolve(&instance.id)
            })
            .collect()
    }

    pub fn load(
        root: impl AsRef<Path>,
    ) -> Result<Self> {
        let root = root.as_ref();

        let cms =
        load_directory::<CmsConfig>(
            &root.join("cms")
        )?;

        let profiles =
        load_directory::<ProfileConfig>(
            &root.join("profiles")
        )?;

        let instances =
        load_directory::<InstanceConfig>(
            &root.join("instances")
        )?;

        let manager = Self {
            cms,
            profiles,
            instances,
        };

        manager.validate()?;

        Ok(manager)
    }

    pub fn resolve(
        &self,
        instance_id: &str,
    ) -> Result<ResolvedInstance> {
        let instance = self
        .instances
        .values()
        .find(|i| i.id == instance_id)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Unknown instance '{}'",
                instance_id
            )
        })?;

        let cms = self
        .cms
        .get(&instance.cms)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Unknown CMS '{}'",
                instance.cms
            )
        })?;

        let profile = self
        .profiles
        .get(&instance.profile)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Unknown profile '{}'",
                instance.profile
            )
        })?;

        Ok(ResolvedInstance {
            instance: instance.clone(),
           cms: cms.clone(),
           profile: profile.clone(),
        })
    }
}
