#[derive(Debug)]
pub struct AccountInfo {
    pub firstname: String,
}

impl AccountInfo {
    pub fn new() -> AccountInfo {
        AccountInfo {
            firstname: "".to_string(),
        }
    }

    pub fn with_firstname(self, firstname: String) -> AccountInfo {
        AccountInfo { firstname, ..self }
    }
}
