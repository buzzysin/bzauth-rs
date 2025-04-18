use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Endpoint {
    Url(String),
    UrlWithParams(String, HashMap<String, String>),
}

impl From<&str> for Endpoint {
    fn from(url: &str) -> Self {
        Endpoint::Url(url.to_string())
    }
}

impl From<String> for Endpoint {
    fn from(url: String) -> Self {
        Endpoint::Url(url)
    }
}

impl From<(String, HashMap<String, String>)> for Endpoint {
    fn from((url, params): (String, HashMap<String, String>)) -> Self {
        Endpoint::UrlWithParams(url, params)
    }
}

impl Endpoint {
    pub fn url(&self) -> String {
        match self {
            Endpoint::Url(url) => url.clone(),
            Endpoint::UrlWithParams(url, params) => {
                let mut url_with_params = url.clone();
                for (key, value) in params {
                    url_with_params.push_str(&format!("&{}={}", key, value));
                }
                url_with_params
            }
        }
    }
}
