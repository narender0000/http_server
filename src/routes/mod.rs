mod echo;
mod home;
mod user_agent;
use crate::{
    request::Request,
    response::{HttpCode, Response},
};
use anyhow::{Context, Result, bail};
pub fn router(request: Request) -> Result<Response> {
    let mut segments = request.path.trim_matches('/').split('/');
    let response = match segments.next() {
        Some("") => home::home().context("processing home request")?,
        Some("echo") => echo::echo(segments.next()).context("processing echo handler")?,

        Some("user-agent") => user_agent::user_agent(&request).context("running user agent")?,
        Some(_) => Response {
            code: HttpCode::NotFound,
            body: None,
        },
        None => bail!("Did not get any segements"),
    };

    Ok(response)
}
