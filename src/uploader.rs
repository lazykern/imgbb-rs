use crate::Error;
use crate::Response;

const URL: &str = "https://api.imgbb.com/1/upload";

pub struct Uploader {
    pub api_key: String,
    pub data: Option<String>,
    pub expiration: Option<u64>,
}
impl Uploader {
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

    pub fn expiration(&mut self, expiration: u64) -> &Self {
        self.expiration = Some(expiration);
        self
    }

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
