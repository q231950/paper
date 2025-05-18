use super::availability::Availability;

#[derive(Debug, uniffi::Record)]
pub struct ItemAvailability {
    pub availabilities: Vec<Availability>,
}

impl ItemAvailability {
    pub fn new() -> Self {
        ItemAvailability {
            availabilities: Vec::new(),
        }
    }

    pub fn with(availabilities: Vec<Availability>) -> Self {
        ItemAvailability { availabilities }
    }
}

#[derive(Debug, uniffi::Enum)]
pub enum AvailabilityStatus {
    AllAvailable {},
    NoneAvailable {},
    SomeAvailable {},
}
