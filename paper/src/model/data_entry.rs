use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, uniffi::Record)]
pub struct DataEntry {
    pub label: String,
    pub value: String,
}
