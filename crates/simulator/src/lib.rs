use shared_types::InstanceInfo;

pub struct Simulator {
    pub instance: InstanceInfo,
}

impl Simulator {
    pub fn new(instance: InstanceInfo) -> Self {
        Self { instance }
    }
}
