use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub cms: Cms,
    pub instance: Instance,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cms {
    pub name: String,
    pub base_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instance {
    pub id: String,
    pub token: String,
    pub requested_port: u16,
}
