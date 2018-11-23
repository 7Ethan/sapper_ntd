use response_builder::default_builder;
use sapper::header::Location;
use sapper::Response;
use sapper::Result;
use sapper::status;
use std::fmt::Display;

pub fn res_redirect<T: AsRef<str>>(redirect_uri: T) -> Result<Response> {
    default_builder()
        .set_status(status::Found)
        .set_header(Location(redirect_uri.as_ref().to_owned()))
        .write_body(format!("redirect to {}", redirect_uri.as_ref().to_owned()))
        .build_ok()
}


pub fn err_400<T: Display>(info: T) -> Result<Response> {
    default_builder()
        .set_status(status::BadRequest)
        .write_body(info)
        .build_error()
}

pub fn err_500<T: Display>(info: T) -> Result<Response> {
    default_builder()
        .set_status(status::InternalServerError)
        .write_body(info)
        .build_error()
}