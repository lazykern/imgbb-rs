use serde::Deserialize;

/// Response from the ImgBB API
/// 
/// The API returns a JSON structure that includes the upload data,
/// as well as status and success information.
#[derive(Debug, Deserialize, Clone)]
pub struct Response {
    /// The image data information if the upload was successful
    pub data: Option<Data>,
    /// Boolean indicating if the request was successful
    pub success: Option<bool>,
    /// HTTP status code
    pub status: Option<u16>,
    /// Error message if the request failed
    pub error: Option<ErrorResponse>,
}

/// Error information returned by the ImgBB API when a request fails
#[derive(Debug, Deserialize, Clone)]
pub struct ErrorResponse {
    /// Error message
    pub message: Option<String>,
    /// Error code
    pub code: Option<u16>,
}

/// Detailed information about an uploaded image
#[derive(Debug, Deserialize, Clone)]
pub struct Data {
    /// Unique ID of the uploaded image
    pub id: Option<String>,
    /// Title of the image (if provided during upload)
    pub title: Option<String>,
    /// URL to view the image on ImgBB website
    pub url_viewer: Option<String>,
    /// Direct URL to the image
    pub url: Option<String>,
    /// Display URL (typically used in HTML)
    pub display_url: Option<String>,
    /// Width of the image in pixels
    pub width: Option<u16>,
    /// Height of the image in pixels
    pub height: Option<u16>,
    /// Size of the image in bytes
    pub size: Option<u32>,
    /// Unix timestamp of when the image was uploaded
    pub time: Option<u64>,
    /// Expiration time in seconds, if set
    pub expiration: Option<u64>,
    /// Full-size image information
    pub image: Option<Image>,
    /// Thumbnail image information
    pub thumb: Option<Image>,
    /// Medium-size image information
    pub medium: Option<Image>,
    /// URL to delete the image
    pub delete_url: Option<String>,
}

/// Information about a specific image variant (original, thumbnail, etc.)
#[derive(Debug, Deserialize, Clone)]
pub struct Image {
    /// Original filename
    pub filename: Option<String>,
    /// Name of the image
    pub name: Option<String>,
    /// MIME type (e.g., "image/jpeg")
    pub mime: Option<String>,
    /// File extension (e.g., "jpg")
    pub extension: Option<String>,
    /// Direct URL to this image variant
    pub url: Option<String>,
}
