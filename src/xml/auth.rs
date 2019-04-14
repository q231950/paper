extern crate xml;

use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

pub struct AuthXmlParser {}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

impl AuthXmlParser {
    pub fn new() -> AuthXmlParser {
        AuthXmlParser {}
    }

    pub fn session_token_from_xml(&self, xml: impl Read) -> Result<String, &'static str> {
        println!("Getting session token from xml...");

        let mut current_element = "".to_string();
        let parser = EventReader::new(xml);
        let mut depth = 0;
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    current_element = name.to_string();
                    depth += 1;
                }
                Ok(XmlEvent::EndElement { name }) => {
                    depth -= 1;
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
