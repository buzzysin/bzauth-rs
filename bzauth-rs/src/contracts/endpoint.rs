use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Endpoint {
    Url(String),
    UrlWithParams(String, HashMap<String, String>),
}

impl From<&str> for Endpoint {
    fn from(url: &str) -> Self {
        Self::Url(url.to_string())
    }
}

impl From<String> for Endpoint {
    fn from(url: String) -> Self {
        Self::Url(url)
    }
}

impl From<(String, HashMap<String, String>)> for Endpoint {
    fn from((url, params): (String, HashMap<String, String>)) -> Self {
        Self::UrlWithParams(url, params)
    }
}

impl Endpoint {
    pub fn url(&self) -> String {
        match self {
            Self::Url(url) => url.clone(),
            Self::UrlWithParams(url, params) => {
                let mut url_with_params = url.clone();
                if params.is_empty() {
                    return url_with_params;
                }

                url_with_params.push('?');
                for (key, value) in params {
                    url_with_params.push_str(&format!("&{key}={value}"));
                }
                url_with_params
            }
        }
    }
}
