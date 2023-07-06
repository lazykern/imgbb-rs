use base64::engine::{general_purpose, Engine};
use std::path::Path;

/// Module for ImgBB API error
pub mod error;
pub use error::Error;

/// Module for ImgBB API response model
pub mod model;
use model::*;

/// Module for ImgBB uploader
pub mod uploader;
use uploader::*;

/// Main struct for this crate
#[derive(Debug)]
pub struct ImgBB {
    client: reqwest::Client,
    api_key: String,
}

impl ImgBB {
    /// Creates a new ImgBB client with the given API key
    pub fn new<T>(api_key: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
        }
    }

    pub fn new_with_client<T>(api_key: T, client: reqwest::Client) -> Self
    where
        T: Into<String>,
    {
        Self {
            client,
            api_key: api_key.into(),
        }
    }

    /// Read base64 data and return an [Uploader](Uploader) struct to upload in the next step
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

    /// Read bytes data and return an [Uploader](Uploader) struct to upload in the next step
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

    /// Read file from path and return an [Uploader](Uploader) struct to upload in the next step
    pub fn read_file<P>(&self, path: P) -> Result<Uploader, Error>
    where
        P: AsRef<Path>,
    {
        let f = std::fs::read(path)?;
        let d = Some(general_purpose::STANDARD.encode(f));

        Ok(Uploader {
            api_key: self.api_key.clone(),
            data: d,
            expiration: None,
        })
    }

    /// Delete an image from ImgBB using the given [delete URL](Data::delete_url) in [Response](Response)::[Data](Data)
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

    /// Straightforward upload base64 data to ImgBB
    pub async fn upload_base64<T>(&self, data: T) -> Result<Response, Error>
    where
        T: AsRef<str>,
    {
        self.read_base64(data).upload().await
    }

    /// Straightforward upload bytes data to ImgBB
    pub async fn upload_bytes<T>(&self, data: T) -> Result<Response, Error>
    where
        T: AsRef<[u8]>,
    {
        self.read_bytes(data).upload().await
    }

    /// Straightforward upload file to ImgBB
    pub async fn upload_file<P>(&self, path: P) -> Result<Response, Error>
    where
        P: AsRef<Path>,
    {
        self.read_file(path)?.upload().await
    }

    /// Upload base64 data to ImgBB with expiration time (seconds)
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

    /// Upload bytes data to ImgBB with expiration time (seconds)
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

    /// Upload file to ImgBB with expiration time (seconds)
    pub async fn upload_file_with_expiration<P>(
        &self,
        path: P,
        expiration: u64,
    ) -> Result<Response, Error>
    where
        P: AsRef<Path>,
    {
        self.read_file(path)?.expiration(expiration).upload().await
    }
}
