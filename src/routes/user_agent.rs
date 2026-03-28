use crate::{
    request::Request,
    response::{HttpCode, Response},
};
use anyhow::Result;
pub fn user_agent(request: &Request) -> Result<Response> {
    let user_agent = request.headers.get("user-agent").map(ToOwned::to_owned);
    Ok(Response {
        code: HttpCode::Ok,
        body: user_agent,
    })
}
