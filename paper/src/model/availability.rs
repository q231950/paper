use super::location::Location;

#[derive(Debug, uniffi::Enum)]
pub enum Availability {
    Available(Location),
    NotAvailable(Location),
    Unknown(Location),
}
