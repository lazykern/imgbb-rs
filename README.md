# imgbb-rs

[ImgBB](https://imgbb.com/) API wrapper for rust

## Supported data types

- File & Path

```rust
    imgbb.read_file("PATH").await
```

- Bytes (`AsRef<u8>`)

```rust
    imgbb.read_bytes(&[u8])
```

- Base64 String

```rust
    imgbb.read_base64("BASE64")
```

## Usage

### Straightforward

```rust
use imgbb::ImgBB;
use tokio;

#[tokio::main]
async fn main() {
    let imgbb = ImgBB::new("<API KEY>");


    let res = match imgbb.upload_file_with_expiration("<FILE PATH>", <SECONDS>).await {
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
        .read_file("PATH").await.expect("Unable to read file")
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

## License

imgbb-rs is licensed under the [GNU GPL v3.0](./LICENSE)
