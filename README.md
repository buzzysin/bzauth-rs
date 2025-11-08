# bzauth-rs

> Authentication for Rust backends, inspired by [Auth.js](https://authjs.dev)

![Status](https://img.shields.io/badge/status-work%20in%20progress-yellow)
[![License: ISC](https://img.shields.io/badge/License-ISC-blue.svg)](LICENSE)

A flexible, framework-agnostic OAuth2 authentication library for Rust, designed to be interoperable with current Rust backend solutions while remaining friendly for newcomers from JavaScript.

## ✨ Features

- **Framework Agnostic**: Works with Axum (additional frameworks planned)
- **OAuth2 Providers**: Google support built-in, extensible for more providers
- **Database Adapters**: PostgreSQL and SQLite via Diesel ORM (SQLx support planned)
- **Secure by Default**: CSRF protection, secure sessions, HttpOnly cookies
- **Beginner Friendly**: Clear APIs for developers coming from JavaScript/TypeScript

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
bzauth-rs = { path = "path/to/bzauth-rs", features = ["axum", "diesel", "postgres"] }
```

Basic setup:

```rust
use bzauth_rs::auth::AuthOptions;
use bzauth_rs::providers::DiscordProvider;
use bzauth_rs::adaptors::diesel::{DieselAdaptor, DieselAdapterOptions};
use bzauth_rs::runtimes::axum::{AxumRuntime, AxumRuntimeOptions};
use axum::{Router, Extension};

// Configure authentication
let auth_options = AuthOptions::new()
    .with_adaptor(DieselAdaptor::from_options(diesel_options).into())
    .add_provider(Box::new(DiscordProvider::new()));

// Create runtime and get auth routes
let AxumRuntime { routes, auth } = AxumRuntime::from_options(
    AxumRuntimeOptions { auth_options }
);

// Build your Axum app with auth routes
let app = Router::new()
    .nest("/auth", routes)
    .layer(Extension(auth));
```

## 📚 Examples

Check out the `examples/` directory for complete usage guides:

- [**Axum with Diesel**](bzauth-rs/examples/axum_with_diesel.rs) - Full OAuth2 implementation with PostgreSQL

## 🔧 Available Features

| Feature    | Type        | Description                     |
| ---------- | ----------- | ------------------------------- |
| `axum`     | `framework` | Axum web framework support      |
| `diesel`   | `adapter`   | Diesel ORM database adapter     |
| `sqlx`     | `adapter`   | SQLx database adapter (planned) |
| `postgres` | `backend`   | PostgreSQL database backend     |
| `sqlite`   | `backend`   | SQLite database backend         |

## 📖 Documentation

> _Coming soon_

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## 📄 License

ISC License - Inspired by and following the same license as [Auth.js](https://github.com/nextauthjs/next-auth)

See [LICENSE](LICENSE) for details.
