use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum CookieAttribute {
    Path,
    Domain,
    Secure,
    HttpOnly,
    SameSite,
    Expires,
    MaxAge,
}

#[derive(Debug, Clone)]
pub struct ParseCookieAttributeError(String);
impl std::fmt::Display for ParseCookieAttributeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid cookie attribute: {}", self.0)
    }
}
impl std::error::Error for ParseCookieAttributeError {}

impl FromStr for CookieAttribute {
    type Err = ParseCookieAttributeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Path" => Ok(CookieAttribute::Path),
            "Domain" => Ok(CookieAttribute::Domain),
            "Secure" => Ok(CookieAttribute::Secure),
            "HttpOnly" => Ok(CookieAttribute::HttpOnly),
            "SameSite" => Ok(CookieAttribute::SameSite),
            "Expires" => Ok(CookieAttribute::Expires),
            "Max-Age" => Ok(CookieAttribute::MaxAge),
            _ => Err(ParseCookieAttributeError(s.to_string())),
        }
    }
}

impl std::fmt::Display for CookieAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CookieAttribute::Path => write!(f, "Path"),
            CookieAttribute::Domain => write!(f, "Domain"),
            CookieAttribute::Secure => write!(f, "Secure"),
            CookieAttribute::HttpOnly => write!(f, "HttpOnly"),
            CookieAttribute::SameSite => write!(f, "SameSite"),
            CookieAttribute::Expires => write!(f, "Expires"),
            CookieAttribute::MaxAge => write!(f, "Max-Age"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum SameSite {
    #[default]
    Strict,
    Lax,
    None,
}

#[derive(Debug, Clone)]
pub struct ParseSameSiteError(String);

impl std::fmt::Display for ParseSameSiteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid SameSite value: {}", self.0)
    }
}

impl std::error::Error for ParseSameSiteError {}

impl FromStr for SameSite {
    type Err = ParseSameSiteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Strict" => Ok(SameSite::Strict),
            "Lax" => Ok(SameSite::Lax),
            "None" => Ok(SameSite::None),
            _ => Err(ParseSameSiteError(s.to_string())),
        }
    }
}

impl std::fmt::Display for SameSite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SameSite::Strict => write!(f, "Strict"),
            SameSite::Lax => write!(f, "Lax"),
            SameSite::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cookie {
    /// The name of the cookie
    pub name: String,
    /// The value of the cookie
    pub value: Option<String>,
    /// The path for which the cookie is valid
    pub path: Option<String>,
    /// The domain for which the cookie is valid
    pub domain: Option<String>,
    /// Whether the cookie is secure (only sent over HTTPS)
    pub secure: bool,
    /// Whether the cookie is HTTP-only (not accessible via JavaScript)
    pub http_only: bool,
    /// The SameSite attribute of the cookie
    pub same_site: SameSite,
    /// The expiration date of the cookie
    pub expires: Option<i32>, // todo: use chrono::DateTime<Utc>,
    /// The maximum age of the cookie in seconds
    pub max_age: Option<i32>, // todo: use chrono::DateTime<Utc>,
}

impl Cookie {
    pub fn new(name: String) -> Self {
        Cookie {
            name,
            value: None,
            path: None,
            domain: None,
            secure: false,
            http_only: false,
            same_site: SameSite::Strict,
            expires: None,
            max_age: None,
        }
    }

    pub fn with_value(self, value: String) -> Self {
        Cookie {
            value: Some(value),
            ..self
        }
    }
    pub fn with_path(self, path: String) -> Self {
        Cookie {
            path: Some(path),
            ..self
        }
    }
    pub fn with_domain(self, domain: String) -> Self {
        Cookie {
            domain: Some(domain),
            ..self
        }
    }
    pub fn with_secure(self, secure: bool) -> Self {
        Cookie { secure, ..self }
    }
    pub fn with_http_only(self, http_only: bool) -> Self {
        Cookie { http_only, ..self }
    }
    pub fn with_same_site(self, same_site: SameSite) -> Self {
        Cookie { same_site, ..self }
    }
    pub fn with_expires(self, expires: i32) -> Self {
        Cookie {
            expires: Some(expires),
            ..self
        }
    }
    pub fn with_max_age(self, max_age: i32) -> Self {
        Cookie {
            max_age: Some(max_age),
            ..self
        }
    }

    /// Turns the cookie into a naive string. True cookies will have to be chunked into parts
    /// and sent as multiple cookies when over the size limit. This is left as a TODO in the
    /// [Cookies] struct.
    ///
    /// [Cookies]: struct.Cookies.html
    pub fn unparse(&self) -> String {
        let mut cookie_string = format!(
            "{}={}",
            self.name,
            self.value.as_ref().unwrap_or(&"".to_string())
        );
        if let Some(path) = &self.path {
            cookie_string.push_str(&format!("; Path={}", path));
        }
        if let Some(domain) = &self.domain {
            cookie_string.push_str(&format!("; Domain={}", domain));
        }
        if self.secure {
            cookie_string.push_str("; Secure");
        }
        if self.http_only {
            cookie_string.push_str("; HttpOnly");
        }
        cookie_string.push_str(&format!("; SameSite={}", self.same_site));
        if let Some(expires) = &self.expires {
            cookie_string.push_str(&format!("; Expires={}", expires));
        }
        if let Some(max_age) = self.max_age {
            cookie_string.push_str(&format!("; Max-Age={}", max_age));
        }

        cookie_string
    }
}

impl FromStr for Cookie {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cookie = Cookie::new("".to_string());

        for part in s.split(';') {
            let mut kv = part.split('=');
            let full_key = kv.next().unwrap_or("").trim().to_string();
            let value = kv.next().unwrap_or("").trim();

            // Secure/Host cookies have a prefix: __Secure-Name=Value or __Host-Name=Value
            let (key, secure, _host) = if full_key.to_lowercase().starts_with("__secure-") {
                (
                    full_key.trim_start_matches("__secure-").to_string(),
                    true,
                    false,
                )
            } else if full_key.starts_with("__host-") {
                (
                    full_key.trim_start_matches("__host-").to_string(),
                    false,
                    true,
                )
            } else {
                (full_key.to_string(), false, false)
            };

            match key.to_lowercase().as_str() {
                "path" => cookie = cookie.with_path(value.to_string()),
                "domain" => cookie = cookie.with_domain(value.to_string()),
                "httponly" => cookie = cookie.with_http_only(value.parse().unwrap_or(false)),
                "samesite" => {
                    cookie = cookie.with_same_site(value.parse().unwrap_or(SameSite::Strict))
                }
                "expires" => cookie = cookie.with_expires(value.parse().unwrap_or(0)),
                "max-age" => cookie = cookie.with_max_age(value.parse::<i32>().unwrap_or(0)),
                _ => {
                    cookie = cookie.with_value(value.to_string());
                    if (secure) && !cookie.name.is_empty() {
                        // If the cookie is secure or a host cookie, we set the secure flag
                        cookie = cookie.with_secure(secure);
                    }
                }
            }
        }
        Ok(cookie)
    }
}

impl std::fmt::Display for Cookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.unparse())
    }
}

// TODO:
const _MAX_COOKIE_SIZE: usize = 4096; // 4KB

#[derive(Debug, Clone, Default)]
pub struct Cookies {
    pub cookies: HashMap<String, Cookie>,
}

impl Cookies {
    pub fn new() -> Self {
        Cookies {
            cookies: HashMap::new(),
        }
    }

    pub fn set<K: Into<String>, V: Into<String>>(&mut self, name: K, value: V) {
        let name = name.into();
        let value = value.into();

        // If the cookie exists, update it
        if let Some(cookie) = self.cookies.get_mut(&name) {
            cookie.value = Some(value);
        } else {
            // Otherwise, create a new cookie
            let cookie = Cookie::new(name.clone()).with_value(value);
            self.cookies.insert(name, cookie);
        }
    }

    pub fn get<K: Into<String>>(&self, name: K) -> Option<Cookie> {
        let name = name.into();

        self.cookies.get(&name).cloned().or_else(|| {
            // If the cookie is not found, return a default cookie
            Some(Cookie::new(name))
        })
    }

    pub fn remove(&mut self, name: &str) {
        self.cookies.remove(name);
    }

    pub fn extend(&mut self, other: Cookies) {
        for (name, cookie) in other.cookies {
            self.cookies.insert(name, cookie);
        }
    }

    pub fn unparse(&self) -> String {
        self.cookies
            .values()
            .map(|cookie| cookie.unparse())
            .collect::<Vec<String>>()
            .join("; ")
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Cookie)> {
        self.cookies.iter()
    }
}

impl FromStr for Cookies {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cookies = Cookies::new();

        let mut currently_processing_cookie: Option<String> = None;
        for part in s.split(';') {
            let part = part.trim();
            let kv: Vec<&str> = part.split('=').collect();
            let k = kv.first().map(|s| s.trim()).unwrap_or("");
            let v = kv.get(1).map(|s| s.trim()).unwrap_or("");

            // Skip empty parts
            if k.is_empty() {
                continue;
            }

            match k.to_string().to_lowercase().as_str() {
                "path" | "domain" | "secure" | "httponly" | "samesite" | "expires" | "max-age" => {
                    // if processing a cookie string, append to the current cookie
                    if let Some(partial_cookie) = &mut currently_processing_cookie {
                        if v.is_empty() {
                            // Probably a Secure or HttpOnly attribute without a value
                            partial_cookie.push_str(&format!("; {}", k));
                        } else {
                            // Probably a key-value pair
                            partial_cookie.push_str(&format!("; {}={}", k, v));
                        }
                    } else {
                        // Start a new cookie
                        currently_processing_cookie = Some(k.to_string());
                    }
                }
                _ => {
                    // If we have a currently processing cookie, finalize it
                    if let Some(partial_cookie) = currently_processing_cookie.take() {
                        let cookie = Cookie::from_str(&partial_cookie)?;
                        cookies.cookies.insert(cookie.name.clone(), cookie);
                    }

                    // Now process the key-value pair as a new cookie
                    currently_processing_cookie = Some(part.to_string());
                }
            }
        }

        Ok(cookies)
    }
}
