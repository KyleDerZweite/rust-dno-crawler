# DNO Data Gatherer - Project Plan

## Project Overview
A simple, modern Rust application to automatically gather data from German Distribution Network Operators (DNOs) using SearXNG for search and Ollama for AI processing. The system provides both web and mobile interfaces through Dioxus, all consuming the same Axum API.

## Tech Stack
- **Rust** (latest stable) - Core language
- **Dioxus** - Unified web/mobile frontend framework
- **Axum** - API server
- **SQLite + SQLx** - Database with built-in caching
- **Tailwind CSS** - Modern styling
- **SearXNG** - Privacy-respecting search integration
- **Ollama** - Local AI processing
- **Puppeteer MCP** - Webpage analysis and crawling
- **Git** - Version control

## Design System
- **Primary Colors**: Dark theme with Blackstone base
- **Accent Colors**: Green (nature), Amber/Orange/Brown (warm light)
- **Typography**: Modern, clean sans-serif
- **UI Elements**: Glass morphism, subtle animations, warm lighting effects

## Architecture Principles
1. **API-First**: All functionality exposed through REST API
2. **Modular Design**: Each component can run independently
3. **Simple Security**: JWT-based auth with refresh tokens
4. **Built-in Caching**: SQLx query caching + response caching
5. **Mock Data Support**: Enable rapid frontend development

## Phase 1: Foundation (Week 1-2)
### 1.1 Project Setup
- Initialize Rust workspace with cargo
- Setup Git repository with .gitignore
- Create base directory structure
- Configure development environment
- Create CLAUDE.md files for each module

### 1.2 Core Infrastructure
- Setup Axum API server with basic routes
- Configure SQLite database with SQLx
- Implement JWT authentication system
- Setup request/response caching middleware
- Create shared types and utilities crate

### 1.3 Development Tools
- Setup cargo-watch for hot reloading
- Configure rustfmt and clippy
- Create Makefile for common tasks
- Setup basic CI/CD with GitHub Actions

## Phase 2: API Development (Week 3-4)
### 2.1 Authentication API
- User registration/login endpoints
- JWT token generation and validation
- Refresh token mechanism
- Password reset functionality
- Session management

### 2.2 Core API Endpoints
- DNO data CRUD operations
- Search endpoint (proxying to SearXNG)
- Data processing endpoints
- WebSocket support for real-time updates
- Rate limiting middleware

### 2.3 Database Schema
- Users table with auth fields
- DNO entities and metadata
- Search history and results cache
- Crawl queue and status tracking
- API keys management

## Phase 3: Crawler Module (Week 5-6)
### 3.1 CLI Tool Setup
- Create standalone crawler binary
- Argument parsing with clap
- Configuration file support
- Mock data generator for testing

### 3.2 SearXNG Integration
- HTTP client for SearXNG instance
- Search query builder
- Result parser and formatter
- Error handling and retries

### 3.3 Puppeteer MCP Integration
- Setup Puppeteer communication
- Page navigation and analysis
- Data extraction strategies
- Screenshot capabilities
- JavaScript execution support

### 3.4 Crawling Engine
- URL queue management
- Concurrent crawling with tokio
- Rate limiting per domain
- Robots.txt compliance
- Content extraction and parsing

## Phase 4: AI Processing (Week 7)
### 4.1 Ollama Integration
- HTTP client for Ollama API
- Model selection and management
- Prompt engineering for DNO data
- Response parsing and validation

### 4.2 Data Processing Pipeline
- Text cleaning and normalization
- Entity extraction (addresses, dates, etc.)
- Structured data generation
- Quality scoring system

## Phase 5: Frontend Development (Week 8-9)
### 5.1 Dioxus Setup
- Create shared component library
- Setup Tailwind CSS with custom theme
- Configure build pipeline
- Setup routing and state management

### 5.2 Web Application
- Authentication pages (login/register)
- Dashboard with DNO overview
- Search interface with filters
- Data visualization components
- Settings and profile pages

### 5.3 Mobile Web App
- Responsive design implementation
- Touch-optimized interactions
- Offline capability with service workers
- Progressive Web App manifest

### 5.4 Design Implementation
- Dark theme with warm accents
- Glass morphism effects
- Smooth animations and transitions
- Loading states and skeletons
- Error handling UI

## Phase 6: Integration & Testing (Week 10)
### 6.1 Integration Testing
- API endpoint testing
- Crawler integration tests
- Frontend E2E tests with Playwright
- Performance benchmarking

### 6.2 Documentation
- API documentation with OpenAPI
- User guide and tutorials
- Developer documentation
- Deployment guide

## Phase 7: Production Setup (Week 11-12)
### 7.1 Docker Configuration
- Multi-stage Dockerfile for Rust
- docker-compose.yml for development
- Production docker-compose with:
    - PostgreSQL database
    - Redis cache
    - Nginx reverse proxy
    - SSL/TLS configuration

### 7.2 Deployment
- Environment configuration
- Database migrations
- Monitoring and logging setup
- Backup strategies
- CI/CD pipeline updates

## Directory Structure
```
dno-data-gatherer/
├── Cargo.toml              # Workspace configuration
├── CLAUDE.md               # Root project documentation
├── README.md               # User-facing documentation
├── Makefile                # Development commands
├── docker-compose.yml      # Development services
├── docker-compose.prod.yml # Production services
│
├── crates/
│   ├── api/               # Axum API server
│   │   ├── CLAUDE.md
│   │   └── src/
│   ├── crawler/           # Standalone crawler
│   │   ├── CLAUDE.md
│   │   └── src/
│   ├── shared/            # Shared types and utils
│   │   ├── CLAUDE.md
│   │   └── src/
│   └── frontend/          # Dioxus web/mobile app
│       ├── CLAUDE.md
│       └── src/
│
├── migrations/            # SQL migrations
├── config/               # Configuration files
└── scripts/              # Development scripts
```

## Development Workflow
1. Each module has its own CLAUDE.md for Claude Code
2. Use feature branches for development
3. Mock data available for all modules
4. Hot reloading for rapid iteration
5. Comprehensive logging and debugging

## Security Considerations
- Environment-based configuration
- Secure password hashing (argon2)
- Rate limiting on all endpoints
- CORS configuration
- Input validation and sanitization
- SQL injection prevention via SQLx

## Performance Goals
- Sub-100ms API response times
- Concurrent crawling (10+ pages)
- Efficient caching strategies
- Optimized database queries
- Lazy loading for frontend

## Future Enhancements (Post-MVP)
- Advanced search filters
- Batch processing capabilities
- Webhook notifications
- API SDK for external integrations
- Admin dashboard
- Multi-language support