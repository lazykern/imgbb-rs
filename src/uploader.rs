use crate::Error;
use crate::Response;

const URL: &str = "https://api.imgbb.com/1/upload";

/// An struct that holds the data (base64) to be uploaded
pub struct Uploader<'a> {
    /// ImgBB API key
    pub api_key: String,
    /// Base64 data to be uploaded
    pub data: Option<String>,
    /// Expiration time in seconds
    pub expiration: Option<u64>,
    /// HTTP client
    pub client: &'a reqwest::Client,
}

impl<'a> Uploader<'a> {
    /// Creates a new Uploader struct with the given API key and client
    pub fn new<T>(api_key: T, client: &'a reqwest::Client) -> Self
    where
        T: Into<String>,
    {
        Self {
            api_key: api_key.into(),
            data: None,
            expiration: None,
            client,
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

        if self.data.is_none() {
            return Err(Error::InvalidParameters("Missing image data".to_string()));
        }

        let form = [("image", self.data.as_ref().unwrap().as_str())];

        let res = self.client
            .post(URL)
            .query(&query)
            .form(&form)
            .send()
            .await?;

        let status = res.status().as_u16();
        let response: Response = res.json().await?;

        if let Some(error) = response.error {
            let error_code = error.code.unwrap_or(0);
            let error_message = error.message.unwrap_or_else(|| "Unknown error".to_string());
            
            return match error_code {
                100 => Err(Error::InvalidApiKey),
                120 => Err(Error::InvalidBase64Data),
                400 => Err(Error::InvalidParameters(error_message)),
                429 => Err(Error::RateLimitExceeded),
                _ => Err(Error::ApiError {
                    message: error_message,
                    status: Some(status),
                    code: Some(error_code),
                }),
            };
        }

        if response.success != Some(true) {
            return Err(Error::ApiError {
                message: "Upload failed without specific error".to_string(),
                status: Some(status),
                code: None,
            });
        }

        Ok(response)
    }
}
