use base64::engine::{general_purpose, Engine};
use std::path::Path;
use tokio::fs;

pub mod error;
pub use error::Error;

pub mod model;
use model::*;

const URL: &str = "https://api.imgbb.com/1/upload";

#[derive(Debug)]
pub struct ImgBB {
    client: reqwest::Client,
    api_key: String,
}

impl ImgBB {
    pub fn new<T>(api_key: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
        }
    }

    pub async fn upload_base64<T>(&self, data: T) -> Result<Response, Error>
    where
        T: Into<String>,
    {
        let form = [("image", data.into())];
        let query = [("key", &self.api_key)];

        let res = self
            .client
            .post(URL)
            .query(&query)
            .form(&form)
            .send()
            .await?
            .error_for_status()?
            .json::<Response>()
            .await?;

        Ok(res)
    }

    pub async fn upload_bytes<T>(&self, data: T) -> Result<Response, Error>
    where
        T: AsRef<[u8]>,
    {
        let d = general_purpose::STANDARD.encode(data.as_ref());
        self.upload_base64(d).await
    }

    pub async fn upload_file<P>(&self, path: P) -> Result<Response, Error>
    where
        P: AsRef<Path>,
    {
        let f = fs::read(path).await?;
        self.upload_bytes(f).await
    }
}
