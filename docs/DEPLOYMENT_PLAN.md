# DNO Crawler - Production Deployment Plan

## Overview

This document outlines the complete deployment strategy for the DNO (Distribution Network Operator) crawler system. The architecture is designed for scalability, maintainability, and production readiness with full Docker containerization.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                          Load Balancer                          │
│                        (Nginx/Traefik)                          │
└─────────────────────────┬───────────────────────────────────────┘
                          │
┌─────────────────────────┼───────────────────────────────────────┐
│                    Docker Network                               │
│                                                                 │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │   API Server    │  │  Website Server │  │ Crawler Service │  │
│  │    (Axum)       │  │    (Dioxus)     │  │   (Background)  │  │
│  │   Port: 3000    │  │   Port: 8000    │  │   (No expose)   │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
│           │                      │                      │       │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │   PostgreSQL    │  │      Redis      │  │     SearXNG     │  │
│  │   Port: 5432    │  │   Port: 6379    │  │   Port: 8080    │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                    Shared Storage                           ││
│  │           (DNO Data, PDFs, Cache, Logs)                     ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

## Service Components

### 1. API Server (api-server)
- **Technology**: Rust + Axum
- **Purpose**: RESTful API backend for all data operations
- **Port**: 3000 (internal), 80/443 (external via load balancer)
- **Features**:
  - JWT authentication with role-based access
  - Rate limiting and CORS handling
  - Database operations via SQLx
  - Redis caching for performance
  - Prometheus metrics exposure
  - WebSocket support for real-time updates

### 2. Website Server (website-server)
- **Technology**: Rust + Dioxus
- **Purpose**: Frontend web application
- **Port**: 8000 (internal), 80/443 (external via load balancer)
- **Features**:
  - Natural language query processing
  - Interactive dashboard and data visualization
  - User authentication and account management
  - Admin panel for data verification
  - Mobile-responsive design

### 3. Crawler Service (crawler-service)
- **Technology**: Rust + AI agents
- **Purpose**: Intelligent data gathering from DNO sources
- **Port**: None (background service)
- **Features**:
  - AI-driven search strategy optimization
  - Reinforcement learning for improved accuracy
  - Automated PDF processing and data extraction
  - Quality evaluation and validation
  - Job queue management
  - SearXNG integration for web searches

### 4. Database Services
- **PostgreSQL**: Primary data storage
- **Redis**: Caching, session storage, and job queues
- **SearXNG**: Search engine for web crawling

### 5. Load Balancer & Reverse Proxy
- **Technology**: Nginx or Traefik
- **Purpose**: SSL termination, routing, and load balancing
- **Features**:
  - SSL/TLS certificate management
  - Request routing based on paths
  - Static file serving
  - Rate limiting and security headers

## Docker Architecture

### Production Docker Compose Structure

```yaml
version: '3.11'

networks:
  dno-network:
    driver: bridge
    ipam:
      config:
        - subnet: 172.20.0.0/16

volumes:
  postgres_data:
  redis_data:
  searxng_data:
  storage_data:
  logs_data:

services:
  # Infrastructure Services
  postgres:
    image: postgres:16-alpine
    environment:
      - POSTGRES_DB=${POSTGRES_DB}
      - POSTGRES_USER=${POSTGRES_USER}
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./migrations:/docker-entrypoint-initdb.d
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${POSTGRES_USER}"]
      interval: 10s
      timeout: 5s
      retries: 5
    networks:
      - dno-network

  redis:
    image: redis:7-alpine
    command: redis-server --appendonly yes --requirepass ${REDIS_PASSWORD}
    volumes:
      - redis_data:/data
    healthcheck:
      test: ["CMD", "redis-cli", "-a", "${REDIS_PASSWORD}", "ping"]
      interval: 10s
      timeout: 5s
      retries: 5
    networks:
      - dno-network

  searxng:
    image: searxng/searxng:latest
    environment:
      - SEARXNG_BASE_URL=${SEARXNG_BASE_URL}
      - SEARXNG_REDIS_URL=redis://default:${REDIS_PASSWORD}@redis:6379/1
    volumes:
      - searxng_data:/etc/searxng
    depends_on:
      redis:
        condition: service_healthy
    networks:
      - dno-network

  # Application Services
  api-server:
    build:
      context: .
      dockerfile: docker/Dockerfile.api
      target: production
    environment:
      - DATABASE_URL=postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres:5432/${POSTGRES_DB}
      - REDIS_URL=redis://dno_user:${REDIS_PASSWORD}@redis:6379/0
      - JWT_SECRET=${JWT_SECRET}
      - RUST_LOG=info
      - SERVER_HOST=0.0.0.0
      - SERVER_PORT=3000
    volumes:
      - storage_data:/app/storage
      - logs_data:/app/logs
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
      interval: 30s
      timeout: 10s
      retries: 3
    networks:
      - dno-network

  website-server:
    build:
      context: .
      dockerfile: docker/Dockerfile.website
      target: production
    environment:
      - API_URL=http://api-server:3000
      - RUST_LOG=info
      - SERVER_HOST=0.0.0.0
      - SERVER_PORT=8000
    depends_on:
      api-server:
        condition: service_healthy
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8000/health"]
      interval: 30s
      timeout: 10s
      retries: 3
    networks:
      - dno-network

  crawler-service:
    build:
      context: .
      dockerfile: docker/Dockerfile.crawler
      target: production
    environment:
      - DATABASE_URL=postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres:5432/${POSTGRES_DB}
      - REDIS_URL=redis://dno_user:${REDIS_PASSWORD}@redis:6379/0
      - SEARXNG_URL=http://searxng:8080
      - RUST_LOG=info
      - CRAWLER_WORKERS=3
      - CRAWLER_INTERVAL=3600
    volumes:
      - storage_data:/app/storage
      - logs_data:/app/logs
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy
      searxng:
        condition: service_started
    restart: unless-stopped
    networks:
      - dno-network

  # Load Balancer
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./docker/nginx/nginx.conf:/etc/nginx/nginx.conf
      - ./docker/nginx/ssl:/etc/nginx/ssl
      - storage_data:/var/www/storage:ro
    depends_on:
      - api-server
      - website-server
    networks:
      - dno-network
```

## Dockerfile Specifications

### 1. API Server Dockerfile (`docker/Dockerfile.api`)

```dockerfile
# Build stage
FROM rust:1.75-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconfig \
    curl

# Set working directory
WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

# Build dependencies first (better caching)
RUN mkdir -p crates/api/src && echo "fn main() {}" > crates/api/src/main.rs
RUN mkdir -p crates/core/src && echo "" > crates/core/src/lib.rs
RUN cargo build --release --bin api
RUN rm crates/api/src/main.rs crates/core/src/lib.rs

# Build actual application
COPY crates/api/src/ ./crates/api/src/
COPY crates/core/src/ ./crates/core/src/
RUN touch crates/api/src/main.rs crates/core/src/lib.rs
RUN cargo build --release --bin api

# Runtime stage
FROM alpine:3.19 AS production

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    curl \
    openssl \
    libgcc

# Create app user
RUN addgroup -g 1001 -S appgroup && \
    adduser -u 1001 -S appuser -G appgroup

# Set working directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/api /app/api

# Create required directories
RUN mkdir -p /app/storage /app/logs && \
    chown -R appuser:appgroup /app

# Switch to non-root user
USER appuser

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Expose port
EXPOSE 3000

# Start application
CMD ["./api"]
```

### 2. Website Server Dockerfile (`docker/Dockerfile.website`)

```dockerfile
# Build stage
FROM rust:1.75-alpine AS builder

# Install dependencies
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig \
    curl \
    nodejs \
    npm

WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

# Install Node.js dependencies for Dioxus
COPY package.json package-lock.json ./
RUN npm install

# Build dependencies
RUN mkdir -p crates/website/src && echo "fn main() {}" > crates/website/src/main.rs
RUN mkdir -p crates/core/src && echo "" > crates/core/src/lib.rs
RUN cargo build --release --bin website
RUN rm crates/website/src/main.rs crates/core/src/lib.rs

# Build actual application
COPY crates/website/src/ ./crates/website/src/
COPY crates/core/src/ ./crates/core/src/
RUN touch crates/website/src/main.rs crates/core/src/lib.rs
RUN cargo build --release --bin website

# Runtime stage
FROM alpine:3.19 AS production

RUN apk add --no-cache \
    ca-certificates \
    curl \
    libgcc

RUN addgroup -g 1001 -S appgroup && \
    adduser -u 1001 -S appuser -G appgroup

WORKDIR /app

COPY --from=builder /app/target/release/website /app/website
COPY --from=builder /app/crates/website/assets /app/assets

RUN chown -R appuser:appgroup /app

USER appuser

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

EXPOSE 8000

CMD ["./website"]
```

### 3. Crawler Service Dockerfile (`docker/Dockerfile.crawler`)

```dockerfile
FROM rust:1.75-alpine AS builder

RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig \
    curl

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

# Build dependencies
RUN mkdir -p crates/crawler/src && echo "fn main() {}" > crates/crawler/src/main.rs
RUN mkdir -p crates/core/src && echo "" > crates/core/src/lib.rs
RUN cargo build --release --bin crawler
RUN rm crates/crawler/src/main.rs crates/core/src/lib.rs

# Build actual application
COPY crates/crawler/src/ ./crates/crawler/src/
COPY crates/core/src/ ./crates/core/src/
RUN touch crates/crawler/src/main.rs crates/core/src/lib.rs
RUN cargo build --release --bin crawler

# Runtime stage
FROM alpine:3.19 AS production

RUN apk add --no-cache \
    ca-certificates \
    curl \
    libgcc

RUN addgroup -g 1001 -S appgroup && \
    adduser -u 1001 -S appuser -G appgroup

WORKDIR /app

COPY --from=builder /app/target/release/crawler /app/crawler

RUN mkdir -p /app/storage /app/logs && \
    chown -R appuser:appgroup /app

USER appuser

# No health check needed for background service

CMD ["./crawler", "daemon"]
```

## Configuration Management

### Environment Variables

Create a comprehensive `.env.production` file:

```env
# Database Configuration
POSTGRES_DB=dno_crawler_prod
POSTGRES_USER=dno_user
POSTGRES_PASSWORD=your_secure_db_password_here

# Redis Configuration  
REDIS_PASSWORD=your_secure_redis_password_here

# Application Secrets
JWT_SECRET=your_super_secure_jwt_secret_256_bit_key_here
ADMIN_SECRET=your_secure_admin_secret_here

# API Configuration
API_URL=https://api.yourdomain.com
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
CORS_ORIGINS=https://yourdomain.com,https://www.yourdomain.com

# Website Configuration
WEBSITE_URL=https://yourdomain.com

# External Services
SEARXNG_BASE_URL=http://searxng:8080
SEARXNG_REDIS_URL=redis://default:your_secure_redis_password_here@redis:6379/1

# Storage Configuration
STORAGE_PATH=/app/storage
TEMP_PATH=/app/temp
UPLOAD_MAX_SIZE=52428800

# Security Configuration
RATE_LIMIT_PER_MINUTE=60
RATE_LIMIT_PER_HOUR=1000

# JWT Configuration
JWT_ACCESS_TOKEN_EXPIRY=3600
JWT_REFRESH_TOKEN_EXPIRY=2592000

# Crawler Configuration
CRAWLER_WORKERS=3
CRAWLER_INTERVAL=3600
CRAWLER_MAX_CONCURRENT_JOBS=5

# Logging
RUST_LOG=info
```

### Production Nginx Configuration (`docker/nginx/nginx.conf`)

```nginx
events {
    worker_connections 1024;
}

http {
    upstream api_backend {
        server api-server:3000;
    }

    upstream website_backend {
        server website-server:8000;
    }

    # Rate limiting
    limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;
    limit_req_zone $binary_remote_addr zone=general:10m rate=30r/s;

    # SSL Configuration
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES128-GCM-SHA256:ECDHE-RSA-AES256-GCM-SHA384;
    ssl_prefer_server_ciphers off;

    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;

    # Main server block
    server {
        listen 80;
        listen 443 ssl http2;
        server_name yourdomain.com www.yourdomain.com;

        # SSL certificates
        ssl_certificate /etc/nginx/ssl/fullchain.pem;
        ssl_certificate_key /etc/nginx/ssl/privkey.pem;

        # Redirect HTTP to HTTPS
        if ($scheme != "https") {
            return 301 https://$server_name$request_uri;
        }

        # API routes
        location /api/ {
            limit_req zone=api burst=20 nodelay;
            proxy_pass http://api_backend;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
            proxy_connect_timeout 30s;
            proxy_send_timeout 30s;
            proxy_read_timeout 30s;
        }

        # WebSocket for real-time updates
        location /ws {
            proxy_pass http://api_backend;
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
        }

        # Static files (storage access)
        location /storage/ {
            alias /var/www/storage/;
            expires 1h;
            add_header Cache-Control "public, immutable";
        }

        # Website routes
        location / {
            limit_req zone=general burst=50 nodelay;
            proxy_pass http://website_backend;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }

        # Health checks
        location /health {
            access_log off;
            return 200 "healthy\n";
            add_header Content-Type text/plain;
        }
    }
}
```

## Build and Deployment Process

### 1. Development Build Script (`scripts/build-dev.sh`)

```bash
#!/bin/bash
set -e

echo "Building DNO Crawler for development..."

# Build all components
cargo build --workspace

# Build website assets
cd crates/website/js_website_example
npm install
npm run build
cd ../../..

echo "Development build completed!"
```

### 2. Production Build Script (`scripts/build-prod.sh`)

```bash
#!/bin/bash
set -e

echo "Building DNO Crawler for production..."

# Ensure we're in the project root
cd "$(dirname "$0")/.."

# Build Docker images
echo "Building API server image..."
docker build -f docker/Dockerfile.api -t dno-crawler/api:latest .

echo "Building website server image..."
docker build -f docker/Dockerfile.website -t dno-crawler/website:latest .

echo "Building crawler service image..."
docker build -f docker/Dockerfile.crawler -t dno-crawler/crawler:latest .

echo "Production build completed!"
```

### 3. Database Migration Script (`scripts/migrate.sh`)

```bash
#!/bin/bash
set -e

# Check if environment is provided
if [ -z "$1" ]; then
    echo "Usage: $0 <environment>"
    echo "Environment: dev, staging, prod"
    exit 1
fi

ENV=$1
ENV_FILE=".env.${ENV}"

if [ ! -f "$ENV_FILE" ]; then
    echo "Environment file $ENV_FILE not found!"
    exit 1
fi

# Load environment variables
export $(cat $ENV_FILE | grep -v '^#' | xargs)

echo "Running migrations for $ENV environment..."

# Install sqlx-cli if not present
if ! command -v sqlx &> /dev/null; then
    cargo install sqlx-cli --no-default-features --features postgres
fi

# Create database if it doesn't exist
sqlx database create

# Run migrations
sqlx migrate run

echo "Migrations completed successfully!"
```

### 4. Deployment Script (`scripts/deploy.sh`)

```bash
#!/bin/bash
set -e

# Configuration
ENV=${1:-prod}
ENV_FILE=".env.${ENV}"

if [ ! -f "$ENV_FILE" ]; then
    echo "Environment file $ENV_FILE not found!"
    exit 1
fi

echo "Deploying DNO Crawler to $ENV environment..."

# Load environment variables
export $(cat $ENV_FILE | grep -v '^#' | xargs)

# Build production images
./scripts/build-prod.sh

# Run database migrations
./scripts/migrate.sh $ENV

# Stop existing containers
docker-compose -f docker-compose.yml -f docker-compose.${ENV}.yml down

# Start services
docker-compose -f docker-compose.yml -f docker-compose.${ENV}.yml up -d

# Wait for services to be healthy
echo "Waiting for services to be healthy..."
sleep 30

# Check service health
./scripts/health-check.sh

echo "Deployment completed successfully!"
```

### 5. Health Check Script (`scripts/health-check.sh`)

```bash
#!/bin/bash

# Load environment
ENV=${1:-prod}
ENV_FILE=".env.${ENV}"

if [ -f "$ENV_FILE" ]; then
    export $(cat $ENV_FILE | grep -v '^#' | xargs)
fi

API_URL=${API_URL:-http://localhost:3000}
WEBSITE_URL=${WEBSITE_URL:-http://localhost:8000}

echo "Checking service health..."

# Check API health
echo -n "API Server: "
if curl -s -f "$API_URL/health" > /dev/null; then
    echo "✅ Healthy"
else
    echo "❌ Unhealthy"
    exit 1
fi

# Check website health
echo -n "Website Server: "
if curl -s -f "$WEBSITE_URL/health" > /dev/null; then
    echo "✅ Healthy"
else
    echo "❌ Unhealthy"
    exit 1
fi

# Check database connectivity
echo -n "Database: "
if docker-compose exec -T postgres pg_isready -U $POSTGRES_USER -d $POSTGRES_DB > /dev/null 2>&1; then
    echo "✅ Healthy"
else
    echo "❌ Unhealthy"
    exit 1
fi

# Check Redis connectivity
echo -n "Redis: "
if docker-compose exec -T redis redis-cli -a $REDIS_PASSWORD ping > /dev/null 2>&1; then
    echo "✅ Healthy"
else
    echo "❌ Unhealthy"
    exit 1
fi

echo "All services are healthy! 🎉"
```

## Crawler Service Implementation Plan

### Architecture Overview

The crawler service will be implemented as a background daemon with the following components:

```rust
// crates/crawler/src/main.rs
use clap::{Parser, Subcommand};
use crawler::{CrawlerService, CrawlerConfig};

#[derive(Parser)]
#[command(name = "crawler")]
#[command(about = "DNO Data Crawler Service")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run crawler as daemon
    Daemon {
        /// Number of worker threads
        #[arg(short, long, default_value = "3")]
        workers: usize,
    },
    /// Run single crawl job
    Crawl {
        /// DNO name to crawl
        dno: String,
        /// Year to crawl
        year: i32,
        /// Data types to crawl
        #[arg(long, default_values = ["netzentgelte", "hlzf"])]
        data_types: Vec<String>,
    },
    /// Management commands
    Status,
    Health,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Daemon { workers } => {
            let config = CrawlerConfig::from_env()?;
            let service = CrawlerService::new(config).await?;
            service.start_daemon(workers).await?;
        }
        Commands::Crawl { dno, year, data_types } => {
            let config = CrawlerConfig::from_env()?;
            let service = CrawlerService::new(config).await?;
            service.crawl_single(dno, year, data_types).await?;
        }
        Commands::Status => {
            // Show crawler status
        }
        Commands::Health => {
            // Health check
        }
    }
    
    Ok(())
}
```

### Key Features to Implement

1. **Job Queue Management**
   - Redis-based job queue
   - Priority-based scheduling
   - Retry logic with exponential backoff
   - Dead letter queue for failed jobs

2. **AI-Driven Search Strategy**
   - Reinforcement learning for search optimization
   - Success pattern recognition
   - Adaptive search term generation
   - Quality score feedback loop

3. **Data Extraction Pipeline**
   - PDF processing with OCR fallback
   - Table detection and extraction
   - Data validation and cleaning
   - Confidence scoring

4. **Monitoring and Metrics**
   - Prometheus metrics export
   - Detailed logging with structured data
   - Performance tracking
   - Error rate monitoring

## Monitoring and Logging

### Prometheus Metrics

```rust
// crates/core/src/metrics.rs
use prometheus::{Counter, Histogram, Gauge, Registry};

pub struct Metrics {
    pub api_requests: Counter,
    pub api_request_duration: Histogram,
    pub active_connections: Gauge,
    pub crawler_jobs_total: Counter,
    pub crawler_job_duration: Histogram,
    pub database_connections: Gauge,
    pub cache_hits: Counter,
    pub cache_misses: Counter,
}

impl Metrics {
    pub fn new() -> Result<Self, prometheus::Error> {
        Ok(Self {
            api_requests: Counter::new("api_requests_total", "Total API requests")?,
            api_request_duration: Histogram::new("api_request_duration_seconds", "API request duration")?,
            active_connections: Gauge::new("active_connections", "Active connections")?,
            crawler_jobs_total: Counter::new("crawler_jobs_total", "Total crawler jobs")?,
            crawler_job_duration: Histogram::new("crawler_job_duration_seconds", "Crawler job duration")?,
            database_connections: Gauge::new("database_connections", "Database connections")?,
            cache_hits: Counter::new("cache_hits_total", "Cache hits")?,
            cache_misses: Counter::new("cache_misses_total", "Cache misses")?,
        })
    }
    
    pub fn register(&self, registry: &Registry) -> Result<(), prometheus::Error> {
        registry.register(Box::new(self.api_requests.clone()))?;
        registry.register(Box::new(self.api_request_duration.clone()))?;
        registry.register(Box::new(self.active_connections.clone()))?;
        registry.register(Box::new(self.crawler_jobs_total.clone()))?;
        registry.register(Box::new(self.crawler_job_duration.clone()))?;
        registry.register(Box::new(self.database_connections.clone()))?;
        registry.register(Box::new(self.cache_hits.clone()))?;
        registry.register(Box::new(self.cache_misses.clone()))?;
        Ok(())
    }
}
```

### Logging Configuration

```rust
// crates/core/src/logging.rs
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true)
                .json()
        )
        .init();
    
    Ok(())
}
```

## Security Considerations

### 1. Authentication & Authorization
- JWT tokens with short expiry (1 hour)
- Refresh tokens with secure rotation
- Role-based access control (pending, user, admin)
- API key authentication for service-to-service communication

### 2. Data Protection
- Encryption at rest for sensitive data
- TLS encryption for all communications
- Input validation and sanitization
- SQL injection prevention via SQLx

### 3. Network Security
- Private Docker network
- No direct database access from outside
- Rate limiting on API endpoints
- Security headers in Nginx

### 4. Container Security
- Non-root user in containers
- Minimal base images (Alpine Linux)
- Regular security updates
- Health checks for all services

## Performance Optimization

### 1. Database Optimization
- Connection pooling with SQLx
- Read replicas for analytics queries
- Proper indexing strategy
- Query optimization

### 2. Caching Strategy
- Redis for session storage
- API response caching
- Database query result caching
- Static file caching via Nginx

### 3. Load Balancing
- Multiple API server instances
- Health check-based routing
- Connection keep-alive
- Request distribution

## Backup and Recovery

### 1. Database Backups
```bash
# Daily backup script
#!/bin/bash
BACKUP_DIR="/backups/postgres"
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="$BACKUP_DIR/dno_crawler_$DATE.sql"

mkdir -p $BACKUP_DIR

docker-compose exec -T postgres pg_dump -U $POSTGRES_USER $POSTGRES_DB > $BACKUP_FILE

# Compress backup
gzip $BACKUP_FILE

# Keep only last 30 days
find $BACKUP_DIR -name "*.sql.gz" -mtime +30 -delete
```

### 2. Storage Backups
```bash
# File storage backup
#!/bin/bash
STORAGE_DIR="/app/storage"
BACKUP_DIR="/backups/storage"
DATE=$(date +%Y%m%d_%H%M%S)

rsync -avz --delete $STORAGE_DIR/ $BACKUP_DIR/current/
tar -czf $BACKUP_DIR/storage_$DATE.tar.gz -C $BACKUP_DIR current/

# Keep only last 7 days for full backups
find $BACKUP_DIR -name "storage_*.tar.gz" -mtime +7 -delete
```

## Continuous Integration/Continuous Deployment

### GitHub Actions Workflow (`.github/workflows/deploy.yml`)

```yaml
name: Deploy to Production

on:
  push:
    branches: [main]
  workflow_dispatch:

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      - name: Run tests
        run: cargo test --workspace
      - name: Run clippy
        run: cargo clippy -- -D warnings

  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v2
      - name: Login to DockerHub
        uses: docker/login-action@v2
        with:
          username: ${{ secrets.DOCKERHUB_USERNAME }}
          password: ${{ secrets.DOCKERHUB_TOKEN }}
      - name: Build and push images
        run: |
          docker buildx build --platform linux/amd64,linux/arm64 \
            -f docker/Dockerfile.api \
            -t dno-crawler/api:${{ github.sha }} \
            -t dno-crawler/api:latest \
            --push .

  deploy:
    needs: build
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
      - name: Deploy to production
        uses: appleboy/ssh-action@v0.1.7
        with:
          host: ${{ secrets.PROD_HOST }}
          username: ${{ secrets.PROD_USER }}
          key: ${{ secrets.PROD_SSH_KEY }}
          script: |
            cd /opt/dno-crawler
            docker-compose pull
            docker-compose up -d
            docker system prune -f
```

## Getting Started

### 1. Initial Setup

```bash
# Clone repository
git clone https://github.com/yourusername/rust-dno-crawler.git
cd rust-dno-crawler

# Create environment files
cp .env.example .env.dev
cp .env.example .env.prod

# Edit environment files with your configurations
vim .env.prod

# Make scripts executable
chmod +x scripts/*.sh
```

### 2. Development Environment

```bash
# Start development services
docker-compose up -d postgres redis searxng

# Run migrations
./scripts/migrate.sh dev

# Start API server
cargo run --bin api

# Start website server (in another terminal)
cargo run --bin website

# Start crawler service (in another terminal)
cargo run --bin crawler daemon
```

### 3. Production Deployment

```bash
# Prepare production environment
./scripts/deploy.sh prod

# Check deployment
./scripts/health-check.sh prod

# Monitor logs
docker-compose logs -f api-server website-server crawler-service
```

## Maintenance Tasks

### 1. Daily Tasks
- Monitor service health
- Check error logs
- Verify backup completion
- Review performance metrics

### 2. Weekly Tasks
- Update dependencies
- Security patch updates
- Performance analysis
- Data quality review

### 3. Monthly Tasks
- Full system backup
- Security audit
- Capacity planning review
- Documentation updates

This deployment plan provides a comprehensive foundation for building and deploying the DNO crawler system in a production environment. The modular architecture allows for easy scaling and maintenance while ensuring security and reliability.