# AI-Powered DNO Data Gatherer 🤖🌿

A cutting-edge Rust application that uses artificial intelligence to automatically gather and process data from German Distribution Network Operators (DNOs). Built with reinforcement learning for intelligent, self-improving data collection.

## Features ✨

- 🧠 **AI-Driven Intelligence**: Reinforcement learning agent that adapts and improves data gathering strategies
- 🎯 **Smart Evaluation**: Comprehensive data quality assessment with German-specific validation
- 🔍 **Intelligent Search**: AI-optimized SearXNG integration for privacy-respecting web searches
- 📊 **Self-Learning**: Continuous improvement through success/failure pattern recognition
- 🚀 **Lightning Fast**: Built with Rust for maximum performance
- 🔐 **Enterprise Security**: JWT authentication with role-based access control
- 📱 **Unified Interface**: Single codebase for web and mobile using Dioxus
- 🎨 **Beautiful UI**: Dark theme with AI status indicators and real-time metrics
- 📈 **Full Metrics**: Comprehensive admin dashboard with AI performance analytics

## AI System Overview 🧠

### Intelligent Gathering Agent
- **Reinforcement Learning**: Uses epsilon-greedy exploration to balance trying new strategies vs proven methods
- **DNO-Specific Adaptation**: Learns unique patterns for each Distribution Network Operator
- **Quality Optimization**: Continuously improves data quality and completeness scores
- **Pattern Recognition**: Identifies successful URL patterns and content indicators

### Data Evaluation Engine
- **German Data Validation**: Specialized rules for German addresses, phone numbers, emails
- **Completeness Scoring**: Ensures all required Netzentgelte and HLZF fields are found
- **Quality Metrics**: Real-time assessment of data accuracy and format compliance
- **Intelligent Recommendations**: AI-generated suggestions for improvement

## Quick Start 🏃

### Prerequisites

- Rust (latest stable)
- SQLite (PostgreSQL for production)
- Node.js (for Tailwind CSS)
- SearXNG instance (required for AI agent)
- Docker (optional, for production setup)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-dno-crawler.git
cd rust-dno-crawler

# Install dependencies
rustup update stable
cargo install cargo-watch sqlx-cli

# Setup database
export DATABASE_URL="sqlite://data.db"
sqlx database create
sqlx migrate run

# Setup SearXNG (required for AI)
export SEARXNG_URL="http://localhost:8080"

# Start development server
cargo watch -x "run --bin api"
```

### AI Crawler Usage

```bash
# Primary AI-driven data gathering
cargo run --bin crawler -- ai-gather "Netze BW" --data-types "netzentgelte,hlzf" --json

# Test SearXNG connectivity
cargo run --bin crawler -- search "test query" --json

# Get help and options
cargo run --bin crawler -- ai-gather --help
```

## Architecture 🏗️

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│   Dioxus Web/   │────▶│   Axum API      │────▶│   SQLite/       │
│   Mobile UI     │     │   Server        │     │   PostgreSQL    │
│   + AI Metrics  │     │   + AI Routes   │     │   + AI Models   │
└─────────────────┘     └─────────────────┘     └─────────────────┘
                               │
                        ┌──────┼──────┐
                        │      │      │
                 ┌──────▼──┐ ┌─▼─┐ ┌──▼────────┐
                 │   AI    │ │API│ │  Admin    │
                 │ Crawler │ │   │ │Dashboard  │
                 │ Agent   │ │   │ │+ Metrics  │
                 └──┬──────┘ └───┘ └───────────┘
                    │
         ┌──────────┼──────────┐
         │          │          │
    ┌────▼────┐ ┌──▼──┐ ┌─────▼─────┐
    │SearXNG  │ │ AI  │ │Evaluation │
    │ Search  │ │Model│ │  Engine   │
    │ Engine  │ │     │ │           │
    └─────────┘ └─────┘ └───────────┘
```

## Website Pages 📱

1. **Homepage** - Overview and quick access to AI crawling
2. **Login/Register** - User authentication
3. **User Dashboard** - Personal data access and crawl job management (user auth)
4. **Admin User Management** - User administration (admin auth)
5. **Admin Crawler Management** - AI metrics, learning status, full crawler control (admin auth)

## API Endpoints 🔌

### Authentication
- `POST /auth/login` - User login
- `POST /auth/register` - User registration
- `POST /auth/refresh` - Token refresh
- `POST /auth/logout` - User logout

### AI-Driven Crawling
- `POST /ai/crawl` - Intelligent AI crawling
- `POST /ai/query` - Natural language queries in German
- `GET /ai/status/:dno` - AI agent status and performance metrics
- `POST /ai/retrain/:dno` - Retrain AI model for specific DNO
- `GET /ai/evaluation/:dno` - Get comprehensive data evaluation report

### DNO Management
- `GET /dnos/` - List all DNOs
- `POST /dnos/` - Create new DNO
- `GET /dnos/:id` - Get specific DNO with AI metrics
- `PUT /dnos/:id` - Update DNO
- `DELETE /dnos/:id` - Delete DNO

### Search & Data
- `POST /search/` - Perform intelligent search
- `GET /search/history` - Search history with AI insights
- `POST /search/gather` - AI-enhanced data gathering
- `POST /dno/query` - Query DNO data with AI assistance
- `POST /dno/analyze` - AI-powered PDF analysis
- `GET /dno/stats` - AI learning statistics

### Admin Dashboard (Admin Auth Required)
- `GET /admin/dashboard` - Admin dashboard with full AI metrics
- `GET /admin/flags` - Data quality flags and AI recommendations
- `POST /admin/flags` - Flag data issues for AI improvement
- `GET /admin/patterns` - AI pattern verification and learning status
- `GET /admin/sources` - Source management with AI quality scores
- `GET /admin/audit` - Comprehensive audit logs including AI decisions

### Job Management (Admin Auth Required)
- `GET /jobs/` - Get automated AI jobs
- `POST /jobs/` - Create new automated AI job
- `PUT /jobs/:job_id` - Update job parameters
- `DELETE /jobs/:job_id` - Delete automated job
- `POST /jobs/:job_id/control` - Control AI job execution
- `GET /jobs/:job_id/history` - AI job execution history with metrics
- `GET /jobs/system/status` - System status including AI performance

## Configuration ⚙️

Create a `.env` file in the root directory:

```env
# Database
DATABASE_URL=sqlite://data.db

# API Server
API_PORT=8080
JWT_SECRET=your-secret-key

# SearXNG (Required for AI agent)
SEARXNG_URL=http://localhost:8080

# Admin Access
ADMIN_SECRET=your-admin-secret

# Optional: AI Model Storage Path
AI_MODEL_PATH=./ai_models/

# Frontend
VITE_API_URL=http://localhost:8080
```

## AI Performance Metrics 📊

The system tracks comprehensive AI performance:

- **Learning Rate**: How quickly the AI adapts to new patterns
- **Success Rate**: Percentage of successful data gathering attempts
- **Quality Score**: Average data quality across all operations
- **Completeness Score**: How completely required fields are found
- **DNO-Specific Performance**: Individual metrics for each operator
- **Pattern Recognition**: Success rates for different URL patterns
- **Exploration vs Exploitation**: Balance between trying new strategies and using proven ones

## Development 👩‍💻

### Available Commands

```bash
# Core development
cargo build                    # Build debug version
cargo build --release          # Build optimized release
cargo test                     # Run all tests including AI tests
cargo run --bin api           # Start API server
cargo run --bin crawler       # Run AI crawler CLI

# AI-specific commands
cargo run --bin crawler -- ai-gather --help  # AI crawler help
RUST_LOG=crawler::ai_agent=debug cargo run --bin crawler -- ai-gather "Test DNO"  # Debug AI decisions

# Database
sqlx migrate run              # Apply migrations
sqlx migrate revert           # Revert migrations
```

### Project Structure

- `/crates/api` - Axum REST API server with AI endpoints
- `/crates/crawler` - AI-driven intelligent crawler with reinforcement learning
- `/crates/frontend` - Dioxus web/mobile application with AI status displays
- `/crates/shared` - Shared types including AI models and evaluation structures

## AI Development Guidelines 🧠

### Improving AI Performance
1. Monitor success rates and quality scores in admin dashboard
2. Adjust exploration rate in `ai_agent.rs` based on performance
3. Add new validation rules in `evaluation_engine.rs` for better quality assessment
4. Train on more DNOs to improve generalization
5. Update reward functions based on user feedback

### Testing AI Features
```bash
# Test AI with different DNOs
cargo run --bin crawler -- ai-gather "Netze BW" --json --max-time 30
cargo run --bin crawler -- ai-gather "Westnetz" --json --priority speed
cargo run --bin crawler -- ai-gather "Bayernwerk" --data-types "hlzf,contact" --json

# Monitor AI learning
curl http://localhost:8080/api/ai/status/netze_bw
curl http://localhost:8080/api/ai/evaluation/westnetz
```

## Deployment 🚀

### Production Environment Variables
```env
DATABASE_URL=postgresql://user:pass@localhost/dnodb
REDIS_URL=redis://localhost:6379
SEARXNG_URL=https://searxng.yourdomain.com
JWT_SECRET=<generated-secret>
ADMIN_SECRET=<generated-admin-secret>
API_URL=https://api.yourdomain.com
```

### Using Docker Compose

```bash
# Development with AI
docker-compose up

# Production with full AI stack
docker-compose -f docker-compose.prod.yml up -d
```

## Security 🔐

- **JWT Authentication**: Short-lived tokens (15min) with secure refresh
- **Role-Based Access**: User and admin roles with appropriate permissions
- **Admin Protection**: Special authentication for admin routes
- **Input Validation**: Strong typing and validation for all AI inputs
- **Rate Limiting**: Protection against AI abuse and resource exhaustion

## Contributing 🤝

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/ai-improvement`)
3. Test AI functionality thoroughly
4. Commit your changes with clear AI-related descriptions
5. Push to the branch (`git push origin feature/ai-improvement`)
6. Open a Pull Request with AI performance metrics

### AI Development Guidelines
- Always test new AI features with multiple DNOs
- Monitor performance impact of AI changes
- Document any new learning algorithms or evaluation criteria
- Include AI metrics in pull request descriptions

## License 📄

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE.md) file for details.

## Acknowledgments 🙏

- Built with love for KyleDerZweite and German friends 🇩🇪
- Powered by the amazing Rust ecosystem 🦀
- AI algorithms inspired by latest reinforcement learning research 🧠
- Special thanks to the Dioxus, Axum, and machine learning communities 🙏

---

**Need help with AI features?** Check the admin dashboard for performance metrics or open an issue with your AI logs! 🤖✨