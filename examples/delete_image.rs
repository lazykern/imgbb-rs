use clap::Parser;
use imgbb::{Error, ImgBB};
use tokio;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// ImgBB API Key
    #[arg(short, long)]
    key: String,

    /// Delete URL for the image (from the delete_url field)
    #[arg(short, long)]
    delete_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    // Create ImgBB client
    let imgbb = ImgBB::new(cli.key);

    // Try to delete the image
    println!("🗑️ Attempting to delete image...");
    
    match imgbb.delete(&cli.delete_url).await {
        Ok(_) => {
            println!("✅ Image successfully deleted!");
            Ok(())
        },
        Err(Error::InvalidApiKey) => {
            eprintln!("❌ Invalid API key! Please check your API key and try again.");
            Err(Error::InvalidApiKey)
        },
        Err(Error::ApiError { message, status, code }) => {
            eprintln!("❌ API Error (Status: {}, Code: {:?}): {}", 
                      status.unwrap_or(0), 
                      code, 
                      message);
            Err(Error::ApiError { message, status, code })
        },
        Err(err) => {
            eprintln!("❌ Delete failed: {}", err);
            Err(err)
        }
    }
} 