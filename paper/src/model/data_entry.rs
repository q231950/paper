use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct DataEntry {
    pub label: String,
    pub value: String,
}
