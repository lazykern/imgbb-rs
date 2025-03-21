use base64::engine::{general_purpose, Engine};
use std::path::Path;
use std::time::Duration;

/// Module for ImgBB API error
pub mod error;
pub use error::Error;

/// Module for ImgBB API response model
pub mod model;
use model::*;

/// Module for ImgBB uploader
pub mod uploader;
use uploader::*;

/// Main client for interacting with the ImgBB API
///
/// The `ImgBB` struct provides methods for uploading and deleting images
/// from the ImgBB service.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust,no_run
/// use imgbb::ImgBB;
///
/// async fn example() -> Result<(), imgbb::Error> {
///     // Create a new ImgBB client
///     let imgbb = ImgBB::new("your_api_key");
///
///     // Upload an image file
///     let response = imgbb.upload_file("path/to/image.jpg").await?;
///
///     // Print the image URL
///     println!("Image URL: {}", response.data.unwrap().url.unwrap());
///
///     Ok(())
/// }
#[derive(Debug)]
pub struct ImgBB {
    client: reqwest::Client,
    api_key: String,
}

/// Builder for creating a customized ImgBB client
///
/// This builder allows you to customize the ImgBB client with options
/// such as timeout duration and user agent.
///
/// # Examples
///
/// ```rust,no_run
/// use imgbb::ImgBB;
/// use std::time::Duration;
///
/// // Create a client with a 30-second timeout and custom user agent
/// let imgbb = ImgBB::builder("your_api_key")
///     .timeout(Duration::from_secs(30))
///     .user_agent("MyApp/1.0")
///     .build()
///     .unwrap();
/// ```
#[derive(Debug)]
pub struct ImgBBBuilder {
    api_key: String,
    timeout: Option<Duration>,
    user_agent: Option<String>,
}

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

impl ImgBB {
    /// Creates a new ImgBB client with the given API key
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use imgbb::ImgBB;
    ///
    /// let imgbb = ImgBB::new("your_api_key");
    /// ```
    pub fn new<T>(api_key: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            client: reqwest::Client::builder()
                .user_agent(APP_USER_AGENT)
                .build()
                .unwrap(),
            api_key: api_key.into(),
        }
    }

    /// Creates a new builder for a customized ImgBB client
    ///
    /// Use this method to create a builder for configuring the ImgBB client
    /// with custom options like timeout and user agent.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use imgbb::ImgBB;
    /// use std::time::Duration;
    ///
    /// let imgbb = ImgBB::builder("your_api_key")
    ///     .timeout(Duration::from_secs(15))
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder<T>(api_key: T) -> ImgBBBuilder
    where
        T: Into<String>,
    {
        ImgBBBuilder {
            api_key: api_key.into(),
            timeout: None,
            user_agent: None,
        }
    }

    /// Creates a new ImgBB client with the given API key and reqwest client
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
            client: &self.client,
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
            client: &self.client,
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
            client: &self.client,
        })
    }

    /// Create a new uploader with custom options
    pub fn upload_builder(&self) -> UploaderBuilder {
        UploaderBuilder {
            api_key: self.api_key.clone(),
            data: None,
            expiration: None,
            name: None,
            title: None,
            album: None,
            client: &self.client,
        }
    }

    /// Delete an image from ImgBB using the given delete URL
    ///
    /// This method deletes an image from ImgBB using the delete URL that was
    /// provided in the upload response.
    ///
    /// # Arguments
    ///
    /// * `delete_url` - The delete URL for the image from the original upload response
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use imgbb::ImgBB;
    ///
    /// async fn example() -> Result<(), imgbb::Error> {
    ///     let imgbb = ImgBB::new("your_api_key");
    ///
    ///     // The delete URL from the upload response
    ///     let delete_url = "https://ibb.co/abc123/delete_hash";
    ///
    ///     // Delete the image
    ///     imgbb.delete(delete_url).await?;
    ///     println!("Image deleted successfully");
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails, if the API returns an error response,
    /// or if the API key is invalid.
    pub async fn delete<T>(&self, delete_url: T) -> Result<(), Error>
    where
        T: Into<String>,
    {
        let query = [("key", self.api_key.as_str())];

        let res = self.client
            .delete(&delete_url.into())
            .query(&query)
            .send()
            .await?;

        let status = res.status().as_u16();
        
        // Try to parse as Response first to handle API errors
        if let Ok(response) = res.json::<Response>().await {
            if let Some(error) = response.error {
                let error_code = error.code.unwrap_or(0);
                let error_message = error.message.unwrap_or_else(|| "Unknown error".to_string());
                
                return match error_code {
                    100 => Err(Error::InvalidApiKey),
                    400 => Err(Error::InvalidParameters(error_message)),
                    429 => Err(Error::RateLimitExceeded),
                    _ => Err(Error::ApiError {
                        message: error_message,
                        status: Some(status),
                        code: Some(error_code),
                    }),
                };
            }

            // If we successfully parsed the response but got a non-2xx status code
            if !response.success.unwrap_or(false) {
                return Err(Error::ApiError {
                    message: "Delete failed without specific error".to_string(),
                    status: Some(status),
                    code: None,
                });
            }

            return Ok(());
        }

        // If we couldn't parse the response and got a non-2xx status code
        if status >= 400 {
            return Err(Error::ApiError {
                message: format!("Delete failed with status code {}", status),
                status: Some(status),
                code: None,
            });
        }

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
        let mut uploader = self.read_base64(data);
        uploader.expiration(expiration);
        uploader.upload().await
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
        let mut uploader = self.read_bytes(data);
        uploader.expiration(expiration);
        uploader.upload().await
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
        let mut uploader = self.read_file(path)?;
        uploader.expiration(expiration);
        uploader.upload().await
    }
}

impl ImgBBBuilder {
    /// Set a custom timeout for all requests
    ///
    /// # Arguments
    ///
    /// * `timeout` - The timeout duration for requests
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use imgbb::ImgBB;
    /// use std::time::Duration;
    ///
    /// let imgbb = ImgBB::builder("your_api_key")
    ///     .timeout(Duration::from_secs(10))
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set a custom user agent
    ///
    /// # Arguments
    ///
    /// * `user_agent` - The user agent string to use for requests
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use imgbb::ImgBB;
    ///
    /// let imgbb = ImgBB::builder("your_api_key")
    ///     .user_agent("MyApp/1.0")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn user_agent<T>(mut self, user_agent: T) -> Self
    where
        T: Into<String>,
    {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Build the ImgBB client
    ///
    /// This method builds the ImgBB client with the configured options.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use imgbb::ImgBB;
    ///
    /// let imgbb = ImgBB::builder("your_api_key")
    ///     .build()
    ///     .unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the reqwest client builder fails to build.
    pub fn build(self) -> Result<ImgBB, Error> {
        let mut client_builder = reqwest::Client::builder();

        // Set user agent
        client_builder = client_builder.user_agent(
            self.user_agent.unwrap_or_else(|| APP_USER_AGENT.to_string()),
        );

        // Set timeout if provided
        if let Some(timeout) = self.timeout {
            client_builder = client_builder.timeout(timeout);
        }

        // Build the client
        let client = client_builder
            .build()
            .map_err(Error::from)?;

        Ok(ImgBB {
            client,
            api_key: self.api_key,
        })
    }
}

/// A builder for creating an uploader with more options
///
/// This builder allows for more flexible configuration of image uploads,
/// including setting the name, title, and album for the image.
///
/// # Examples
///
/// ```rust,no_run
/// use imgbb::ImgBB;
///
/// async fn example() -> Result<(), imgbb::Error> {
///     let imgbb = ImgBB::new("your_api_key");
///
///     // Create an uploader with custom options
///     let mut uploader = imgbb.upload_builder();
///     let response = uploader
///         .file("path/to/image.jpg")?
///         .name("custom_name")
///         .title("My Image")
///         .expiration(86400) // 24 hours
///         .album("album_id")
///         .upload()
///         .await?;
///
///     println!("Upload successful: {}", response.data.unwrap().url.unwrap());
///
///     Ok(())
/// }
/// ```
pub struct UploaderBuilder<'a> {
    api_key: String,
    data: Option<String>,
    expiration: Option<u64>,
    name: Option<String>,
    title: Option<String>,
    album: Option<String>,
    client: &'a reqwest::Client,
}

impl<'a> UploaderBuilder<'a> {
    /// Set the base64 data
    pub fn data<T>(&mut self, data: T) -> &mut Self
    where
        T: AsRef<str>,
    {
        self.data = Some(data.as_ref().to_string());
        self
    }

    /// Set the raw bytes data, which will be encoded as base64
    pub fn bytes<T>(&mut self, data: T) -> &mut Self
    where
        T: AsRef<[u8]>,
    {
        self.data = Some(general_purpose::STANDARD.encode(data.as_ref()));
        self
    }

    /// Set data from a file path
    pub fn file<P>(&mut self, path: P) -> Result<&mut Self, Error>
    where
        P: AsRef<Path>,
    {
        let f = std::fs::read(path)?;
        self.data = Some(general_purpose::STANDARD.encode(f));
        Ok(self)
    }

    /// Set the expiration time in seconds
    pub fn expiration(&mut self, expiration: u64) -> &mut Self {
        self.expiration = Some(expiration);
        self
    }

    /// Set the image name
    pub fn name<T>(&mut self, name: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    /// Set the image title
    pub fn title<T>(&mut self, title: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.title = Some(title.into());
        self
    }

    /// Set the album ID
    pub fn album<T>(&mut self, album: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.album = Some(album.into());
        self
    }

    /// Upload the image with all specified options
    pub async fn upload(&self) -> Result<Response, Error> {
        if self.data.is_none() {
            return Err(Error::MissingField("data".to_string()));
        }

        let mut query = vec![("key", self.api_key.as_str())];
        let mut form = vec![("image", self.data.as_ref().unwrap().as_str())];

        // Handle expiration separately to avoid temporary value issues
        let expiration_str;
        if let Some(exp) = &self.expiration {
            expiration_str = exp.to_string();
            query.push(("expiration", expiration_str.as_str()));
        }

        if let Some(name) = &self.name {
            form.push(("name", name.as_str()));
        }

        if let Some(title) = &self.title {
            form.push(("title", title.as_str()));
        }

        if let Some(album) = &self.album {
            form.push(("album", album.as_str()));
        }

        let res = self.client
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

const URL: &str = "https://api.imgbb.com/1/upload";
