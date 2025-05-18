use crate::model::Charge;

#[derive(uniffi::Record, Debug)]
pub struct Balance {
    pub total: String,
    pub charges: Vec<Charge>,
}
