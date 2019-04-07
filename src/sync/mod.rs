pub struct AccountManager {
    token: String,
}

impl AccountManager {
    pub fn new(token: String) -> AccountManager {
        AccountManager { token }
    }

    pub fn account_info(&self) {
        println!("Getting account info for token: {:?}", self.token.clone());
    }
}
