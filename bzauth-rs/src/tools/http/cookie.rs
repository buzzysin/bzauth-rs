use std::collections::HashMap;
use std::str::FromStr;

// Re-export the well-tested cookie crate types
pub use cookie::{Cookie as InnerCookie, SameSite};

/// A wrapper around a collection of cookies
/// This provides a simple interface for managing cookies in requests and responses
#[derive(Debug, Clone, Default)]
pub struct Cookies {
    cookies: HashMap<String, InnerCookie<'static>>,
}

impl Cookies {
    pub fn new() -> Self {
        Self {
            cookies: HashMap::new(),
        }
    }

    /// Set a cookie with the given name and value
    /// Creates a new cookie with default settings if it doesn't exist
    pub fn set<K: AsRef<str>, V: AsRef<str>>(&mut self, name: K, value: V) {
        let name = name.as_ref().to_string();
        let value = value.as_ref().to_string();

        // If the cookie exists, update its value
        if let Some(cookie) = self.cookies.get_mut(&name) {
            *cookie = InnerCookie::new(name.clone(), value);
        } else {
            // Otherwise, create a new cookie with sensible defaults
            let mut cookie = InnerCookie::new(name.clone(), value);
            cookie.set_path("/");
            cookie.set_same_site(SameSite::Lax);
            cookie.set_http_only(true);
            self.cookies.insert(name, cookie);
        }
    }

    /// Get a cookie by name
    pub fn get<K: AsRef<str>>(&self, name: K) -> Option<&InnerCookie<'static>> {
        self.cookies.get(name.as_ref())
    }

    /// Remove a cookie by name
    pub fn remove(&mut self, name: &str) {
        self.cookies.remove(name);
    }

    /// Extend this collection with cookies from another collection
    pub fn extend(&mut self, other: Self) {
        for (name, cookie) in other.cookies {
            self.cookies.insert(name, cookie);
        }
    }

    /// Convert cookies to a string suitable for the Cookie header
    pub fn unparse(&self) -> String {
        self.cookies
            .values()
            .map(|c| format!("{}={}", c.name(), c.value()))
            .collect::<Vec<String>>()
            .join("; ")
    }

    /// Check if the collection is empty
    pub fn is_empty(&self) -> bool {
        self.cookies.is_empty()
    }

    /// Get an iterator over the cookies
    pub fn iter(&self) -> impl Iterator<Item = (&String, &InnerCookie<'static>)> {
        self.cookies.iter()
    }
}

impl FromStr for Cookies {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cookies = Self::new();

        // Parse cookies from a Cookie header (format: name1=value1; name2=value2)
        for part in s.split(';') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            // Try to parse as a simple name=value pair
            if let Some((name, value)) = part.split_once('=') {
                let name = name.trim().to_string();
                let value = value.trim().to_string();
                
                let mut cookie = InnerCookie::new(name.clone(), value);
                cookie.set_path("/");
                cookie.set_same_site(SameSite::Lax);
                cookie.set_http_only(true);
                
                cookies.cookies.insert(name, cookie);
            } else {
                // Handle cookies without values
                let name = part.trim().to_string();
                let mut cookie = InnerCookie::new(name.clone(), String::new());
                cookie.set_path("/");
                cookie.set_same_site(SameSite::Lax);
                cookie.set_http_only(true);
                
                cookies.cookies.insert(name, cookie);
            }
        }

        Ok(cookies)
    }
}

