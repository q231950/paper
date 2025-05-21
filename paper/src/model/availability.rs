use super::location::Location;

#[derive(Debug, PartialEq, uniffi::Enum)]
pub enum Availability {
    Available(Location),
    NotAvailable(Location),
    Unknown(Location),
}
