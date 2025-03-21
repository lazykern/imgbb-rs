use clap::Parser;
use imgbb::{Error, ImgBB};
use std::time::Duration;
use tokio;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// ImgBB API Key
    #[arg(short, long)]
    key: String,

    /// File path of the image to upload
    #[arg(short, long)]
    file: String,

    /// Expiration time in seconds (optional)
    #[arg(short, long)]
    expiration: Option<u64>,

    /// Custom timeout in seconds
    #[arg(short, long)]
    timeout: Option<u64>,

    /// Title for the image
    #[arg(short, long)]
    title: Option<String>,

    /// Name for the image
    #[arg(short, long)]
    name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    // Create a builder for the ImgBB client
    let mut builder = ImgBB::builder(cli.key);
    
    // Add timeout if provided
    if let Some(timeout) = cli.timeout {
        builder = builder.timeout(Duration::from_secs(timeout));
    }
    
    // Build the client
    let imgbb = builder.build()?;

    // If we have additional parameters, use the builder pattern
    if cli.title.is_some() || cli.name.is_some() {
        let mut uploader = imgbb.upload_builder();
        
        // Set the file
        uploader.file(&cli.file)?;
        
        // Set optional parameters
        if let Some(exp) = cli.expiration {
            uploader.expiration(exp);
        }
        
        if let Some(title) = cli.title {
            uploader.title(title);
        }
        
        if let Some(name) = cli.name {
            uploader.name(name);
        }
        
        // Upload the image
        let response = uploader.upload().await?;
        
        // Print the result
        if let Some(data) = response.data {
            println!("✅ Upload successful!");
            println!("ID: {}", data.id.unwrap_or_default());
            println!("URL: {}", data.url.unwrap_or_default());
            println!("Delete URL: {}", data.delete_url.unwrap_or_default());
            
            if let (Some(width), Some(height)) = (data.width, data.height) {
                println!("Dimensions: {}x{}", width, height);
            }
            
            if let Some(size) = data.size {
                println!("Size: {} bytes", size);
            }
        }
    } else {
        // Simple upload
        let response = if let Some(exp) = cli.expiration {
            imgbb.upload_file_with_expiration(&cli.file, exp).await?
        } else {
            imgbb.upload_file(&cli.file).await?
        };
        
        // Print the result
        if let Some(data) = response.data {
            println!("✅ Upload successful!");
            println!("URL: {}", data.url.unwrap_or_default());
            println!("Delete URL: {}", data.delete_url.unwrap_or_default());
        }
    }
    
    Ok(())
}
