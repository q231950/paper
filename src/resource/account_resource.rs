use crate::auth::SessionToken;
use crate::model::AccountInfo;
use crate::resource::Resource;
use crate::xml::AccountInfoXmlParser;

use std::io::Read;

pub struct AccountInfoResource {
}

impl Resource<AccountInfo> for AccountInfoResource {

    fn parse(&self, bytes: impl Read) -> Result<AccountInfo, &'static str> {
        let parser = AccountInfoXmlParser::new();
        parser.account_info_from_xml(bytes)
    }


    fn request_body(&self, token: &SessionToken) -> String {
        format!(
            r#"<?xml version='1.0' encoding='utf-8'?>
        <soap12:Envelope xmlns:xsi='http://www.w3.org/2001/XMLSchema-instance'
    xmlns:xsd='http://www.w3.org/2001/XMLSchema'
    xmlns:soap12='http://www.w3.org/2003/05/soap-envelope'>
            <soap12:Body>
                <GetBorrowerSummary xmlns='http://bibliomondo.com/websevices/webuser'>
                    <sessionId>{}</sessionId>
                </GetBorrowerSummary>
            </soap12:Body>
        </soap12:Envelope>"#,
        token
        )
    }

}
