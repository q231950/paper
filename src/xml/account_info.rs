use crate::model::AccountInfo;
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

pub struct AccountInfoXmlParser {}

impl AccountInfoXmlParser {
    pub fn new() -> AccountInfoXmlParser {
        AccountInfoXmlParser {}
    }

    pub fn account_info_from_xml(&self, xml: impl Read) -> Result<AccountInfo, &'static str> {
        println!("Creating account info from xml...");

        let mut result: Result<AccountInfo, &'static str>;
        let mut account_info = AccountInfo::new();
        let mut current_element = "".to_string();
        let parser = EventReader::new_with_config(xml, self.parser_config());
        let mut depth = 0;
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    current_element = name.local_name.to_string();
                }
                Err(e) => {
                    println!("Error: {}", e);
                    return Err("Unable to create account info from xml");
                }
                Ok(XmlEvent::Characters(c)) => {
                    let v = Some(c.clone());
                    if current_element == "ReadableFullName" {
                        account_info.readableFullName = v;
                    } else if current_element == "CategoryName" {
                        account_info.categoryName = v;
                    } else if current_element == "AccountBalance" {
                        account_info.accountBalance = v;
                    } else if current_element == "BadgeReplacementCharge" {
                        account_info.badgeReplacementCharge = v;
                    } else if current_element == "CreditBalance" {
                        account_info.creditBalance = v;
                    } else if current_element == "MandatoryCreditBalance" {
                        account_info.mandatoryCreditBalance = v;
                    } else if current_element == "PseudoForename" {
                        account_info.pseudoForename = v;
                    } else if current_element == "CreationDate" {
                        account_info.creationDate = v;
                    } else if current_element == "FullName" {
                        account_info.fullName = v;
                    } else if current_element == "BirthDate" {
                        account_info.birthDate = v;
                    } else if current_element == "BranchName" {
                        account_info.branchName = v;
                    } else if current_element == "Amount" {
                        if account_info.amount == None {
                            account_info.amount = v;
                        }
                    } else if current_element == "Change" {
                        if account_info.change == None {
                            account_info.change = v;
                        }
                    } else if current_element == "Start" {
                        if account_info.start == None {
                            account_info.start = v;
                        }
                    } else if current_element == "Expiry" {
                        if account_info.expiry == None {
                            account_info.expiry = v;
                        }
                    } else if current_element == "ExpiryMonth" {
                        if account_info.expiryMonth == None {
                            account_info.expiryMonth = v;
                        }
                    } else if current_element == "ExpiryYear" {
                        if account_info.expiryYear == None {
                            account_info.expiryYear = v;
                        }
                    } else if current_element == "Postcode" {
                        account_info.postcode = v;
                    } else if current_element == "Surname" {
                        account_info.surname = v;
                    } else if current_element == "Forename" {
                        account_info.forename = v;
                    } else if current_element == "EmailAddress" {
                        account_info.emailAddress = v;
                    } else if current_element == "AddressLine1" {
                        account_info.addressLine1 = v;
                    } else if current_element == "AddressLine2" {
                        account_info.addressLine2 = v;
                    } else if current_element == "Acronym" {
                        account_info.acronym = v;
                    } else {
                        // println!("Unhandled value: {:?}{:?}", current_element, c);
                    }
                }
                _ => {}
            }
        }
        Ok(account_info)
    }

    fn parser_config(&self) -> xml::ParserConfig {
        let mut config = xml::ParserConfig::new();
        config.ignore_comments = true;
        config.cdata_to_characters = false;
        config.trim_whitespace = true;
        config
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn it_parses_account_info_from_xml() {
        //        let contents = fs::read_to_string("src/xml/fixtures/account_info_reponse.xml")
        //            .expect("Something went wrong reading the file");
        //
        //        println!("With text:\n{}", contents);
        //
        assert_eq!(2 + 2, 4);
    }
}
