use crate::resource::Resource;
use crate::model::SessionToken;
use crate::xml::AuthXmlParser;

use std::io::Read;

pub struct AuthenticationResource {
    pub username: String,
    pub password: String
}

impl Resource<SessionToken> for AuthenticationResource {

    fn parse(&self, bytes: impl Read) -> Result<SessionToken, &'static str> {
        let parser = AuthXmlParser::new();
        parser.session_token_from_xml(bytes)
    }

    /// Generates the session token request body for the given username and password
    fn request_body(&self) -> String {
        format!(
            r#"<?xml version='1.0' encoding='utf-8'?>
        <soap12:Envelope xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance' xmlns:xsd='http://www.w3.org/2001/XMLSchema' xmlns:soap12='http://www.w3.org/2003/05/soap-envelope'>
            <soap12:Body>
                <CheckBorrower xmlns='http://bibliomondo.com/websevices/webuser'>
                <borrowerNumber>{}</borrowerNumber>
                <pin>{}</pin></CheckBorrower>
            </soap12:Body>
        </soap12:Envelope>"#,
        self.username, self.password
        )
    }
}
