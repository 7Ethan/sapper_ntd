use response_special::err_400;
use sapper::Key;
use sapper::Params;
use sapper::PathParams;
use sapper::Request;
use sapper::Result;
use sapper_body::BodyMap;
use sapper_body::FormParams;
use sapper_body::JsonParams;
use sapper_query::QueryMap;
use sapper_query::QueryParams;
use serde::de::DeserializeOwned;
use serde_json::from_value;
use std::borrow::Borrow;
use std::fmt::Display;
use std::hash::Hash;
use utils;
use utils::missing_or_unrecognized;

pub fn get_params<'a, T: Key>(req: &'a Request) -> Result<&'a T::Value> {
    req.ext().get::<T>().ok_or(utils::ok_to_err(err_400("no params")))
}

pub fn get_path_params<'a>(req: &'a Request) -> Result<&'a Params> {
    get_params::<PathParams>(req)
}

pub fn get_query_params<'a>(req: &'a Request) -> Result<&'a QueryMap> {
    get_params::<QueryParams>(req)
}

pub fn get_form_params<'a>(req: &'a Request) -> Result<&'a BodyMap> {
    get_params::<FormParams>(req)
}

pub fn get_path_param_value<'a>(req: &'a Request, key: &str) -> Result<&'a str> {
    let params = get_path_params(req)?;
    let value = params.get(key).ok_or_else(|| missing_or_unrecognized(key).unwrap_err())?;
    Ok(value[0])
}

pub fn get_query_param_value<'a, Q: ?Sized>(req: &'a Request, key: &Q) -> Result<&'a str> where String: Borrow<Q>, Q: Hash + Eq + Display {
    let params = get_query_params(req)?;
    let value = params.get(key).ok_or_else(|| missing_or_unrecognized(key).unwrap_err())?;
    Ok(&value[0])
}

pub fn get_form_param_value<'a, Q: ?Sized>(req: &'a Request, key: &Q) -> Result<&'a str> where String: Borrow<Q>, Q: Hash + Eq + Display {
    let params = get_form_params(req)?;
    let value = params.get(key).ok_or_else(|| missing_or_unrecognized(key).unwrap_err())?;
    Ok(&value[0])
}

pub fn get_json_params<T: DeserializeOwned>(req: &Request) -> Result<T> {
    let json_value = get_params::<JsonParams>(req)?;
    from_value::<T>(json_value.clone())
        .or(utils::ok_to_custom_res_err(err_400("Json parameter not match to struct.")))
}