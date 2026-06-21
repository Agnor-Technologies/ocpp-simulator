use config::ResolvedInstance;
use crate::task::InstanceTask;
use crate::state::ChargerState;

#[derive(Debug)]
pub enum InstanceStatus {
    Stopped,
    Starting,
    Running,
    Error(String),
}

#[derive(Debug)]
pub struct RuntimeInstance {
    pub config: ResolvedInstance,

    pub actual_port: Option<u16>,

    pub status: InstanceStatus,


    pub state: ChargerState,

    pub task: Option<InstanceTask>,
}

impl RuntimeInstance {
    pub fn new(
        config: ResolvedInstance,
    ) -> Self {
        Self {
            config,
            actual_port: None,
            status: InstanceStatus::Stopped,
            task: None,
            state: ChargerState::Available,
        }
    }
}
