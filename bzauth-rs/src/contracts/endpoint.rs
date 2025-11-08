use std::collections::HashMap;
use std::fmt::Write as _; // for write!

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
                if params.is_empty() {
                    return url.clone();
                }

                let mut url_with_params = String::with_capacity(
                    url.len() + params.len() * 20, // rough estimate
                );
                url_with_params.push_str(url);
                url_with_params.push('?');

                let mut first = true;
                for (key, value) in params {
                    if !first {
                        url_with_params.push('&');
                    }
                    let _ = write!(url_with_params, "{key}={value}");
                    first = false;
                }
                url_with_params
            }
        }
    }
}
