# 🦀 Rust REST API built on Actix-web
![Rust](https://img.shields.io/badge/rust-1.94.0-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

A REST API built with Actix-web featuring JWT authentication
and role-based permissions management.

## Features
- **Users** - registration, JWT login, account management
- **Groups** - role assignment and permissions control
- **Anime** - proxy to the [Jikan API](https://jikan.moe/)

## Getting Started

### Prerequisites
- Rust 1.94.0+
- MongoDB 8.0+ (local)

### Installation
```bash
git clone https://github.com/Cenaure/rust-server
cd rust-server
cp .env.example .env
cargo run
```

### Environment Variables
See [.env.example](https://github.com/Cenaure/rust-server/blob/main/.env.example)
