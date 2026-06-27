use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct HarRoot {
    pub log: HarLog,
}

#[derive(Deserialize, Debug)]
pub struct HarLog {
    pub entries: Vec<HarEntry>,
}

#[derive(Deserialize, Debug)]
pub struct HarEntry {
    pub request: HarRequest,
    pub response: HarResponse,
}

#[derive(Deserialize, Debug)]
pub struct HarRequest {
    pub method: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct HarResponse {
    pub content: HarContent,
}

#[derive(Deserialize, Debug)]
pub struct HarContent {
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    pub text: Option<String>,
}
