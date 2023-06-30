# imgbb-rs

[ImgBB](https://imgbb.com/) API wrapper for rust

## Getting Started

1. [Register/Log in to ImbBB](https://imgbb.com/login)
2. [Obtain the API Key](https://api.imgbb.com)
3. Use the API Key in with the library

## Usage

### Straightforward

```rust
use imgbb::ImgBB;
use tokio;

#[tokio::main]
async fn main() {
    let imgbb = ImgBB::new("<API KEY>");


    let res = match imgbb.upload_file_with_expiration("<PATH>", <SECONDS>).await {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    println!("{:#?}", res);
}
```

### With uploader

```rust
use imgbb::ImgBB;
use tokio;

#[tokio::main]
async fn main() {

    let imgbb = ImgBB::new("<API KEY>");

    let ul = imgbb
        .read_file("<PATH>").expect("Unable to read file")
        .expiration(<SECONDS>);

    let res = match ul.upload().await {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    println!("{:#?}", res);
}
```

## Supported data types

- File & Path

```rust
    imgbb.read_file("PATH").expect("Unable to read file").upload().await
    // or
    imgbb.upload_file("PATH").await
```

- Bytes (`AsRef<u8>`)

```rust
    imgbb.read_bytes(&[u8]).upload().await
    // or
    imgbb.upload_bytes(&[u8]).await
```

- Base64 String

```rust
    imgbb.read_base64("BASE64").upload().await
    // or
    imgbb.upload_base64("BASE64").await
```

## License

imgbb-rs is licensed under the [GNU GPL v3.0](./LICENSE)
