use response_builder::default_builder;
use sapper::header::ContentType;
use sapper::Response;
use sapper::Result;
use sapper_tmpl::Context;
use sapper_tmpl::render;

pub fn res_html<T: AsRef<str>>(html: T, context: Context) -> Result<Response> {
    let res_str = render(html.as_ref(), context);
    default_builder()
        .set_header(ContentType::html())
        .write_body(res_str)
        .build_ok()
}