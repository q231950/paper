#[derive(Debug)]
pub struct AccountInfo {
    pub readableFullName: Option<String>,
    pub categoryName: Option<String>,
    pub accountBalance: Option<String>,
    pub badgeReplacementCharge: Option<String>,
    pub creditBalance: Option<String>,
    pub mandatoryCreditBalance: Option<String>,
    pub pseudoForename: Option<String>,
    pub creationDate: Option<String>,
    pub fullName: Option<String>,
    pub birthDate: Option<String>,
    pub branchName: Option<String>,
    pub amount: Option<String>, // the monetary amount of the subscription
    pub change: Option<String>, // the last modification date of the subscription
    pub start: Option<String>,  // the start date of the current subscription
    pub expiry: Option<String>, // the expiry date of the current subscription
    pub expiryMonth: Option<String>, // the month of the next expiration
    pub expiryYear: Option<String>, // the year of the next expiration
    pub postcode: Option<String>,
    pub surname: Option<String>,
    pub forename: Option<String>,
    pub emailAddress: Option<String>,
    pub addressLine1: Option<String>,
    pub addressLine2: Option<String>,
    pub acronym: Option<String>,
}

impl AccountInfo {
    pub fn new() -> AccountInfo {
        AccountInfo {
            readableFullName: None,
            categoryName: None,
            accountBalance: None,
            badgeReplacementCharge: None,
            creditBalance: None,
            mandatoryCreditBalance: None,
            pseudoForename: None,
            creationDate: None,
            fullName: None,
            birthDate: None,
            branchName: None,
            amount: None,
            change: None,
            start: None,
            expiry: None,
            expiryMonth: None,
            expiryYear: None,
            postcode: None,
            surname: None,
            forename: None,
            emailAddress: None,
            addressLine1: None,
            addressLine2: None,
            acronym: None,
        }
    }
}
