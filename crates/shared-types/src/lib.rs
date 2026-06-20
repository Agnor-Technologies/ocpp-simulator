use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceInfo {
    pub id: String,
    pub requested_port: u16,
}
