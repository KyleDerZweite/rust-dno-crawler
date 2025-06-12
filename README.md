# rust-dno-crawler

A high-performance, asynchronous web crawler and HTTP API for collecting, processing, and serving data from German Distribution Network Operators (DNOs). Built in Rust with a modular Cargo workspace: separate crates for core logic, authentication, crawling, API server, and frontend.

## Workspace Structure

- **core/**   : Shared data models, database integration (SQLx + SQLite), utility functions.
- **auth/**   : Authentication library (password hashing, JWT) for secure access.
- **api/**    : HTTP server (Axum) exposing RESTful endpoints. Offers both library and binary.
- **crawler/**: Web crawler logic and CLI tool for data harvesting. Provides library and binary.
- **website/**: Dioxus-based frontend (SSR + SPA). Runs standalone with mock or DB data. Library + binary.
- **public/** : Static assets for the frontend (CSS, images).
- **tests/**  : Integration and end-to-end tests.

## Prerequisites

- Rust >=1.70 (stable) with Cargo
- SQLite3 development libraries
- Node.js & npm (for website/tailwind asset pipeline)

## Building the Project

At the root of the workspace:

```bash
# Prepare database file if needed
touch data.db

# Build all crates
cargo build --workspace

# Build frontend assets
npm install --prefix website
npm run build --prefix website
```

## Running Components

### API Server

```bash
cd api
cargo run --bin api
```

### Crawler CLI

```bash
cd crawler
cargo run --bin crawler -- [OPTIONS]
```

### Frontend Website

```bash
cd website
cargo run --bin website
# or serve static assets in public/
```

## Configuration

Environment variables (via `.env` or shell):

- `DATABASE_URL` (SQLite path)
- `JWT_SECRET`
- `OLLAMA_ENDPOINT`

Refer to `.env.example` for full list.

## Testing

- Run unit tests per crate: `cargo test --package <crate>`
- Run all tests: `cargo test --workspace`
- Frontend component tests: `cargo test --package website`

## Contributing

1. Fork the repo and create a feature branch.
2. Adhere to Rust standards: `cargo fmt`, `cargo clippy`.
3. Write tests for new features.
4. Use Conventional Commits (`feat:`, `fix:`, `chore:`, etc.).
5. Submit a pull request with a clear description.

## Roadmap

- Formalize database migrations (SQLx Migrations).
- Complete decoupling of monolith to crates.
- Add scheduler and automations for periodic crawling.
- Enhance frontend UI and user management.

## License

Licensed under the MIT License. See [LICENSE.md](LICENSE.md) for details.

---
