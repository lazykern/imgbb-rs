use crate::Error;
use crate::Response;

const URL: &str = "https://api.imgbb.com/1/upload";

/// An struct that holds the data (base64) to be uploaded
pub struct Uploader {
    /// ImgBB API key
    pub api_key: String,
    /// Base64 data to be uploaded
    pub data: Option<String>,
    /// Expiration time in seconds
    pub expiration: Option<u64>,
}

impl Uploader {
    /// Creates a new Uploader struct with the given API key
    pub fn new<T>(api_key: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            api_key: api_key.into(),
            data: None,
            expiration: None,
        }
    }

    /// Set [expiration time](Uploader::expiration)
    pub fn expiration(&mut self, expiration: u64) -> &Self {
        self.expiration = Some(expiration);
        self
    }

    /// Upload [data](Uploader::data) to ImgBB
    pub async fn upload(&self) -> Result<Response, Error> {
        let mut query = vec![("key", self.api_key.as_str())];

        let exp_str = self.expiration.as_ref().unwrap_or(&0).to_string();
        if self.expiration.is_some() {
            query.push(("expiration", exp_str.as_str()));
        }

        let form = [("image", self.data.as_ref().unwrap().as_str())];

        let res = reqwest::Client::new()
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
}
