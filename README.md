# imgbb-rs

A comprehensive and flexible [ImgBB](https://imgbb.com/) API client for Rust

[![Crates.io](https://img.shields.io/crates/v/imgbb.svg)](https://crates.io/crates/imgbb)
[![Docs.rs](https://docs.rs/imgbb/badge.svg)](https://docs.rs/imgbb)

## Features

- Upload images using file path, bytes, or base64 encoded strings
- Customize uploads with name, title, expiration time, and album ID
- Delete images
- Robust error handling with specialized error types
- Builder pattern for flexible configuration
- Custom timeout settings
- Custom user agent support
- TLS features options: rustls-tls or native-tls

## Getting Started

1. [Register/Log in to ImgBB](https://imgbb.com/login)
2. [Obtain the API Key](https://api.imgbb.com)
3. Add imgbb to your project dependencies:

```toml
[dependencies]
imgbb = "1.3.0"
```

## Usage Examples

### Simple Upload

```rust
use imgbb::ImgBB;
use tokio;

#[tokio::main]
async fn main() -> Result<(), imgbb::Error> {
    // Initialize with your API key
    let imgbb = ImgBB::new("YOUR_API_KEY");

    // Upload an image file
    let response = imgbb.upload_file("path/to/image.jpg").await?;
    
    // Print the image URL
    println!("Uploaded image URL: {}", response.data.unwrap().url.unwrap());
    
    Ok(())
}
```

### Advanced Upload with Builder Pattern

```rust
use imgbb::ImgBB;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> Result<(), imgbb::Error> {
    // Initialize with custom configuration
    let imgbb = ImgBB::builder("YOUR_API_KEY")
        .timeout(Duration::from_secs(30))
        .user_agent("MyApp/1.0")
        .build()?;
        
    // Create an upload with additional options
    let mut uploader = imgbb.upload_builder();
    let response = uploader
        .file("path/to/image.jpg")?
        .name("my_custom_name")
        .title("My Image Title")
        .expiration(86400) // 24 hours
        .album("album_id")
        .upload()
        .await?;
        
    // Print image details
    let data = response.data.unwrap();
    println!("Image ID: {}", data.id.unwrap());
    println!("Image URL: {}", data.url.unwrap());
    println!("Delete URL: {}", data.delete_url.unwrap());
    
    Ok(())
}
```

### Error Handling

```rust
use imgbb::{ImgBB, Error};
use tokio;

#[tokio::main]
async fn main() {
    let imgbb = ImgBB::new("YOUR_API_KEY");
    
    match imgbb.upload_file("path/to/image.jpg").await {
        Ok(response) => {
            println!("Upload successful!");
            println!("URL: {}", response.data.unwrap().url.unwrap());
        },
        Err(Error::InvalidApiKey) => {
            eprintln!("Your API key is invalid");
        },
        Err(Error::ImageTooLarge) => {
            eprintln!("Image exceeds the maximum size limit");
        },
        Err(Error::Timeout) => {
            eprintln!("Request timed out, please try again");
        },
        Err(e) => {
            eprintln!("Upload failed: {}", e);
        }
    }
}
```

### Deleting Images

```rust
use imgbb::ImgBB;
use tokio;

#[tokio::main]
async fn main() -> Result<(), imgbb::Error> {
    let imgbb = ImgBB::new("YOUR_API_KEY");
    
    // First upload an image
    let response = imgbb.upload_file("path/to/image.jpg").await?;
    
    // Get the delete URL
    let delete_url = response.data.unwrap().delete_url.unwrap();
    println!("Delete URL: {}", delete_url);
    
    // Delete the image
    imgbb.delete(delete_url).await?;
    println!("Image deleted successfully");
    
    Ok(())
}
```

## Advanced Configuration

### TLS Options

By default, this crate uses the native TLS implementation. You can switch to rustls by using a feature flag:

```toml
[dependencies]
imgbb = { version = "1.3.0", features = ["rustls-tls"], default-features = false }
```

### Timeout Configuration

```rust
use imgbb::ImgBB;
use std::time::Duration;

// Create a client with a 10-second timeout
let imgbb = ImgBB::builder("YOUR_API_KEY")
    .timeout(Duration::from_secs(10))
    .build()
    .unwrap();
```

## API Reference

For complete API documentation, see [docs.rs/imgbb](https://docs.rs/imgbb)

## License

imgbb-rs is licensed under the [GNU GPL v3.0](./LICENSE)
