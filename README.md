# DNO Data Gatherer 🌿

A modern, simple Rust application for automatically gathering and processing data from German Distribution Network Operators (DNOs).

## Features ✨

- 🔍 **Smart Search**: Integrated SearXNG for privacy-respecting web searches
- 🤖 **AI Processing**: Local Ollama integration for intelligent data extraction
- 📱 **Cross-Platform**: Single codebase for web and mobile using Dioxus
- 🚀 **Fast & Efficient**: Built with Rust for maximum performance
- 🔐 **Secure**: JWT authentication with refresh tokens
- 🎨 **Beautiful UI**: Dark theme with nature-inspired accents
- 📊 **Real-time Updates**: WebSocket support for live data

## Quick Start 🏃

### Prerequisites

- Rust (latest stable)
- SQLite
- Node.js (for Tailwind CSS)
- Docker (optional, for production setup)
- SearXNG instance
- Ollama installed locally

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/dno-data-gatherer.git
cd dno-data-gatherer

# Install dependencies
make setup

# Run database migrations
make migrate

# Start development server
make dev
```

### Running Individual Components

```bash
# API Server only
cargo run --bin api

# Crawler CLI
cargo run --bin crawler -- --help

# Frontend with mock data
cd crates/frontend && npm run dev
```

## Architecture 🏗️

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Dioxus    │────▶│   Axum API  │────▶│   SQLite    │
│  Web/Mobile │     │   Server    │     │  Database   │
└─────────────┘     └─────────────┘     └─────────────┘
                           │
                    ┌──────┴──────┐
                    │             │
              ┌─────▼─────┐ ┌────▼────┐
              │  Crawler  │ │ Ollama  │
              │  Engine   │ │   AI    │
              └─────┬─────┘ └─────────┘
                    │
              ┌─────▼─────┐
              │ SearXNG   │
              │  Search   │
              └───────────┘
```

## Configuration ⚙️

Create a `.env` file in the root directory:

```env
# Database
DATABASE_URL=sqlite://data.db

# API Server
API_PORT=8080
JWT_SECRET=your-secret-key

# SearXNG
SEARXNG_URL=http://localhost:8888

# Ollama
OLLAMA_URL=http://localhost:11434
OLLAMA_MODEL=llama3

# Frontend
VITE_API_URL=http://localhost:8080
```

## Development 👩‍💻

### Available Commands

```bash
make dev        # Start all services in development mode
make test       # Run all tests
make lint       # Run clippy and format check
make build      # Build release binaries
make docker     # Build Docker images
```

### Project Structure

- `/crates/api` - Axum REST API server
- `/crates/crawler` - Standalone crawling engine
- `/crates/frontend` - Dioxus web/mobile application
- `/crates/shared` - Shared types and utilities

## API Documentation 📚

API documentation is available at `http://localhost:8080/swagger` when running in development mode.

### Key Endpoints

- `POST /auth/login` - User authentication
- `GET /api/dnos` - List all DNOs
- `POST /api/search` - Search DNO data
- `GET /api/crawl/status` - Crawl job status
- `WS /ws` - WebSocket for real-time updates

## Deployment 🚀

### Using Docker Compose

```bash
# Development
docker-compose up

# Production
docker-compose -f docker-compose.prod.yml up -d
```

### Manual Deployment

1. Build release binaries: `make build-release`
2. Set up PostgreSQL and Redis (production)
3. Configure environment variables
4. Run migrations: `./api migrate`
5. Start services with systemd or supervisor

## Design Philosophy 🎨

Our UI follows a nature-inspired dark theme:
- **Base**: Deep blackstone colors
- **Accents**: Forest greens and warm amber/orange tones
- **Effects**: Subtle glass morphism and smooth animations
- **Typography**: Clean, modern sans-serif

## Contributing 🤝

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments 🙏

- Built with love for KyleDerZweite and German friends
- Powered by the amazing Rust ecosystem
- Special thanks to the Dioxus and Axum communities

---

**Need help?** Open an issue or reach out in discussions!