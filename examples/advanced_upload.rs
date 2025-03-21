use clap::Parser;
use imgbb::{ImgBB, Error};
use std::time::Duration;
use tokio;

#[derive(Parser)]
#[clap(author, version, about = "Advanced ImgBB upload example")]
struct Cli {
    /// ImgBB API key
    #[clap(short, long)]
    key: String,
    
    /// Path to the image file
    #[clap(short, long)]
    path: String,
    
    /// Optional timeout in seconds
    #[clap(short, long, default_value = "30")]
    timeout: u64,
    
    /// Optional image title
    #[clap(short, long)]
    title: Option<String>,
    
    /// Optional image name
    #[clap(short, long)]
    name: Option<String>,
    
    /// Optional expiration time in seconds
    #[clap(short, long)]
    expiration: Option<u64>,
    
    /// Optional album ID
    #[clap(short, long)]
    album: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // Create ImgBB client with custom settings
    let imgbb = ImgBB::builder(cli.key)
        .timeout(Duration::from_secs(cli.timeout))
        .user_agent(format!("MyApp/1.0 ImgBB-Uploader"))
        .build()?;
    
    // Create upload builder
    let mut builder = imgbb.upload_builder();
    
    // Add the file
    match builder.file(&cli.path) {
        Ok(_) => println!("File loaded successfully: {}", cli.path),
        Err(e) => {
            eprintln!("Error loading file: {}", e);
            return Err(e.into());
        }
    }
    
    // Add optional parameters
    if let Some(exp) = cli.expiration {
        builder.expiration(exp);
        println!("Setting expiration: {} seconds", exp);
    }
    
    if let Some(title) = cli.title {
        builder.title(title.clone());
        println!("Setting title: {}", title);
    }
    
    if let Some(name) = cli.name {
        builder.name(name.clone());
        println!("Setting name: {}", name);
    }
    
    if let Some(album) = cli.album {
        builder.album(album.clone());
        println!("Setting album ID: {}", album);
    }
    
    // Upload with detailed error handling
    println!("Uploading image (timeout: {} seconds)...", cli.timeout);
    let res = match builder.upload().await {
        Ok(res) => res,
        Err(Error::ReqwestError(e)) if e.is_timeout() => {
            eprintln!("✗ Upload timed out after {} seconds", cli.timeout);
            return Err(e.into());
        },
        Err(Error::ReqwestError(e)) if e.is_connect() => {
            eprintln!("✗ Connection error: {}", e);
            return Err(e.into());
        },
        Err(e) => {
            eprintln!("✗ Upload error: {}", e);
            return Err(e.into());
        }
    };
    
    // Check the response
    if let Some(true) = res.success {
        if let Some(data) = res.data {
            println!("\n✓ Upload successful!");
            println!("----------------------------------");
            println!("Image ID: {}", data.id.unwrap_or_default());
            println!("Image URL: {}", data.url.unwrap_or_default());
            println!("Display URL: {}", data.display_url.unwrap_or_default());
            println!("Delete URL: {}", data.delete_url.unwrap_or_default());
            
            if let Some(width) = data.width {
                if let Some(height) = data.height {
                    println!("Dimensions: {}x{}", width, height);
                }
            }
            
            if let Some(size) = data.size {
                println!("Size: {} bytes", size);
            }
            
            if let Some(exp) = data.expiration {
                println!("Expires in: {} seconds", exp);
            }
            
            println!("----------------------------------");
        }
    } else if let Some(error) = res.error {
        eprintln!("✗ Upload failed: {}", error.message.unwrap_or_default());
        if let Some(code) = error.code {
            eprintln!("Error code: {}", code);
        }
    } else {
        eprintln!("✗ Upload failed with unknown error");
    }
    
    Ok(())
} 