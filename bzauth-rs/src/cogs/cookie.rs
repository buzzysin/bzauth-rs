use std::{collections::HashMap, str::FromStr};

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

    pub fn unparse(&self) -> String {
        // Algorithm:
        // 1. Naively concatenate all fields
        // 2. Check if the size exceeds MAX_COOKIE_SIZE
        // 3. If it does, truncate the value to fit and split the cookie
        // 3.1 New cookie names are original.(index) where index is the number of chunks
        // Repeat until all fields are within the size limit

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
            let key = kv.next().unwrap_or("").trim();
            let value = kv.next().unwrap_or("").trim();

            match key {
                "Path" => cookie = cookie.with_path(value.to_string()),
                "Domain" => cookie = cookie.with_domain(value.to_string()),
                "Secure" => cookie = cookie.with_secure(value.parse().unwrap_or(false)),
                "HttpOnly" => cookie = cookie.with_http_only(value.parse().unwrap_or(false)),
                "SameSite" => {
                    cookie = cookie.with_same_site(value.parse().unwrap_or(SameSite::Strict))
                }
                "Expires" => cookie = cookie.with_expires(value.parse().unwrap_or(0)),
                "Max-Age" => cookie = cookie.with_max_age(value.parse::<i32>().unwrap_or(0)),
                _ => cookie = cookie.with_value(value.to_string()),
            }
        }
        Ok(cookie)
    }
}

impl std::fmt::Display for Cookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
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

    pub fn set(&mut self, name: String, value: String) {
        // If the cookie exists, update it
        if let Some(cookie) = self.cookies.get_mut(&name) {
            cookie.value = Some(value);
        } else {
            // Otherwise, create a new cookie
            let cookie = Cookie::new(name.clone()).with_value(value);
            self.cookies.insert(name, cookie);
        }
    }

    pub fn get(&self, name: &str) -> Option<Cookie> {
        self.cookies.get(name).cloned().or_else(|| {
            // If the cookie is not found, return a default cookie
            Some(Cookie::new(name.to_string()))
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

        let mut this_cookie = Cookie::new("".to_string());
        // Split the string by semicolon
        // This will produce key=value , with some cookie attributes
        for part in s.split(';') {
            let mut kv = part.split('=');
            let key = kv.next().unwrap_or("").trim();
            let value = kv.next().unwrap_or("").trim();
            // Check if the key is a cookie attribute
            match key {
                "Path" => this_cookie.path = Some(value.to_string()),
                "Domain" => this_cookie.domain = Some(value.to_string()),
                "Secure" => this_cookie.secure = true,
                "HttpOnly" => this_cookie.http_only = true,
                "SameSite" => this_cookie.same_site = value.parse().unwrap_or(SameSite::Strict),
                "Expires" => this_cookie.expires = Some(value.parse().unwrap_or(0)),
                "Max-Age" => this_cookie.max_age = Some(value.parse::<i32>().unwrap_or(0)),
                _ => {
                    // If the key is not a cookie attribute, it is a cookie name
                    if this_cookie.name.is_empty() {
                        this_cookie.name = key.to_string();
                        this_cookie.value = Some(value.to_string());
                    }
                    // If we already have a cookie name, this is a new cookie
                    else {
                        cookies.set(this_cookie.name.clone(), this_cookie.value.unwrap());
                        this_cookie = Cookie::new(key.to_string());
                    }
                }
            }
        }

        // Add the last cookie
        if !this_cookie.name.is_empty() {
            cookies.set(this_cookie.name.clone(), this_cookie.value.unwrap());
        }

        Ok(cookies)
    }
}
