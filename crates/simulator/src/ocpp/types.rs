#[derive(Debug, Clone, Copy)]
pub enum ChargePointStatus {
    Available,
    Preparing,
    Charging,
    SuspendedEv,
    SuspendedEvse,
    Finishing,
    Reserved,
    Unavailable,
    Faulted,
}

impl ChargePointStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Available => "Available",
            Self::Preparing => "Preparing",
            Self::Charging => "Charging",
            Self::SuspendedEv => "SuspendedEV",
            Self::SuspendedEvse => "SuspendedEVSE",
            Self::Finishing => "Finishing",
            Self::Reserved => "Reserved",
            Self::Unavailable => "Unavailable",
            Self::Faulted => "Faulted",
        }
    }
}
