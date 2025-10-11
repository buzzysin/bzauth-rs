#![forbid(unsafe_code)]
#![warn(
    // missing_docs, // TODO: this is a work in progress, so we will enable this later
    // missing_debug_implementations,
    clippy::all,
    clippy::pedantic,
    clippy::nursery
)]
#![
    allow(
        // TODO: Document all possible errors
        clippy::missing_errors_doc,
        // TODO: Mark values that should not be discarded (like [[nodiscard]])
        clippy::must_use_candidate
    )
]

//! ## (bz)Auth.rs
//! A Rust library for building authentication and authorization systems.
//!
//! This library provides a set of tools and contracts to create secure authentication flows, manage user sessions, and integrate with various adaptors for different authentication providers.
//! ### Features
//! - **Adaptors**: Support for multiple authentication providers.
//! - **Contracts**: Define the core contracts for users, accounts, and sessions.
//! - **Tools**: Utility functions for handling requests, responses, and errors.
//! ### Usage
//! To use this library, coose a runtime, some providers and a database adaptors (or write your own of each) and configure the authentication system according to your needs. The library provides a flexible architecture that allows you to customize the authentication flow, session management, and user interactions.
//!
//! ### TODOs
//! - [ ] Implement more adaptors for different authentication providers. Currently only supports Discord and Google
//! - [ ] Add more runtime support, currently only supports Axum (Actix coming soon)
//! - [ ] Improve documentation and examples for better usability.
//! - [ ] Improve function signatures for better clarity and usability.
//! - [ ] Introduce more user configurability options (i.e. encryption, session management, etc.)

// Internals
pub mod contracts;
pub mod tools;

// Externals
pub mod adaptors;
pub mod auth;
pub mod providers;
pub mod runtimes;
