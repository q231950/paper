use crate::auth::SessionToken;
use std::io::Read;

pub trait Resource<ResultType> {

    fn parse(&self, bytes: impl Read) -> Result<ResultType, &'static str>;

    fn request_body(&self, token: &SessionToken) -> String;
}


