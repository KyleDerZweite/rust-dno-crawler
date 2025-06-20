# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

# DNO Data Gatherer - Claude Code Instructions

## Project Overview

This is a Rust-based AI-driven application for intelligent gathering and processing of data from German Distribution Network Operators (DNOs). The project uses modern web technologies with a focus on artificial intelligence and machine learning for automated data collection.

**IMPORTANT**: When working in subdirectories, Claude Code should generate new CLAUDE.md files specific to each module/crate in the workspace.

## Project Context

- **Target Audience**: KyleDerZweite and German friends
- **Primary Goal**: Automate DNO data collection using AI-driven intelligent agents
- **Design Philosophy**: AI-first approach with self-learning capabilities
- **Architecture**: API-driven with intelligent AI agents

## Technology Stack

### Core
- **Language**: Rust (latest stable)
- **Build System**: Cargo workspace
- **Version Control**: Git

### Backend
- **Web Framework**: Axum
- **Database**: SQLite with SQLx (PostgreSQL in production)
- **Caching**: Built-in SQLx caching (Redis in production)
- **Authentication**: JWT with refresh tokens

### AI & Machine Learning
- **AI Agent**: Reinforcement Learning with epsilon-greedy exploration
- **Data Evaluation**: Intelligent quality assessment and validation
- **Search Strategy**: Adaptive learning from successful patterns
- **Model Storage**: Persistent AI model weights and memory

### Frontend
- **Framework**: Dioxus (unified web/mobile)
- **Styling**: Tailwind CSS
- **State Management**: Dioxus built-in
- **Build Tool**: Vite

### External Services
- **Search**: SearXNG instance
- **AI Processing**: Ollama (local)
- **Web Scraping**: Puppeteer MCP

## Project Structure

```
rust-dno-crawler/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── api/               # Axum API server with AI endpoints
│   ├── crawler/           # AI-driven intelligent crawler
│   ├── frontend/          # Dioxus web/mobile interface
│   └── shared/            # Shared types and models
├── migrations/            # SQL migrations
└── config/               # Configuration files
```

## AI System Architecture

### Intelligent Gathering Agent
The core of the system is an AI agent that:
- **Learns**: Uses reinforcement learning to improve data gathering strategies
- **Adapts**: Adjusts search patterns based on DNO-specific success rates
- **Evaluates**: Self-assesses data quality and completeness
- **Optimizes**: Balances exploration of new strategies vs exploitation of proven methods

### Data Evaluation Engine
Comprehensive quality assessment:
- **German Data Validation**: Specialized rules for German addresses, phone numbers, emails
- **Completeness Scoring**: Ensures all required Netzentgelte and HLZF fields are found
- **Quality Metrics**: Assesses data accuracy and format compliance
- **Recommendations**: Provides intelligent suggestions for improvement

## Development Setup

```bash
# Install dependencies
rustup update stable
cargo install cargo-watch sqlx-cli
npm install -g @tailwindcss/cli

# Setup database
export DATABASE_URL="sqlite://data.db"
sqlx database create
sqlx migrate run

# Setup SearXNG (required for AI agent)
export SEARXNG_URL="http://localhost:8080"

# Run development server
cargo watch -x "run --bin api"
```

## Key Commands

### Building
```bash
cargo build                 # Debug build
cargo build --release      # Release build
cargo build --bin crawler  # Build AI crawler
```

### Testing
```bash
cargo test                 # Run all tests
cargo test --workspace     # Test entire workspace
cargo test -p api         # Test specific crate
```

### Running
```bash
cargo run --bin api        # Run API server
cargo run --bin crawler    # Run AI crawler CLI
```

### AI Crawler Commands
```bash
# Primary AI-driven storage gathering
crawler ai-gather "Netze BW" --storage-types "netzentgelte,hlzf" --json

# Test SearXNG connectivity
crawler search "test query" --json

# Help and options
crawler ai-gather --help
```

### Database
```bash
sqlx migrate add <name>    # Create new migration
sqlx migrate run          # Run migrations
sqlx migrate revert       # Revert last migration
```

## Design System

### Colors
- **Background**: #0a0a0a (Deep black)
- **Surface**: #1a1a1a (Blackstone)
- **Primary**: #22c55e (Forest green)
- **Secondary**: #f59e0b (Warm amber)
- **Accent**: #a16207 (Deep brown)

### UI Components
- Glass morphism effects with backdrop-blur
- Smooth transitions (200-300ms)
- Warm lighting gradients
- Nature-inspired iconography

## API Design Principles

1. **Data-Driven**: All endpoints provide structured data access
2. **RESTful**: Follow REST conventions
3. **Versioned**: `/api/v1/` prefix
4. **Consistent**: Standardized response format
5. **Secure**: JWT auth on protected routes

### Standard Response Format
```json
{
  "success": true,
  "data": {},
  "metadata": {
    "timestamp": "2025-01-01T00:00:00Z",
    "version": "2.0.0",
    "request_id": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

## AI Crawler Architecture

The AI crawler features:
1. **Intelligent Agent**: Self-learning data gathering with reinforcement learning
2. **SearXNG Integration**: Optimized search strategies for German DNO data
3. **Quality Evaluation**: Real-time assessment of gathered data
4. **Adaptive Learning**: Improves strategies based on success/failure patterns

### Core AI Features
- **Exploration vs Exploitation**: Balances trying new strategies vs using proven ones
- **DNO-Specific Learning**: Adapts strategies for different Distribution Network Operators
- **Pattern Recognition**: Learns URL patterns and content indicators
- **Quality Optimization**: Continuously improves data quality scores

## API Endpoints

### Authentication
- `POST /auth/login` - User login
- `POST /auth/register` - User registration
- `POST /auth/refresh` - Token refresh
- `POST /auth/logout` - User logout

### DNO Management
- `GET /dnos/` - List all DNOs
- `POST /dnos/` - Create new DNO
- `GET /dnos/:id` - Get specific DNO
- `PUT /dnos/:id` - Update DNO
- `DELETE /dnos/:id` - Delete DNO

### Search & Data
- `POST /search/` - Perform search
- `GET /search/history` - Search history
- `POST /search/gather` - Gather data from search results

### Search & Data Retrieval
- `POST /search/dno` - Search data by DNO name or ID
- `POST /search/year` - Search data by year
- `POST /search/data-type` - Search data by type (netzentgelte/hlzf)
- `GET /search/` - Search with multiple filters

### Crawl Jobs
- `GET /crawl/jobs` - List crawl jobs
- `POST /crawl/jobs` - Create new crawl job
- `GET /crawl/jobs/:id` - Get job status
- `GET /crawl/jobs/:id/status` - Get detailed job status

### Admin (Admin Auth Required)
- `GET /admin/dashboard` - Admin dashboard with AI metrics
- `GET /admin/flags` - Data quality flags
- `POST /admin/flags` - Flag data issues
- `POST /admin/flags/:id/resolve` - Resolve flagged data
- `GET /admin/patterns` - Pattern verification
- `POST /admin/patterns/:id/verify` - Verify patterns
- `GET /admin/sources` - Source management
- `GET /admin/audit` - Audit logs

### Job Management (Admin Auth Required)
- `GET /jobs/` - Get automated jobs
- `POST /jobs/` - Create automated job
- `PUT /jobs/:job_id` - Update job
- `DELETE /jobs/:job_id` - Delete job
- `POST /jobs/:job_id/control` - Control job execution
- `GET /jobs/:job_id/history` - Job execution history
- `GET /jobs/system/status` - System status

## Frontend Development

### Architecture & Responsibilities
The frontend (website) layer handles:
- **Natural Language Processing**: Parse German text queries like "Zeig mir die Netzentgelte von Netze BW für 2024"
- **Query Translation**: Convert user input to structured API calls
- **User Experience**: Interactive components and natural language responses
- **DNO Recognition**: Identify and normalize German distribution network operator names
- **Data Presentation**: Display search results and data in user-friendly formats

The API layer provides:
- **Fast Data Access**: Structured, deterministic endpoints for data retrieval
- **Authentication**: User management and authorization
- **Data Management**: CRUD operations for DNO data and metadata

### Website Pages
1. **Homepage**: Landing page with project overview and getting started information
2. **Login/Register**: User authentication
3. **User Dashboard**: Natural language query interface, personal data access and search history (user auth)
4. **Admin User Management**: User administration (admin auth)
5. **Admin Data Management**: Data verification, source management, system monitoring (admin auth)

### Dioxus Components
- Use functional components with hooks
- Implement responsive design first
- Support both web and mobile from single codebase
- Use Tailwind utility classes only

### State Management
```rust
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct AppState {
    user: Option<User>,
    dnos: Vec<Dno>,
    ai_metrics: Option<AiMetrics>,
    crawler_status: CrawlerStatus,
}
```

## Security Guidelines

1. **Authentication**: JWT with short expiry (15min)
2. **Refresh Tokens**: Secure HTTP-only cookies
3. **Input Validation**: Use strong types
4. **SQL Injection**: Prevented by SQLx
5. **CORS**: Configured per environment
6. **Rate Limiting**: Per-route limits
7. **Admin Protection**: Admin routes require special authentication

## Testing Strategy

### Unit Tests
- Test AI agent learning algorithms
- Mock external dependencies
- Focus on business logic

### Integration Tests
- Test API endpoints
- Test AI crawler functionality
- Use test database
- Clean state between tests

### E2E Tests
- Playwright for frontend
- Full user flows including AI crawling
- Cross-browser testing

## Performance Targets

- API Response: <100ms p95
- Database Queries: <50ms p95
- AI Crawler: 10+ concurrent intelligent agents
- Frontend: <3s initial load
- AI Processing: <30s for comprehensive data gathering

## Common Tasks

### Adding a New API Endpoint
1. Define route in `api/src/routes.rs`
2. Implement handler in appropriate module
3. Add request/response types to `shared`
4. Update OpenAPI documentation
5. Write tests

### Improving AI Agent
1. Modify `crawler/src/ai_agent.rs`
2. Update learning algorithms or reward functions
3. Test with different DNOs
4. Monitor performance metrics
5. Update evaluation engine if needed

### Creating Frontend Components
1. Create component in `frontend/src/components/`
2. Use Tailwind classes for styling
3. Implement responsive behavior
4. Add AI status indicators
5. Document props and usage

## Debugging Tips

### API Issues
```bash
RUST_LOG=debug cargo run --bin api
# Check logs for detailed traces
```

### AI Agent Issues
```bash
RUST_LOG=crawler::ai_agent=debug cargo run --bin crawler -- ai-gather "Test DNO" --json
# Monitor AI decision making and learning
```

### Database Problems
```bash
sqlx migrate info          # Check migration status
sqlite3 storage.db           # Direct database access
```

### Frontend Debugging
- Use browser DevTools
- Check network tab for API calls
- Monitor AI status endpoints
- Enable Dioxus debug mode

## Production Deployment

### Environment Variables
```
DATABASE_URL=postgresql://...
REDIS_URL=redis://...
JWT_SECRET=<generated>
API_URL=https://api.domain.com
SEARXNG_URL=https://searxng.domain.com
ADMIN_SECRET=<generated>
```

### Docker Commands
```bash
docker-compose -f docker-compose.prod.yml build
docker-compose -f docker-compose.prod.yml up -d
docker-compose -f docker-compose.prod.yml logs -f
```

## Error Handling

Always use proper error types:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("AI Agent error: {0}")]
    AiAgent(String),
    
    #[error("Evaluation failed: {0}")]
    Evaluation(String),
    
    #[error("Not found")]
    NotFound,
}
```

## Git Workflow

1. Create feature branch: `git checkout -b feature/name`
2. Make changes and test
3. Test AI functionality thoroughly
4. Commit with clear messages
5. Push and create PR
6. Ensure CI passes

### Commit Guidelines for Claude Code

**IMPORTANT**: Claude Code must create a commit after completing any of the following:

1. **Major Updates**: New features, significant functionality additions
2. **AI Improvements**: Changes to AI agent, evaluation engine, or learning algorithms
3. **Refactors**: Code restructuring, architecture changes, module reorganization
4. **Bug Fixes**: Resolving issues or fixing broken functionality
5. **Configuration Changes**: Database migrations, dependency updates, config modifications
6. **Documentation Updates**: Significant changes to README, API docs, or project documentation

### Commit Message Format
```
<type>: <description>

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

**Types:**
- `feat`: New feature
- `ai`: AI/ML improvements
- `fix`: Bug fix
- `refactor`: Code restructuring without functionality changes
- `docs`: Documentation updates
- `config`: Configuration or dependency changes
- `test`: Adding or updating tests
- `perf`: Performance improvements

**Examples:**
```
ai: implement reinforcement learning for intelligent data gathering

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

```
feat: add comprehensive data evaluation engine with German validation

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

## Additional Notes

- **AI-First Design**: All new features should consider AI integration
- Keep dependencies minimal and well-maintained
- Prefer standard library over external crates
- Document AI algorithms and learning strategies thoroughly
- Monitor AI performance metrics continuously
- Use semantic versioning
- Regular security audits with `cargo audit`

Remember: **AI-driven simplicity is key**. The AI should make complex tasks simple, not add complexity.