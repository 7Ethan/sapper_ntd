use sapper::Error;
use sapper::header::Header;
use sapper::header::HeaderFormat;
use sapper::Response;
use sapper::Result;
use sapper::status::Status;
use std::borrow::Cow;
use std::fmt::Display;

#[derive(Debug)]
pub struct ResponseBuilder {
    response: Response
}

pub fn default_builder() -> ResponseBuilder {
    ResponseBuilder::default()
}

impl Default for ResponseBuilder {
    fn default() -> Self {
        ResponseBuilder {
            response: Response::new()
        }
    }
}

impl ResponseBuilder {
    pub fn set_header<T: Header + HeaderFormat>(mut self, value: T) -> Self {
        self.response.headers_mut().set(value);
        self
    }

    pub fn set_raw_header<K: Into<Cow<'static, str>>>(mut self, name: K,
                                                      value: Vec<Vec<u8>>) -> Self {
        self.response.headers_mut().set_raw(name, value);
        self
    }

    pub fn set_status(mut self, status: Status) -> Self {
        self.response.set_status(status);
        self
    }

    pub fn write_body<T: Display>(mut self, body: T) -> Self {
        self.response.write_body(format!("{}", body));
        self
    }

    pub fn write_raw_body(mut self, body: Vec<u8>) -> Self {
        self.response.write_raw_body(body);
        self
    }

    pub fn build(self) -> Response {
        self.response
    }

    pub fn build_ok(self) -> Result<Response> {
        Ok(self.build())
    }

    pub fn build_error<T>(self) -> Result<T> {
        Err(Error::CustomResponse(self.build()))
    }
}

