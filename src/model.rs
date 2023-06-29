use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Response {
    pub data: Option<Data>,
    pub success: Option<bool>,
    pub status: Option<u16>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Data {
    pub id: Option<String>,
    pub title: Option<String>,
    pub url_viewer: Option<String>,
    pub url: Option<String>,
    pub display_url: Option<String>,
    pub width: Option<u16>,
    pub height: Option<u16>,
    pub size: Option<u32>,
    pub time: Option<u64>,
    pub expiration: Option<u64>,
    pub image: Option<Image>,
    pub thumb: Option<Image>,
    pub medium: Option<Image>,
    pub delete_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Image {
    pub filename: Option<String>,
    pub name: Option<String>,
    pub mime: Option<String>,
    pub extension: Option<String>,
    pub url: Option<String>,
}
