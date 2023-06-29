use base64::engine::{general_purpose, Engine};
use std::path::Path;
use tokio::fs;

pub mod error;
pub use error::Error;

pub mod model;
use model::*;

pub mod uploader;
use uploader::*;

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

    pub fn read_base64<T>(&self, data: T) -> Uploader
    where
        T: AsRef<str>,
    {
        Uploader {
            api_key: self.api_key.clone(),
            data: Some(data.as_ref().to_string()),
            expiration: None,
        }
    }

    pub fn read_bytes<T>(&self, data: T) -> Uploader
    where
        T: AsRef<[u8]>,
    {
        let d = general_purpose::STANDARD.encode(data.as_ref());
        Uploader {
            api_key: self.api_key.clone(),
            data: Some(d),
            expiration: None,
        }
    }

    pub async fn read_file<P>(&self, path: P) -> Result<Uploader, Error>
    where
        P: AsRef<Path>,
    {
        let f = fs::read(path).await?;
        let d = Some(general_purpose::STANDARD.encode(f));

        Ok(Uploader {
            api_key: self.api_key.clone(),
            data: d,
            expiration: None,
        })
    }

    pub async fn delete<T>(&self, delete_url: T) -> Result<(), Error>
    where
        T: Into<String>,
    {
        let query = [("key", self.api_key.as_str())];

        self.client
            .delete(&delete_url.into())
            .query(&query)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn upload_base64<T>(&self, data: T) -> Result<Response, Error>
    where
        T: AsRef<str>,
    {
        self.read_base64(data).upload().await
    }

    pub async fn upload_bytes<T>(&self, data: T) -> Result<Response, Error>
    where
        T: AsRef<[u8]>,
    {
        self.read_bytes(data).upload().await
    }

    pub async fn upload_file<P>(&self, path: P) -> Result<Response, Error>
    where
        P: AsRef<Path>,
    {
        self.read_file(path).await?.upload().await
    }

    pub async fn upload_base64_with_expiration<T>(
        &self,
        data: T,
        expiration: u64,
    ) -> Result<Response, Error>
    where
        T: AsRef<str>,
    {
        self.read_base64(data).expiration(expiration).upload().await
    }

    pub async fn upload_bytes_with_expiration<T>(
        &self,
        data: T,
        expiration: u64,
    ) -> Result<Response, Error>
    where
        T: AsRef<[u8]>,
    {
        self.read_bytes(data).expiration(expiration).upload().await
    }

    pub async fn upload_file_with_expiration<P>(
        &self,
        path: P,
        expiration: u64,
    ) -> Result<Response, Error>
    where
        P: AsRef<Path>,
    {
        self.read_file(path)
            .await?
            .expiration(expiration)
            .upload()
            .await
    }
}
