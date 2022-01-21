use crate::auth::SessionToken;
use crate::xml::LoansInfoXmlParser;
use crate::model::LoansInfo;
use std::io::Read;

pub trait Resource<ResultType> {

    fn token(&self) -> &SessionToken;

    fn parse(&self, bytes: impl Read) -> Result<ResultType, &'static str>;
    
    fn request_body(&self) -> String;
}

pub struct LoansInfoResource {
    pub session_token: SessionToken
}

impl Resource<LoansInfo> for LoansInfoResource {

    fn token(&self) -> &SessionToken {
        &self.session_token
    }

    fn parse(&self, bytes: impl Read) -> Result<LoansInfo, &'static str> {
        let parser = LoansInfoXmlParser::new();
        parser.loans_info_from_xml(bytes)
    }


    fn request_body(&self) -> String {
        format!(
            r#"<?xml version='1.0' encoding='utf-8'?>
        <soap12:Envelope xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance'
    xmlns:xsd='http://www.w3.org/2001/XMLSchema'
    xmlns:soap12='http://www.w3.org/2003/05/soap-envelope'>
            <soap12:Body>
                <GetBorrowerLoans xmlns='http://bibliomondo.com/websevices/webuser'>
                    <sessionId>{}</sessionId>
                </GetBorrowerLoans>
            </soap12:Body>
        </soap12:Envelope>"#,
        self.session_token
        )
    }
}
