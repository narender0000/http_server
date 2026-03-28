use crate::response::{HttpCode, Response};
use anyhow::{Ok, Result, bail};

pub fn echo(path_params: Option<&str>) -> Result<Response> {
    let Some(param) = path_params else {
        bail!("missing path param");
    };
    let resp = Response {
        code: HttpCode::Ok,
        body: Some(param.to_owned()),
    };
    Ok(resp)
}
