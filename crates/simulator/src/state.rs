#[derive(Debug, Clone)]
pub enum ChargerState {
    Available,
    Preparing,
    Charging,
    Finishing,
    Faulted,
}
