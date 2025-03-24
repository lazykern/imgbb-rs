use clap::Parser;
use imgbb::ImgBB;
use reqwest::Client;
use std::time::Duration;

#[derive(Parser)]
#[command(about = "Example of using ImgBB with custom client configurations")]
struct Args {
    /// ImgBB API key
    #[arg(short, long)]
    key: String,

    /// Path to the image file
    #[arg(short, long)]
    file: String,

    /// Use proxy (optional)
    #[arg(long)]
    proxy: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Example 1: Basic custom client
    println!("Example 1: Basic custom client");
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("MyApp/1.0")
        .https_only(true)
        .build()?;

    let imgbb = ImgBB::new_with_client(&args.key, client);
    let response = imgbb.upload_file(&args.file).await?;
    println!("Upload successful: {}", response.data.unwrap().url.unwrap());

    // Example 2: Using builder pattern
    println!("\nExample 2: Using builder pattern");
    let client = Client::builder()
        .timeout(Duration::from_secs(45))
        .user_agent("CustomApp/2.0")
        .build()?;

    let imgbb = ImgBB::builder(&args.key)
        .client(client)
        .build()
        .expect("Failed to create ImgBB client");
    
    let response = imgbb.upload_file(&args.file).await?;
    println!("Upload successful: {}", response.data.unwrap().url.unwrap());

    // Example 3: With proxy (if provided)
    if let Some(proxy_url) = args.proxy {
        println!("\nExample 3: With proxy configuration");
        let proxy = reqwest::Proxy::http(&proxy_url)?;
        
        let client = Client::builder()
            .proxy(proxy)
            .timeout(Duration::from_secs(60))
            .https_only(true)
            .build()?;

        let imgbb = ImgBB::new_with_client(&args.key, client);
        let response = imgbb.upload_file(&args.file).await?;
        println!("Upload successful: {}", response.data.unwrap().url.unwrap());
    }

    Ok(())
} 