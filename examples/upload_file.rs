use clap::Parser;
use imgbb::ImgBB;
use tokio;

#[derive(Parser)]
struct Cli {
    key: String,
    path: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let imgbb = ImgBB::new(cli.key);
    let path = cli.path;

    let res = match imgbb.upload_file(path).await {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    println!("{:#?}", res);
}
