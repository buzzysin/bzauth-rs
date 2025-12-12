# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial project structure
- OAuth2 authentication support
- Google OAuth2 provider
- Diesel ORM adapter for PostgreSQL and SQLite
- Axum web framework runtime integration
- CSRF protection with state validation
- Secure session management
- HttpOnly, Secure, SameSite cookie handling
- Session validation endpoint (`/auth/session`)
- Logout endpoint (`/auth/logout`)
- Token refresh endpoint (`/auth/refresh/{provider}`)
- Database migrations for users, accounts, sessions, and verification tokens
- Integration with `cookie` crate for RFC-compliant cookie handling
- Comprehensive test suite with mock OAuth2 provider
- Example: Axum with Diesel

### Security

- CSRF token validation in OAuth2 flow
- Cryptographically secure session token generation
- Parameterized database queries via Diesel ORM
- Secure cookie attributes (HttpOnly, Secure, SameSite)

## [0.1.0] - Unreleased

Initial development version.

[Unreleased]: https://github.com/buzzysin/bzauth-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/buzzysin/bzauth-rs/releases/tag/v0.1.0
