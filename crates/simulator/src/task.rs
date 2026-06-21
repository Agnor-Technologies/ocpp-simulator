use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct InstanceTask {
    pub handle: JoinHandle<()>,

}

