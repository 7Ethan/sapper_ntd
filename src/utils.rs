use response_special::err_400;
use sapper::{Error, Response, Result};
use sapper::Request;
use sapper_body;
use sapper_logger;
use sapper_query;
use sapper_session;
use std::fmt::Display;

pub fn ok_to_custom_res_err<T>(source: Result<Response>) -> Result<T> {
    Err(ok_to_err(source))
}

pub fn ok_to_err(source: Result<Response>) -> Error {
    if source.is_err() {
        source.unwrap_err()
    } else {
        Error::CustomResponse(source.unwrap())
    }
}

pub fn init(req: &mut Request, cookie_key: Option<&'static str>) -> Result<()> {
    sapper_logger::init(req)?;
    sapper_query::parse(req)?;
    sapper_body::parse(req)?;
    sapper_session::session_val(req, cookie_key)?;

    Ok(())
}

pub fn finish(req: &Request, res: &mut Response) -> Result<()> {
    sapper_logger::log(req, res)?;

    Ok(())
}

pub fn missing_or_unrecognized<T: Display>(field: T) -> Result<Response> {
    println!("missing or unrecognized parameter {}.", field);
    err_400(format!("missing or unrecognized parameter {}.", field))
}

pub fn using_default<T: Display>(field: T, default: T) -> T {
    println!("missing or unrecognized parameter {}, using default {}.", field, default);
    default
}
