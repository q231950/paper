#[derive(uniffi::Enum)]
pub enum ValidationStatus {
    Valid,         // correct credentials
    Invalid,       // incorrect credentials
    Error(String), // error during verifying credentials
}
