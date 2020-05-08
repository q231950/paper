extern crate xml;

use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

pub struct AuthXmlParser {}

impl AuthXmlParser {
    pub fn new() -> AuthXmlParser {
        AuthXmlParser {}
    }

    pub fn session_token_from_xml(&self, xml: impl Read) -> Result<String, &'static str> {
        let mut current_element = "".to_string();
        let parser = EventReader::new(xml);
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    current_element = name.to_string();
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                Ok(XmlEvent::Characters(c)) => {
                    if current_element == "sessionId" {
                        return Ok(c);
                    }
                }
                _ => {}
            }
        }
        Err("Unable to find session token in xml")
    }
}
