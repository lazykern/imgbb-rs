use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),
    
    #[error("Reqwest Error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    
    #[error("ImgBB API Error: {message}")]
    ApiError {
        message: String,
        status: Option<u16>,
        code: Option<u16>,
    },
    
    #[error("Missing field '{0}' in API response")]
    MissingField(String),
    
    #[error("Invalid API key")]
    InvalidApiKey,
    
    #[error("Invalid base64 data")]
    InvalidBase64Data,
    
    #[error("Image too large")]
    ImageTooLarge,
    
    #[error("Unsupported image format")]
    UnsupportedFormat,
    
    #[error("Request timeout")]
    Timeout,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Invalid or missing parameters: {0}")]
    InvalidParameters(String),
}
