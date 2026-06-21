use crate::{
    instance::{InstanceStatus, RuntimeInstance},
    port,
    state::ChargerState,
};
use config::ResolvedInstance;
use std::collections::HashMap;
use anyhow::{anyhow, Result};
use crate::task::InstanceTask;

#[derive(Debug)]
pub struct SimulatorManager {
    instances: HashMap<String, RuntimeInstance>,
}

impl SimulatorManager {
    pub fn new() -> Self {
        Self {
            instances: HashMap::new(),
        }
    }

    pub fn add_instance(
        &mut self,
        config: ResolvedInstance,
    ) {
        let id = config.instance.id.clone();

        self.instances.insert(
            id,
            RuntimeInstance::new(config),
        );
    }

    pub fn list(
        &self,
    ) -> Vec<&RuntimeInstance> {
        self.instances.values().collect()
    }

    pub fn get(
        &self,
        id: &str,
    ) -> Option<&RuntimeInstance> {
        self.instances.get(id)
    }

    pub fn stop_instance(
        &mut self,
        id: &str,
    ) -> Result<()> {
        let instance = self
        .instances
        .get_mut(id)
        .ok_or_else(|| {
            anyhow!(
                "Unknown instance '{}'",
                id
            )
        })?;

        if let Some(task) = instance.task.take() {
            task.handle.abort();
        }

        instance.actual_port = None;
        instance.status = InstanceStatus::Stopped;

        Ok(())
    }

    pub fn start_instance(
        &mut self,
        id: &str,
    ) -> Result<()> {
        let instance = self
        .instances
        .get_mut(id)
        .ok_or_else(|| {
            anyhow!(
                "Unknown instance '{}'",
                id
            )
        })?;

        if matches!(
            instance.status,
            InstanceStatus::Running
        ) {
            return Ok(());
        }


        let requested =
        instance.config.instance.requested_port;

        let actual =
        port::allocate_port(requested)
        .ok_or_else(|| {
            anyhow!("No available port")
        })?;


        let config =
        instance.config.clone();

        let handle = tokio::spawn(
            crate::runtime::run(config)
        );

        instance.task = Some(
            InstanceTask {
                handle,
            }
        );

        instance.actual_port = Some(actual);
        instance.status = InstanceStatus::Running;

        Ok(())
    }
    pub fn start_transaction(
        &mut self,
        id: &str,
    ) -> Result<()> {
        let instance = self
        .instances
        .get_mut(id)
        .ok_or_else(|| anyhow!("Unknown instance"))?;

        instance.state =
        ChargerState::Charging;

        Ok(())
    }
    pub fn stop_transaction(
        &mut self,
        id: &str,
    ) -> Result<()> {
        let instance = self
        .instances
        .get_mut(id)
        .ok_or_else(|| anyhow!("Unknown instance"))?;

        instance.state =
        ChargerState::Available;

        Ok(())
    }
    pub fn ids(
        &self,
    ) -> Vec<String> {
        self.instances
        .keys()
        .cloned()
        .collect()
    }

}
