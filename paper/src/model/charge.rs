#[derive(uniffi::Record, Debug)]
pub struct Charge {
    pub timestamp: i64,
    pub amount_owed: f64,
    pub amount_payed: f64,
    pub reason: String,
    pub item: String,
    pub source: String,
}

impl Charge {
    pub(crate) fn new() -> Self {
        Charge {
            timestamp: 0,
            amount_owed: 0.0,
            amount_payed: 0.0,
            reason: "".to_string(),
            item: "".to_string(),
            source: "".to_string(),
        }
    }
}
