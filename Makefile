.PHONY: help setup dev build test lint clean api crawler frontend docs docker

# Default target
help:
	@echo "DNO Data Gatherer - Development Commands"
	@echo ""
	@echo "Setup Commands:"
	@echo "  setup     - Install dependencies and setup database"
	@echo "  migrate   - Run database migrations"
	@echo ""
	@echo "Development Commands:"
	@echo "  dev       - Start all services in development mode"
	@echo "  api       - Run API server only"
	@echo "  crawler   - Run crawler CLI"
	@echo "  frontend  - Run frontend development server"
	@echo ""
	@echo "Build Commands:"
	@echo "  build     - Build all crates (debug)"
	@echo "  release   - Build all crates (release)"
	@echo ""
	@echo "Quality Commands:"
	@echo "  test      - Run all tests"
	@echo "  lint      - Run clippy and format check"
	@echo "  fmt       - Format code"
	@echo ""
	@echo "Utility Commands:"
	@echo "  clean     - Clean build artifacts"
	@echo "  docs      - Generate documentation"
	@echo "  mock      - Generate mock data"

# Setup and installation
setup:
	@echo "🔧 Setting up development environment..."
	rustup update stable
	cargo install cargo-watch sqlx-cli
	cp .env.example .env
	@echo "✅ Setup complete! Edit .env file and run 'make migrate'"

migrate:
	@echo "🗄️ Running database migrations..."
	sqlx database create
	sqlx migrate run
	@echo "✅ Migrations complete!"

# Development commands
dev:
	@echo "🚀 Starting development environment..."
	cargo watch -x "run --bin api"

api:
	@echo "🔥 Starting API server..."
	cargo run --bin api

crawler:
	@echo "🕷️ Starting crawler CLI..."
	cargo run --bin crawler -- --help

frontend:
	@echo "🎨 Starting frontend development..."
	cd crates/frontend && cargo run

# Build commands
build:
	@echo "🔨 Building all crates (debug)..."
	cargo build --workspace

release:
	@echo "🔨 Building all crates (release)..."
	cargo build --workspace --release

# Quality commands
test:
	@echo "🧪 Running tests..."
	cargo test --workspace

lint:
	@echo "🔍 Running clippy..."
	cargo clippy --workspace --all-targets --all-features -- -D warnings
	@echo "🎨 Checking formatting..."
	cargo fmt --all -- --check

fmt:
	@echo "🎨 Formatting code..."
	cargo fmt --all

# Utility commands
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

docs:
	@echo "📚 Generating documentation..."
	cargo doc --workspace --no-deps --open

mock:
	@echo "🎭 Generating mock data..."
	cargo run --bin crawler -- mock

# Docker commands (for future use)
docker-build:
	@echo "🐳 Building Docker images..."
	docker-compose build

docker-up:
	@echo "🐳 Starting services with Docker..."
	docker-compose up -d

docker-down:
	@echo "🐳 Stopping Docker services..."
	docker-compose down

docker-logs:
	@echo "🐳 Showing Docker logs..."
	docker-compose logs -f