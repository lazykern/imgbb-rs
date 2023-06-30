use clap::Parser;
use imgbb::ImgBB;
use tokio;

#[derive(Parser)]
struct Cli {
    key: String,
    path: String,
    expiration: Option<u64>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let imgbb = ImgBB::new(cli.key);
    let path = cli.path;

    let mut ul = imgbb.read_file(path).expect("Unable to read file");

    if let Some(expiration) = cli.expiration {
        ul.expiration(expiration);
    }

    let res = match ul.upload().await {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    println!("{:#?}", res);
}
