use crate::model::AccountInfo;
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

pub struct AccountInfoXmlParser {}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

impl AccountInfoXmlParser {
    pub fn new() -> AccountInfoXmlParser {
        AccountInfoXmlParser {}
    }

    pub fn account_info_from_xml(&self, xml: impl Read) -> Result<AccountInfo, &'static str> {
        println!("Creating account info from xml...");

        let mut result: Result<AccountInfo, &'static str>;
        let mut account_info = AccountInfo::new();
        result = Ok(account_info);
        let mut current_element = "".to_string();
        let parser = EventReader::new(xml);
        let mut depth = 0;
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    current_element = name.to_string();
                    //println!("{}+{}+{:?}", indent(depth), name, attributes);
                    depth += 1;
                }
                Ok(XmlEvent::EndElement { name }) => {
                    depth -= 1;
                    //println!("{}-{}", indent(depth), name);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    result = Err("Unable to create account info from xml");
                }
                Ok(XmlEvent::Characters(c)) => {
                    println!("{:?}", current_element);
                    if current_element
                        == "{http://bibliomondo.com/webservices/webuser}ReadableFullName"
                    {
                        println!("{}{:?}", indent(depth), c);
                        let mut account_info = AccountInfo::new();
                        account_info.firstname = c.clone();
                        result = Ok(account_info);
                    }
                }
                _ => {}
            }
        }
        result
    }
}
