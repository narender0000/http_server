use anyhow::{Context, bail};
#[derive(Debug)]
pub enum Method {
    Get,
}

impl TryFrom<Vec<u8>> for Method {
    type Error = anyhow::Error;
    fn try_from(value: Vec<u8>) -> std::prelude::v1::Result<Self, Self::Error> {
        let method_string =
            String::from_utf8(value).context("converting bytes to method string")?;
        Ok(match method_string.to_uppercase().trim() {
            "GET" => Self::Get,
            _ => bail!("Unkown Method"),
        })
    }
}
