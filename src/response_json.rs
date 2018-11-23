use response_builder::default_builder;
use sapper::{Response, Result};
use sapper::header::ContentType;
use serde::Serialize;
use serde_json;

pub fn res_json<T: Serialize>(json: &T) -> Result<Response> {
    default_builder()
        .set_header(ContentType::json())
        .write_body(serde_json::to_string(json).unwrap())
        .build_ok()
}

pub fn res_json_ok<T: Serialize>(info: T) -> Result<Response> {
    let ref json2ret = json!({
            "success": true,
            "info": info
    });
    res_json(json2ret)
}

pub fn res_json_error<T: Serialize>(info: T) -> Result<Response> {
    let ref json2ret = json!({
            "success": false,
            "info": info
    });
    res_json(json2ret)
}