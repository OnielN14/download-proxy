use reqwest::{header, Response};

pub trait ParseFilename {
    fn get_filename(&self) -> String;
}

impl ParseFilename for Response {
    fn get_filename(&self) -> String {
        let mut filename = String::from("filename");

        if let Some(val) = self.headers().get(header::CONTENT_DISPOSITION) {
            if !val.is_empty() {
                filename = val
                    .to_str()
                    .ok()
                    .filter(|content| content.contains("filename="))
                    .unwrap()
                    .split('=')
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .to_string();
            }
        } else if let Some(val) = self.url().path().split('/').last() {
            filename = urlencoding::decode(val).unwrap().replace(' ', "-");
        }

        filename
    }
}

pub async fn download(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    Ok(response)
}
