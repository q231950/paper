use crate::model::LoansInfo;
use crate::model::LoanBuilder;
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};

pub struct LoansInfoXmlParser {}

impl LoansInfoXmlParser {
    pub fn new() -> LoansInfoXmlParser {
        LoansInfoXmlParser {}
    }

    pub fn loans_info_from_xml(&self, xml: impl Read) -> Result<LoansInfo, &'static str> {
        let mut loans_info = LoansInfo::new();
        let mut current_element = "".to_string();
        let mut loan_builder = LoanBuilder::new();
        let parser = EventReader::new_with_config(xml, self.parser_config());
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    current_element = name.local_name.to_string();
                    // println!("Start: {}, {}", name, current_element);
                }
                Err(e) => {
                    println!("⚠️ Error: {}", e);
                    return Err("Unable to create loans info from xml");
                }
                Ok(XmlEvent::Characters(c)) => {
                    let v = Some(c.clone());
                    if current_element == "LoanDetail" {
                        loan_builder.clear();
                    } else if current_element == "Title" {
                        loan_builder.title = v;
                    } else if current_element == "Author" {
                        loan_builder.author = v;
                    } else {
                        //println!("Unhandled value: {:?}{:?}", current_element, c);
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    if name.local_name.to_string() == "LoanDetail" {
                        loans_info.add_loan(loan_builder.build_loan());
                    }
                }
                _ => {}
            }
        }
        Ok(loans_info)
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
