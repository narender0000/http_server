use crate::response::{HttpCode, Response};
use anyhow::Result;
pub fn home() -> Result<Response> {
    Ok(Response {
        code: HttpCode::Ok,
        body: None,
    })
}
