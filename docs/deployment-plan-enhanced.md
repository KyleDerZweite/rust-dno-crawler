# DNO Crawler - Enhanced Production Deployment Plan

## Overview

This document outlines the complete deployment strategy for the DNO (Distribution Network Operator) crawler system. The architecture is designed for scalability, maintainability, and production readiness with full Docker containerization.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     Load Balancer (HAProxy)                     │
│                   SSL Termination + Failover                    │
└─────────────────────────┬───────────────────────────────────────┘
                          │
┌─────────────────────────┼───────────────────────────────────────┐
│                    Docker Swarm / K8s Network                   │
│                                                                 │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │   API Server    │  │  Website Server │  │ Crawler Service │  │
│  │    (Axum)       │  │    (Dioxus)     │  │   (Background)  │  │
│  │   Port: 3000    │  │   Port: 8000    │  │   (No expose)   │  │
│  │   Replicas: 3   │  │   Replicas: 2   │  │   Replicas: N   │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
│           │                      │                      │       │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐  │
│  │   PostgreSQL    │  │      Redis      │  │     SearXNG     │  │
│  │   Primary +     │  │   Cluster Mode  │  │   Port: 8080    │  │
│  │   Read Replicas │  │   Port: 6379    │  │                 │  │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘  │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │              Monitoring & Logging Stack                      ││
│  │  Prometheus | Grafana | Loki | Alertmanager | Jaeger       ││
│  └─────────────────────────────────────────────────────────────┘│
│                                                                 │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                    Shared Storage (S3/MinIO)                ││
│  │           (DNO Data, PDFs, Cache, Logs, Backups)            ││
│  └─────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

## Table of Contents

1. [Service Components](#service-components)
2. [Docker Architecture](#docker-architecture)
3. [Configuration Management](#configuration-management)
4. [SSL/TLS Management](#ssltls-management)
5. [Secrets Management](#secrets-management)
6. [Database Management](#database-management)
7. [Monitoring Stack](#monitoring-stack)
8. [Logging & Error Tracking](#logging--error-tracking)
9. [CI/CD Pipeline](#cicd-pipeline)
10. [Zero-Downtime Deployment](#zero-downtime-deployment)
11. [Backup & Disaster Recovery](#backup--disaster-recovery)
12. [Security Implementation](#security-implementation)
13. [Performance & Scaling](#performance--scaling)
14. [API Documentation](#api-documentation)
15. [Testing Strategy](#testing-strategy)
16. [Operational Runbooks](#operational-runbooks)

## Service Components

### 1. API Server (api-server)
- **Technology**: Rust + Axum
- **Purpose**: RESTful API backend for all data operations
- **Port**: 3000 (internal), 80/443 (external via load balancer)
- **Replicas**: 3 (minimum for HA)
- **Features**:
  - JWT authentication with role-based access
  - Rate limiting per user/IP with Redis
  - CORS handling with configurable origins
  - Database connection pooling (min: 5, max: 20)
  - Prometheus metrics at `/metrics`
  - WebSocket support with Redis pub/sub for scaling
  - OpenAPI documentation at `/api/docs`
  - Health checks: `/health` (basic), `/health/ready` (deep)
  - Graceful shutdown with connection draining

### 2. Website Server (website-server)
- **Technology**: Rust + Dioxus
- **Purpose**: Frontend web application
- **Port**: 8000 (internal), 80/443 (external via load balancer)
- **Replicas**: 2 (minimum for HA)
- **Features**:
  - Server-side rendering for SEO
  - Asset caching with versioning
  - Content Security Policy headers
  - Feature flags via environment variables
  - Session affinity for WebSocket connections
  - Progressive Web App support
  - CDN integration for static assets

### 3. Crawler Service (crawler-service)
- **Technology**: Rust + AI agents
- **Purpose**: Intelligent data gathering from DNO sources
- **Port**: None (background service)
- **Replicas**: Variable based on workload
- **Features**:
  - Job queue with priority levels
  - Dead letter queue for failed jobs
  - Distributed locking for job coordination
  - Metrics export for monitoring
  - Configurable concurrency limits
  - Circuit breaker for external services
  - Retry with exponential backoff

### 4. Database Services
- **PostgreSQL**: 
  - Primary-replica setup with streaming replication
  - Connection pooling via PgBouncer
  - Automated failover with repmgr
  - Point-in-time recovery capability
- **Redis**: 
  - Cluster mode for high availability
  - Persistence with AOF + RDB
  - Separate instances for cache/sessions/queues
- **SearXNG**: 
  - Load balanced instances
  - Result caching in Redis

### 5. Load Balancer & Reverse Proxy
- **Technology**: HAProxy + Nginx
- **Features**:
  - Active-passive HA setup
  - SSL/TLS termination with Let's Encrypt
  - WebSocket sticky sessions
  - Request/response compression
  - DDoS protection
  - Geographic routing

## Docker Architecture

### Production Docker Compose Structure

```yaml
version: '3.11'

x-common-variables: &common-variables
  TZ: ${TZ:-UTC}
  NODE_ENV: production

x-healthcheck-defaults: &healthcheck-defaults
  interval: 30s
  timeout: 10s
  retries: 3
  start_period: 40s

networks:
  dno-network:
    driver: overlay
    attachable: true
    ipam:
      config:
        - subnet: 172.20.0.0/16

volumes:
  postgres_data:
    driver: local
  postgres_backup:
    driver: local
  redis_data:
    driver: local
  searxng_data:
    driver: local
  prometheus_data:
    driver: local
  grafana_data:
    driver: local
  loki_data:
    driver: local

secrets:
  postgres_password:
    external: true
  jwt_secret:
    external: true
  redis_password:
    external: true

configs:
  nginx_config:
    file: ./docker/nginx/nginx.conf
  prometheus_config:
    file: ./docker/prometheus/prometheus.yml

services:
  # Infrastructure Services
  postgres-primary:
    image: postgres:16-alpine
    deploy:
      replicas: 1
      resources:
        limits:
          cpus: '2'
          memory: 4G
        reservations:
          cpus: '1'
          memory: 2G
    environment:
      <<: *common-variables
      POSTGRES_DB: ${POSTGRES_DB}
      POSTGRES_USER: ${POSTGRES_USER}
      POSTGRES_PASSWORD_FILE: /run/secrets/postgres_password
      POSTGRES_INITDB_ARGS: "--encoding=UTF8 --locale=C"
      POSTGRES_HOST_AUTH_METHOD: scram-sha-256
    secrets:
      - postgres_password
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - postgres_backup:/backups
      - ./migrations:/docker-entrypoint-initdb.d:ro
      - ./docker/postgres/postgresql.conf:/etc/postgresql/postgresql.conf:ro
    command: postgres -c config_file=/etc/postgresql/postgresql.conf
    healthcheck:
      <<: *healthcheck-defaults
      test: ["CMD-SHELL", "pg_isready -U ${POSTGRES_USER} -d ${POSTGRES_DB}"]
    networks:
      - dno-network

  postgres-replica:
    image: postgres:16-alpine
    deploy:
      replicas: 1
      resources:
        limits:
          cpus: '2'
          memory: 4G
    environment:
      <<: *common-variables
      PGUSER: ${POSTGRES_USER}
      POSTGRES_PASSWORD_FILE: /run/secrets/postgres_password
    secrets:
      - postgres_password
    volumes:
      - ./docker/postgres/recovery.conf:/var/lib/postgresql/data/recovery.conf:ro
    depends_on:
      postgres-primary:
        condition: service_healthy
    networks:
      - dno-network

  pgbouncer:
    image: pgbouncer/pgbouncer:latest
    deploy:
      replicas: 2
      resources:
        limits:
          cpus: '0.5'
          memory: 512M
    environment:
      DATABASES_HOST: postgres-primary
      DATABASES_PORT: 5432
      DATABASES_USER: ${POSTGRES_USER}
      DATABASES_PASSWORD_FILE: /run/secrets/postgres_password
      POOL_MODE: transaction
      MAX_CLIENT_CONN: 1000
      DEFAULT_POOL_SIZE: 25
    secrets:
      - postgres_password
    depends_on:
      postgres-primary:
        condition: service_healthy
    networks:
      - dno-network

  redis-node-1:
    image: redis:7-alpine
    deploy:
      resources:
        limits:
          cpus: '1'
          memory: 2G
    command: >
      redis-server
      --cluster-enabled yes
      --cluster-config-file nodes.conf
      --cluster-node-timeout 5000
      --appendonly yes
      --requirepass ${REDIS_PASSWORD}
      --maxmemory 1gb
      --maxmemory-policy allkeys-lru
    volumes:
      - redis_data:/data
    secrets:
      - redis_password
    healthcheck:
      <<: *healthcheck-defaults
      test: ["CMD", "redis-cli", "--no-auth-warning", "-a", "${REDIS_PASSWORD}", "ping"]
    networks:
      - dno-network

  searxng:
    image: searxng/searxng:latest
    deploy:
      replicas: 2
      resources:
        limits:
          cpus: '1'
          memory: 1G
    environment:
      <<: *common-variables
      SEARXNG_BASE_URL: ${SEARXNG_BASE_URL}
      SEARXNG_REDIS_URL: redis://:${REDIS_PASSWORD}@redis-node-1:6379/1
    volumes:
      - searxng_data:/etc/searxng
      - ./docker/searxng/settings.yml:/etc/searxng/settings.yml:ro
    depends_on:
      redis-node-1:
        condition: service_healthy
    healthcheck:
      <<: *healthcheck-defaults
      test: ["CMD", "curl", "-f", "http://localhost:8080/healthz"]
    networks:
      - dno-network

  # Application Services
  api-server:
    image: ${DOCKER_REGISTRY}/dno-crawler/api:${VERSION:-latest}
    deploy:
      replicas: 3
      update_config:
        parallelism: 1
        delay: 10s
        order: start-first
      restart_policy:
        condition: any
        delay: 5s
        max_attempts: 3
      resources:
        limits:
          cpus: '2'
          memory: 2G
        reservations:
          cpus: '0.5'
          memory: 512M
    environment:
      <<: *common-variables
      DATABASE_URL: postgresql://${POSTGRES_USER}@pgbouncer:5432/${POSTGRES_DB}
      DATABASE_READ_URL: postgresql://${POSTGRES_USER}@postgres-replica:5432/${POSTGRES_DB}
      REDIS_URL: redis://:${REDIS_PASSWORD}@redis-node-1:6379/0
      JWT_SECRET_FILE: /run/secrets/jwt_secret
      RUST_LOG: ${RUST_LOG:-info}
      SERVER_HOST: 0.0.0.0
      SERVER_PORT: 3000
      ENABLE_METRICS: true
      ENABLE_TRACING: true
      JAEGER_ENDPOINT: http://jaeger:14268/api/traces
    secrets:
      - postgres_password
      - jwt_secret
      - redis_password
    volumes:
      - ./storage:/app/storage
      - ./logs:/app/logs
    depends_on:
      pgbouncer:
        condition: service_started
      redis-node-1:
        condition: service_healthy
    healthcheck:
      <<: *healthcheck-defaults
      test: ["CMD", "curl", "-f", "http://localhost:3000/health/ready"]
    networks:
      - dno-network

  website-server:
    image: ${DOCKER_REGISTRY}/dno-crawler/website:${VERSION:-latest}
    deploy:
      replicas: 2
      update_config:
        parallelism: 1
        delay: 10s
        order: start-first
      resources:
        limits:
          cpus: '1'
          memory: 1G
    environment:
      <<: *common-variables
      API_URL: http://api-server:3000
      RUST_LOG: ${RUST_LOG:-info}
      SERVER_HOST: 0.0.0.0
      SERVER_PORT: 8000
      ENABLE_METRICS: true
    depends_on:
      api-server:
        condition: service_healthy
    healthcheck:
      <<: *healthcheck-defaults
      test: ["CMD", "curl", "-f", "http://localhost:8000/health"]
    networks:
      - dno-network

  crawler-service:
    image: ${DOCKER_REGISTRY}/dno-crawler/crawler:${VERSION:-latest}
    deploy:
      replicas: ${CRAWLER_REPLICAS:-3}
      resources:
        limits:
          cpus: '2'
          memory: 4G
    environment:
      <<: *common-variables
      DATABASE_URL: postgresql://${POSTGRES_USER}@pgbouncer:5432/${POSTGRES_DB}
      REDIS_URL: redis://:${REDIS_PASSWORD}@redis-node-1:6379/0
      SEARXNG_URL: http://searxng:8080
      RUST_LOG: ${RUST_LOG:-info}
      CRAWLER_WORKERS: ${CRAWLER_WORKERS:-3}
      CRAWLER_INTERVAL: ${CRAWLER_INTERVAL:-3600}
      ENABLE_METRICS: true
    secrets:
      - postgres_password
      - redis_password
    volumes:
      - ./storage:/app/storage
      - ./logs:/app/logs
    depends_on:
      pgbouncer:
        condition: service_started
      redis-node-1:
        condition: service_healthy
      searxng:
        condition: service_healthy
    restart: unless-stopped
    networks:
      - dno-network

  # Monitoring Stack
  prometheus:
    image: prom/prometheus:latest
    deploy:
      replicas: 1
      resources:
        limits:
          cpus: '1'
          memory: 2G
    volumes:
      - prometheus_data:/prometheus
      - ./docker/prometheus:/etc/prometheus:ro
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      - '--storage.tsdb.retention.time=30d'
      - '--web.enable-lifecycle'
    healthcheck:
      <<: *healthcheck-defaults
      test: ["CMD", "wget", "--spider", "-q", "http://localhost:9090/-/healthy"]
    networks:
      - dno-network

  grafana:
    image: grafana/grafana:latest
    deploy:
      replicas: 1
      resources:
        limits:
          cpus: '1'
          memory: 1G
    environment:
      GF_SECURITY_ADMIN_PASSWORD: ${GRAFANA_PASSWORD}
      GF_INSTALL_PLUGINS: grafana-clock-panel,grafana-piechart-panel
    volumes:
      - grafana_data:/var/lib/grafana
      - ./docker/grafana/provisioning:/etc/grafana/provisioning:ro
      - ./docker/grafana/dashboards:/var/lib/grafana/dashboards:ro
    depends_on:
      - prometheus
    healthcheck:
      <<: *healthcheck-defaults
      test: ["CMD", "curl", "-f", "http://localhost:3000/api/health"]
    networks:
      - dno-network

  loki:
    image: grafana/loki:latest
    deploy:
      replicas: 1
      resources:
        limits:
          cpus: '1'
          memory: 2G
    volumes:
      - loki_data:/loki
      - ./docker/loki/loki-config.yml:/etc/loki/loki-config.yml:ro
    command: -config.file=/etc/loki/loki-config.yml
    healthcheck:
      <<: *healthcheck-defaults
      test: ["CMD", "wget", "--spider", "-q", "http://localhost:3100/ready"]
    networks:
      - dno-network

  promtail:
    image: grafana/promtail:latest
    deploy:
      mode: global
      resources:
        limits:
          cpus: '0.5'
          memory: 512M
    volumes:
      - ./logs:/app/logs:ro
      - /var/log:/var/log:ro
      - ./docker/promtail/promtail-config.yml:/etc/promtail/promtail-config.yml:ro
    command: -config.file=/etc/promtail/promtail-config.yml
    depends_on:
      - loki
    networks:
      - dno-network

  alertmanager:
    image: prom/alertmanager:latest
    deploy:
      replicas: 1
      resources:
        limits:
          cpus: '0.5'
          memory: 512M
    volumes:
      - ./docker/alertmanager:/etc/alertmanager:ro
    command:
      - '--config.file=/etc/alertmanager/alertmanager.yml'
      - '--storage.path=/alertmanager'
    networks:
      - dno-network

  jaeger:
    image: jaegertracing/all-in-one:latest
    deploy:
      replicas: 1
      resources:
        limits:
          cpus: '1'
          memory: 2G
    environment:
      COLLECTOR_ZIPKIN_HOST_PORT: 9411
      COLLECTOR_OTLP_ENABLED: true
    networks:
      - dno-network

  # Load Balancer
  haproxy:
    image: haproxy:alpine
    deploy:
      replicas: 2
      update_config:
        parallelism: 1
        delay: 10s
      resources:
        limits:
          cpus: '2'
          memory: 1G
    ports:
      - target: 80
        published: 80
        mode: host
      - target: 443
        published: 443
        mode: host
      - target: 8404
        published: 8404
        mode: ingress
    volumes:
      - ./docker/haproxy/haproxy.cfg:/usr/local/etc/haproxy/haproxy.cfg:ro
      - ./docker/haproxy/ssl:/etc/ssl:ro
      - ./docker/haproxy/errors:/etc/haproxy/errors:ro
    depends_on:
      - api-server
      - website-server
    healthcheck:
      <<: *healthcheck-defaults
      test: ["CMD", "haproxy", "-c", "-f", "/usr/local/etc/haproxy/haproxy.cfg"]
    networks:
      - dno-network

  # Certificate Management
  certbot:
    image: certbot/certbot:latest
    deploy:
      replicas: 1
    volumes:
      - ./docker/certbot/conf:/etc/letsencrypt
      - ./docker/certbot/www:/var/www/certbot
    entrypoint: "/bin/sh -c 'trap exit TERM; while :; do certbot renew; sleep 12h & wait $${!}; done;'"
    networks:
      - dno-network
```

## Configuration Management

### Environment Variables Structure

Create environment-specific files:

```bash
# .env.common - Shared across all environments
TZ=Europe/Berlin
DOCKER_REGISTRY=registry.yourdomain.com
VERSION=latest

# .env.dev - Development environment
ENV=development
POSTGRES_DB=dno_crawler_dev
POSTGRES_USER=dno_dev
LOG_LEVEL=debug
ENABLE_DEBUG=true

# .env.staging - Staging environment  
ENV=staging
POSTGRES_DB=dno_crawler_staging
POSTGRES_USER=dno_staging
LOG_LEVEL=info
ENABLE_METRICS=true

# .env.prod - Production environment
ENV=production
POSTGRES_DB=dno_crawler_prod
POSTGRES_USER=dno_prod
LOG_LEVEL=warn
ENABLE_METRICS=true
ENABLE_TRACING=true

# Rate Limiting Configuration
RATE_LIMIT_PER_SECOND=10
RATE_LIMIT_BURST=50
RATE_LIMIT_PER_USER_MINUTE=100
RATE_LIMIT_PER_USER_HOUR=1000

# Connection Pool Configuration
DB_POOL_MIN_SIZE=5
DB_POOL_MAX_SIZE=20
DB_POOL_TIMEOUT=30
DB_POOL_IDLE_TIMEOUT=600

# Redis Configuration
REDIS_POOL_SIZE=10
REDIS_CONNECT_TIMEOUT=5
REDIS_COMMAND_TIMEOUT=5

# Crawler Configuration
CRAWLER_REPLICAS=3
CRAWLER_WORKERS=3
CRAWLER_INTERVAL=3600
CRAWLER_MAX_CONCURRENT_JOBS=10
CRAWLER_JOB_TIMEOUT=300
CRAWLER_RETRY_ATTEMPTS=3
CRAWLER_RETRY_DELAY=60

# Feature Flags
FEATURE_NEW_UI=true
FEATURE_AI_SEARCH=true
FEATURE_BETA_API=false

# External Services
SENTRY_DSN=https://xxx@sentry.io/xxx
JAEGER_ENDPOINT=http://jaeger:14268/api/traces
```

### HAProxy Configuration (`docker/haproxy/haproxy.cfg`)

```haproxy
global
    maxconn 4096
    log stdout local0
    stats socket /var/run/haproxy.sock mode 660 level admin
    ssl-default-bind-ciphers ECDHE+AESGCM:ECDHE+AES256:ECDHE+AES128:!PSK:!DHE:!RSA:!DSS:!aNull:!MD5
    ssl-default-bind-options no-sslv3 no-tlsv10 no-tlsv11
    tune.ssl.default-dh-param 2048

defaults
    mode http
    option httplog
    option dontlognull
    option forwardfor
    option http-server-close
    timeout connect 5s
    timeout client 30s
    timeout server 30s
    timeout http-request 10s
    timeout http-keep-alive 15s
    
    # Enable compression
    compression algo gzip
    compression type text/html text/css application/javascript application/json

    # Error files
    errorfile 400 /etc/haproxy/errors/400.http
    errorfile 403 /etc/haproxy/errors/403.http
    errorfile 408 /etc/haproxy/errors/408.http
    errorfile 500 /etc/haproxy/errors/500.http
    errorfile 502 /etc/haproxy/errors/502.http
    errorfile 503 /etc/haproxy/errors/503.http
    errorfile 504 /etc/haproxy/errors/504.http

# Statistics
listen stats
    bind *:8404
    stats enable
    stats uri /stats
    stats refresh 5s
    stats auth admin:${HAPROXY_STATS_PASSWORD}

# Frontend - HTTP
frontend http_front
    bind *:80
    # Redirect all HTTP to HTTPS
    redirect scheme https code 301 if !{ ssl_fc }
    
    # ACME challenge for Let's Encrypt
    acl letsencrypt-acl path_beg /.well-known/acme-challenge/
    use_backend letsencrypt-backend if letsencrypt-acl

# Frontend - HTTPS
frontend https_front
    bind *:443 ssl crt /etc/ssl/certs/ alpn h2,http/1.1
    
    # Security headers
    http-response set-header Strict-Transport-Security "max-age=63072000; includeSubDomains; preload"
    http-response set-header X-Frame-Options "SAMEORIGIN"
    http-response set-header X-Content-Type-Options "nosniff"
    http-response set-header X-XSS-Protection "1; mode=block"
    http-response set-header Referrer-Policy "strict-origin-when-cross-origin"
    
    # ACLs
    acl is_api path_beg /api/
    acl is_ws path_beg /ws
    acl is_metrics path /metrics
    acl is_health path_beg /health
    
    # Rate limiting
    stick-table type ip size 100k expire 30s store http_req_rate(10s)
    http-request track-sc0 src
    http-request deny if { sc_http_req_rate(0) gt 100 }
    
    # Routing
    use_backend api_backend if is_api
    use_backend ws_backend if is_ws
    use_backend metrics_backend if is_metrics
    use_backend health_backend if is_health
    default_backend website_backend

# Backend - API
backend api_backend
    balance leastconn
    option httpchk GET /health HTTP/1.1\r\nHost:\ api
    
    # Retry and timeout settings
    retries 3
    timeout server 30s
    
    # Circuit breaker
    option redispatch
    
    # Connection reuse
    http-reuse safe
    
    # Servers with health checks
    server api1 api-server:3000 check inter 5s fall 3 rise 2 weight 100
    server api2 api-server:3001 check inter 5s fall 3 rise 2 weight 100
    server api3 api-server:3002 check inter 5s fall 3 rise 2 weight 100

# Backend - WebSocket
backend ws_backend
    balance source
    option http-server-close
    option forceclose
    
    # WebSocket specific settings
    timeout tunnel 1h
    
    server api1 api-server:3000 check
    server api2 api-server:3001 check
    server api3 api-server:3002 check

# Backend - Website
backend website_backend
    balance roundrobin
    option httpchk GET /health HTTP/1.1\r\nHost:\ website
    
    # Cache static assets
    http-response set-header Cache-Control "public, max-age=3600" if { path_end .js .css .png .jpg .jpeg .gif .ico .woff .woff2 }
    
    server web1 website-server:8000 check inter 5s fall 3 rise 2
    server web2 website-server:8001 check inter 5s fall 3 rise 2

# Backend - Metrics (internal only)
backend metrics_backend
    # Restrict to internal IPs only
    acl internal_network src 10.0.0.0/8 172.16.0.0/12 192.168.0.0/16
    http-request deny if !internal_network
    
    server api1 api-server:3000

# Backend - Health checks
backend health_backend
    server api1 api-server:3000

# Backend - Let's Encrypt
backend letsencrypt-backend
    server certbot certbot:80
```

## SSL/TLS Management

### Automated Certificate Management Script (`scripts/ssl-management.sh`)

```bash
#!/bin/bash
set -e

DOMAIN=${1:-yourdomain.com}
EMAIL=${2:-admin@yourdomain.com}
STAGING=${3:-false}

# Certificate paths
CERT_PATH="/etc/letsencrypt/live/${DOMAIN}"
HAPROXY_CERT_PATH="/etc/haproxy/ssl"

# Functions
setup_certbot() {
    echo "Setting up Certbot for ${DOMAIN}..."
    
    STAGING_FLAG=""
    if [ "$STAGING" = "true" ]; then
        STAGING_FLAG="--staging"
    fi
    
    docker run --rm \
        -v ./docker/certbot/conf:/etc/letsencrypt \
        -v ./docker/certbot/www:/var/www/certbot \
        certbot/certbot certonly \
        --webroot \
        --webroot-path=/var/www/certbot \
        --email ${EMAIL} \
        --agree-tos \
        --no-eff-email \
        --force-renewal \
        ${STAGING_FLAG} \
        -d ${DOMAIN} \
        -d www.${DOMAIN}
}

combine_certificates() {
    echo "Combining certificates for HAProxy..."
    
    # Combine cert and key for HAProxy
    cat ${CERT_PATH}/fullchain.pem ${CERT_PATH}/privkey.pem > ${HAPROXY_CERT_PATH}/${DOMAIN}.pem
    
    # Set proper permissions
    chmod 600 ${HAPROXY_CERT_PATH}/${DOMAIN}.pem
}

reload_haproxy() {
    echo "Reloading HAProxy..."
    docker service update --force haproxy
}

setup_auto_renewal() {
    echo "Setting up auto-renewal..."
    
    # Create renewal script
    cat > /etc/cron.daily/certbot-renew <<EOF
#!/bin/bash
certbot renew --quiet --no-self-upgrade --post-hook "/usr/local/bin/ssl-management.sh combine && /usr/local/bin/ssl-management.sh reload"
EOF
    
    chmod +x /etc/cron.daily/certbot-renew
}

# Main execution
case "${4:-setup}" in
    setup)
        setup_certbot
        combine_certificates
        reload_haproxy
        setup_auto_renewal
        ;;
    combine)
        combine_certificates
        ;;
    reload)
        reload_haproxy
        ;;
    renew)
        certbot renew
        combine_certificates
        reload_haproxy
        ;;
    *)
        echo "Usage: $0 <domain> <email> <staging> [setup|combine|reload|renew]"
        exit 1
        ;;
esac

echo "SSL management completed!"
```

## Secrets Management

### HashiCorp Vault Integration (`scripts/vault-setup.sh`)

```bash
#!/bin/bash
set -e

VAULT_ADDR=${VAULT_ADDR:-http://vault:8200}
VAULT_TOKEN=${VAULT_TOKEN}

# Initialize Vault
init_vault() {
    echo "Initializing Vault..."
    
    # Start Vault in dev mode for initial setup
    docker run -d \
        --name vault-init \
        --cap-add=IPC_LOCK \
        -e 'VAULT_DEV_ROOT_TOKEN_ID=myroot' \
        -e 'VAULT_DEV_LISTEN_ADDRESS=0.0.0.0:8200' \
        vault
    
    sleep 5
    
    # Configure Vault
    export VAULT_ADDR='http://localhost:8200'
    export VAULT_TOKEN='myroot'
    
    # Enable KV v2 secret engine
    vault secrets enable -version=2 kv
    
    # Create policies
    vault policy write dno-api-policy - <<EOF
path "kv/data/dno-crawler/api/*" {
  capabilities = ["read"]
}
EOF
    
    vault policy write dno-admin-policy - <<EOF
path "kv/*" {
  capabilities = ["create", "read", "update", "delete", "list"]
}
EOF
}

# Store secrets
store_secrets() {
    echo "Storing secrets in Vault..."
    
    # Database credentials
    vault kv put kv/dno-crawler/database \
        username="${POSTGRES_USER}" \
        password="${POSTGRES_PASSWORD}" \
        host="postgres-primary" \
        port="5432" \
        database="${POSTGRES_DB}"
    
    # Redis credentials
    vault kv put kv/dno-crawler/redis \
        password="${REDIS_PASSWORD}" \
        host="redis-node-1" \
        port="6379"
    
    # JWT secrets
    vault kv put kv/dno-crawler/auth \
        jwt_secret="${JWT_SECRET}" \
        admin_secret="${ADMIN_SECRET}"
    
    # External service credentials
    vault kv put kv/dno-crawler/external \
        sentry_dsn="${SENTRY_DSN}" \
        smtp_password="${SMTP_PASSWORD}"
}

# Create Docker secrets from Vault
create_docker_secrets() {
    echo "Creating Docker secrets..."
    
    # Fetch from Vault and create Docker secrets
    vault kv get -field=password kv/dno-crawler/database | \
        docker secret create postgres_password -
    
    vault kv get -field=password kv/dno-crawler/redis | \
        docker secret create redis_password -
    
    vault kv get -field=jwt_secret kv/dno-crawler/auth | \
        docker secret create jwt_secret -
}

# Main execution
case "${1:-all}" in
    init)
        init_vault
        ;;
    store)
        store_secrets
        ;;
    docker-secrets)
        create_docker_secrets
        ;;
    all)
        init_vault
        store_secrets
        create_docker_secrets
        ;;
    *)
        echo "Usage: $0 [init|store|docker-secrets|all]"
        exit 1
        ;;
esac
```

## Database Management

### PostgreSQL Configuration (`docker/postgres/postgresql.conf`)

```conf
# Connection settings
listen_addresses = '*'
max_connections = 200
superuser_reserved_connections = 3

# Memory settings
shared_buffers = 1GB
effective_cache_size = 3GB
maintenance_work_mem = 256MB
work_mem = 4MB

# Checkpoint settings
checkpoint_completion_target = 0.9
wal_buffers = 16MB
min_wal_size = 2GB
max_wal_size = 8GB

# Query tuning
random_page_cost = 1.1
effective_io_concurrency = 200
default_statistics_target = 100

# Replication settings
wal_level = replica
max_wal_senders = 3
wal_keep_segments = 64
hot_standby = on
hot_standby_feedback = on

# Logging
log_destination = 'stderr'
logging_collector = on
log_directory = 'pg_log'
log_filename = 'postgresql-%Y-%m-%d_%H%M%S.log'
log_rotation_age = 1d
log_rotation_size = 100MB
log_line_prefix = '%t [%p]: [%l-1] user=%u,db=%d,app=%a,client=%h '
log_checkpoints = on
log_connections = on
log_disconnections = on
log_duration = off
log_lock_waits = on
log_statement = 'ddl'
log_temp_files = 0

# Performance monitoring
shared_preload_libraries = 'pg_stat_statements'
pg_stat_statements.track = all

# Autovacuum tuning
autovacuum_max_workers = 4
autovacuum_naptime = 30s
autovacuum_vacuum_scale_factor = 0.02
autovacuum_analyze_scale_factor = 0.01
```

### Database Migration Management (`scripts/db-migrate.sh`)

```bash
#!/bin/bash
set -e

# Configuration
ENV=${1:-dev}
COMMAND=${2:-up}
ENV_FILE=".env.${ENV}"

if [ ! -f "$ENV_FILE" ]; then
    echo "Environment file $ENV_FILE not found!"
    exit 1
fi

# Load environment
export $(cat $ENV_FILE | grep -v '^#' | xargs)

# Functions
install_migrate() {
    if ! command -v migrate &> /dev/null; then
        echo "Installing golang-migrate..."
        curl -L https://github.com/golang-migrate/migrate/releases/latest/download/migrate.linux-amd64.tar.gz | tar xvz
        sudo mv migrate /usr/local/bin/
    fi
}

create_migration() {
    NAME=$3
    if [ -z "$NAME" ]; then
        echo "Usage: $0 $ENV create <migration_name>"
        exit 1
    fi
    
    migrate create -ext sql -dir migrations -seq ${NAME}
    echo "Created migration: ${NAME}"
}

run_migration() {
    echo "Running migrations ${COMMAND} for ${ENV}..."
    
    DATABASE_URL="postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/${POSTGRES_DB}?sslmode=disable"
    
    case $COMMAND in
        up)
            migrate -path migrations -database "${DATABASE_URL}" up
            ;;
        down)
            migrate -path migrations -database "${DATABASE_URL}" down 1
            ;;
        force)
            VERSION=$3
            migrate -path migrations -database "${DATABASE_URL}" force ${VERSION}
            ;;
        version)
            migrate -path migrations -database "${DATABASE_URL}" version
            ;;
        *)
            echo "Unknown command: $COMMAND"
            exit 1
            ;;
    esac
}

backup_before_migration() {
    if [ "$ENV" = "prod" ] && [ "$COMMAND" = "up" ]; then
        echo "Creating backup before migration..."
        ./scripts/backup.sh db pre-migration
    fi
}

# Main execution
install_migrate
backup_before_migration
run_migration

echo "Migration completed!"
```

## Monitoring Stack

### Prometheus Configuration (`docker/prometheus/prometheus.yml`)

```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s
  external_labels:
    cluster: 'dno-crawler-prod'
    replica: '$(HOSTNAME)'

# Alertmanager configuration
alerting:
  alertmanagers:
    - static_configs:
        - targets:
            - alertmanager:9093

# Load rules
rule_files:
  - "alerts/*.yml"

# Scrape configurations
scrape_configs:
  # API Server metrics
  - job_name: 'api-server'
    dns_sd_configs:
      - names:
          - 'tasks.api-server'
        type: 'A'
        port: 3000
    relabel_configs:
      - source_labels: [__address__]
        target_label: instance
        regex: '([^:]+):.*'
        replacement: '${1}'
    metric_relabel_configs:
      - source_labels: [__name__]
        regex: 'go_.*'
        action: drop

  # Website Server metrics
  - job_name: 'website-server'
    dns_sd_configs:
      - names:
          - 'tasks.website-server'
        type: 'A'
        port: 8000

  # Crawler Service metrics
  - job_name: 'crawler-service'
    dns_sd_configs:
      - names:
          - 'tasks.crawler-service'
        type: 'A'
        port: 9090

  # PostgreSQL exporter
  - job_name: 'postgres'
    static_configs:
      - targets: ['postgres-exporter:9187']

  # Redis exporter
  - job_name: 'redis'
    static_configs:
      - targets: ['redis-exporter:9121']

  # Node exporter for host metrics
  - job_name: 'node'
    static_configs:
      - targets: ['node-exporter:9100']

  # HAProxy metrics
  - job_name: 'haproxy'
    static_configs:
      - targets: ['haproxy:8404']
```

### Alert Rules (`docker/prometheus/alerts/application.yml`)

```yaml
groups:
  - name: application
    interval: 30s
    rules:
      # API Server alerts
      - alert: APIHighResponseTime
        expr: histogram_quantile(0.95, rate(http_request_duration_seconds_bucket{job="api-server"}[5m])) > 0.5
        for: 5m
        labels:
          severity: warning
          service: api
        annotations:
          summary: "High API response time"
          description: "95th percentile response time is {{ $value }}s"

      - alert: APIHighErrorRate
        expr: rate(http_requests_total{job="api-server",status=~"5.."}[5m]) > 0.05
        for: 5m
        labels:
          severity: critical
          service: api
        annotations:
          summary: "High API error rate"
          description: "Error rate is {{ $value | humanizePercentage }}"

      # Database alerts
      - alert: DatabaseConnectionPoolExhausted
        expr: pg_stat_database_numbackends / pg_settings_max_connections > 0.8
        for: 5m
        labels:
          severity: warning
          service: database
        annotations:
          summary: "Database connection pool near exhaustion"
          description: "{{ $value | humanizePercentage }} of connections used"

      - alert: DatabaseReplicationLag
        expr: pg_replication_lag_seconds > 10
        for: 5m
        labels:
          severity: critical
          service: database
        annotations:
          summary: "High database replication lag"
          description: "Replication lag is {{ $value }}s"

      # Crawler alerts
      - alert: CrawlerJobQueueBacklog
        expr: redis_queue_length{queue="crawler_jobs"} > 1000
        for: 10m
        labels:
          severity: warning
          service: crawler
        annotations:
          summary: "Large crawler job queue backlog"
          description: "{{ $value }} jobs in queue"

      - alert: CrawlerHighFailureRate
        expr: rate(crawler_jobs_failed_total[5m]) / rate(crawler_jobs_total[5m]) > 0.1
        for: 5m
        labels:
          severity: critical
          service: crawler
        annotations:
          summary: "High crawler job failure rate"
          description: "{{ $value | humanizePercentage }} failure rate"

      # Infrastructure alerts
      - alert: ContainerDown
        expr: up == 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Container {{ $labels.job }} is down"
          description: "Container has been down for more than 1 minute"

      - alert: HighMemoryUsage
        expr: container_memory_usage_bytes / container_spec_memory_limit_bytes > 0.9
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High memory usage in {{ $labels.container_name }}"
          description: "Memory usage is {{ $value | humanizePercentage }}"

      - alert: DiskSpaceRunningOut
        expr: (node_filesystem_avail_bytes / node_filesystem_size_bytes) < 0.1
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "Disk space running out on {{ $labels.mountpoint }}"
          description: "Only {{ $value | humanizePercentage }} disk space left"
```

### Grafana Dashboards

Create comprehensive dashboards in `docker/grafana/dashboards/`:

1. **Application Overview Dashboard** (`application-overview.json`)
   - Request rate and latency
   - Error rates by endpoint
   - Active connections
   - Response time distribution

2. **Database Performance Dashboard** (`database-performance.json`)
   - Connection pool usage
   - Query performance
   - Replication lag
   - Cache hit rates

3. **Crawler Monitoring Dashboard** (`crawler-monitoring.json`)
   - Job processing rate
   - Success/failure rates
   - Queue depths
   - Resource utilization

4. **Infrastructure Dashboard** (`infrastructure.json`)
   - CPU and memory usage
   - Disk I/O
   - Network traffic
   - Container health

## Logging & Error Tracking

### Structured Logging Configuration

```rust
// crates/core/src/logging.rs
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use tracing_loki::LokiLayer;
use sentry_tracing::SentryLayer;

pub fn init_logging(config: &LogConfig) -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.level));

    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::CLOSE)
        .json();

    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer);

    // Add Loki layer for centralized logging
    if let Some(loki_url) = &config.loki_url {
        let loki_layer = LokiLayer::new(loki_url.clone(), "dno-crawler".to_string())?;
        let subscriber = subscriber.with(loki_layer);
    }

    // Add Sentry layer for error tracking
    if let Some(sentry_dsn) = &config.sentry_dsn {
        let _guard = sentry::init((
            sentry_dsn.clone(),
            sentry::ClientOptions {
                release: sentry::release_name!(),
                environment: Some(config.environment.clone().into()),
                sample_rate: config.sentry_sample_rate,
                traces_sample_rate: config.sentry_traces_sample_rate,
                ..Default::default()
            },
        ));
        
        let sentry_layer = SentryLayer::default();
        let subscriber = subscriber.with(sentry_layer);
    }

    subscriber.init();
    
    Ok(())
}
```

### Loki Configuration (`docker/loki/loki-config.yml`)

```yaml
auth_enabled: false

server:
  http_listen_port: 3100
  grpc_listen_port: 9095

ingester:
  wal:
    enabled: true
    dir: /loki/wal
  lifecycler:
    address: 127.0.0.1
    ring:
      kvstore:
        store: inmemory
      replication_factor: 1
    final_sleep: 0s
  chunk_idle_period: 5m
  chunk_retain_period: 30s

schema_config:
  configs:
    - from: 2023-01-01
      store: boltdb-shipper
      object_store: filesystem
      schema: v11
      index:
        prefix: index_
        period: 24h

storage_config:
  boltdb_shipper:
    active_index_directory: /loki/index
    cache_location: /loki/index_cache
    shared_store: filesystem
  filesystem:
    directory: /loki/chunks

limits_config:
  enforce_metric_name: false
  reject_old_samples: true
  reject_old_samples_max_age: 168h
  ingestion_rate_mb: 10
  ingestion_burst_size_mb: 20

chunk_store_config:
  max_look_back_period: 0s

table_manager:
  retention_deletes_enabled: true
  retention_period: 720h

compactor:
  working_directory: /loki/compactor
  shared_store: filesystem
```

### Error Tracking with Sentry Integration

```rust
// crates/api/src/middleware/error_handler.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::any::Any;

#[derive(Debug)]
pub struct AppError {
    pub code: StatusCode,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Capture error in Sentry
        sentry::capture_message(&self.message, sentry::Level::Error);
        
        // Log error with context
        tracing::error!(
            code = %self.code,
            message = %self.message,
            details = ?self.details,
            "Application error"
        );
        
        // Return user-friendly error
        let body = Json(json!({
            "error": {
                "code": self.code.as_u16(),
                "message": self.message,
                "details": self.details,
                "request_id": sentry::last_event_id().map(|id| id.to_string()),
            }
        }));
        
        (self.code, body).into_response()
    }
}

// Panic handler for unhandled errors
pub fn setup_panic_handler() {
    let default_panic = std::panic::take_hook();
    
    std::panic::set_hook(Box::new(move |panic_info| {
        let panic_message = panic_info
            .payload()
            .downcast_ref::<String>()
            .map(|s| s.clone())
            .or_else(|| {
                panic_info
                    .payload()
                    .downcast_ref::<&str>()
                    .map(|s| s.to_string())
            })
            .unwrap_or_else(|| "Unknown panic".to_string());
        
        sentry::capture_message(&panic_message, sentry::Level::Fatal);
        
        default_panic(panic_info);
    }));
}
```

## CI/CD Pipeline

### GitHub Actions Workflow (`.github/workflows/ci-cd.yml`)

```yaml
name: CI/CD Pipeline

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]
  release:
    types: [created]

env:
  RUST_VERSION: "1.75"
  CARGO_TERM_COLOR: always
  DOCKER_REGISTRY: registry.yourdomain.com
  SCCACHE_GHA_ENABLED: "true"

jobs:
  # Code quality checks
  quality:
    name: Code Quality
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: ${{ env.RUST_VERSION }}
          components: rustfmt, clippy
      
      - name: Setup sccache
        uses: mozilla-actions/sccache-action@v0.0.3
      
      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2
      
      - name: Check formatting
        run: cargo fmt --all -- --check
      
      - name: Run clippy
        run: cargo clippy --all-targets --all-features -- -D warnings
      
      - name: Check dependencies
        run: |
          cargo install cargo-audit cargo-outdated cargo-license
          cargo audit
          cargo outdated --exit-code 1
          cargo license --json > licenses.json

  # Security scanning
  security:
    name: Security Scan
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Run Trivy vulnerability scanner
        uses: aquasecurity/trivy-action@master
        with:
          scan-type: 'fs'
          scan-ref: '.'
          format: 'sarif'
          output: 'trivy-results.sarif'
      
      - name: Upload Trivy results to GitHub Security
        uses: github/codeql-action/upload-sarif@v2
        with:
          sarif_file: 'trivy-results.sarif'
      
      - name: Run cargo-deny
        uses: EmbarkStudios/cargo-deny-action@v1

  # Unit and integration tests
  test:
    name: Test Suite
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16-alpine
        env:
          POSTGRES_USER: test
          POSTGRES_PASSWORD: test
          POSTGRES_DB: dno_test
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 5432:5432
      
      redis:
        image: redis:7-alpine
        options: >-
          --health-cmd "redis-cli ping"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 6379:6379
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: ${{ env.RUST_VERSION }}
      
      - name: Setup sccache
        uses: mozilla-actions/sccache-action@v0.0.3
      
      - name: Cache dependencies
        uses: Swatinem/rust-cache@v2
      
      - name: Run migrations
        env:
          DATABASE_URL: postgresql://test:test@localhost:5432/dno_test
        run: |
          cargo install sqlx-cli --no-default-features --features postgres
          sqlx migrate run
      
      - name: Run tests
        env:
          DATABASE_URL: postgresql://test:test@localhost:5432/dno_test
          REDIS_URL: redis://localhost:6379
          RUST_LOG: debug
        run: |
          cargo test --all --all-features -- --test-threads=1
          cargo test --all --all-features --release -- --ignored
      
      - name: Generate test coverage
        run: |
          cargo install cargo-tarpaulin
          cargo tarpaulin --out Xml --all-features
      
      - name: Upload coverage to Codecov
        uses: codecov/codecov-action@v3
        with:
          files: ./cobertura.xml

  # Load testing
  load-test:
    name: Load Testing
    runs-on: ubuntu-latest
    needs: [quality, test]
    if: github.event_name == 'pull_request'
    steps:
      - uses: actions/checkout@v4
      
      - name: Run load tests
        run: |
          docker run --rm \
            -v $PWD/tests/load:/scripts \
            -v $PWD/results:/results \
            grafana/k6 run \
            --out json=/results/load-test.json \
            /scripts/api-load-test.js
      
      - name: Upload results
        uses: actions/upload-artifact@v3
        with:
          name: load-test-results
          path: results/

  # Build Docker images
  build:
    name: Build Images
    runs-on: ubuntu-latest
    needs: [quality, security, test]
    strategy:
      matrix:
        service: [api, website, crawler]
    steps:
      - uses: actions/checkout@v4
      
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3
      
      - name: Log in to registry
        uses: docker/login-action@v3
        with:
          registry: ${{ env.DOCKER_REGISTRY }}
          username: ${{ secrets.DOCKER_USERNAME }}
          password: ${{ secrets.DOCKER_PASSWORD }}
      
      - name: Extract metadata
        id: meta
        uses: docker/metadata-action@v5
        with:
          images: ${{ env.DOCKER_REGISTRY }}/dno-crawler/${{ matrix.service }}
          tags: |
            type=ref,event=branch
            type=ref,event=pr
            type=semver,pattern={{version}}
            type=semver,pattern={{major}}.{{minor}}
            type=sha,prefix={{branch}}-
      
      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          file: ./docker/Dockerfile.${{ matrix.service }}
          platforms: linux/amd64,linux/arm64
          push: true
          tags: ${{ steps.meta.outputs.tags }}
          labels: ${{ steps.meta.outputs.labels }}
          cache-from: type=gha
          cache-to: type=gha,mode=max
          build-args: |
            VERSION=${{ github.sha }}
            BUILD_DATE=$(date -u +'%Y-%m-%dT%H:%M:%SZ')

  # Deploy to staging
  deploy-staging:
    name: Deploy to Staging
    runs-on: ubuntu-latest
    needs: build
    if: github.ref == 'refs/heads/develop'
    environment:
      name: staging
      url: https://staging.yourdomain.com
    steps:
      - uses: actions/checkout@v4
      
      - name: Deploy to staging
        uses: appleboy/ssh-action@v1.0.0
        with:
          host: ${{ secrets.STAGING_HOST }}
          username: ${{ secrets.STAGING_USER }}
          key: ${{ secrets.STAGING_SSH_KEY }}
          script: |
            cd /opt/dno-crawler
            git pull origin develop
            ./scripts/deploy.sh staging ${{ github.sha }}

  # Deploy to production
  deploy-production:
    name: Deploy to Production
    runs-on: ubuntu-latest
    needs: build
    if: github.ref == 'refs/heads/main'
    environment:
      name: production
      url: https://yourdomain.com
    steps:
      - uses: actions/checkout@v4
      
      - name: Run pre-deployment checks
        run: ./scripts/pre-deploy-check.sh prod
      
      - name: Deploy to production
        uses: appleboy/ssh-action@v1.0.0
        with:
          host: ${{ secrets.PROD_HOST }}
          username: ${{ secrets.PROD_USER }}
          key: ${{ secrets.PROD_SSH_KEY }}
          script: |
            cd /opt/dno-crawler
            git pull origin main
            ./scripts/deploy.sh prod ${{ github.sha }}
      
      - name: Run post-deployment tests
        run: ./scripts/post-deploy-test.sh prod
      
      - name: Notify deployment
        if: always()
        uses: 8398a7/action-slack@v3
        with:
          status: ${{ job.status }}
          text: 'Production deployment ${{ job.status }}'
          webhook_url: ${{ secrets.SLACK_WEBHOOK }}
```

## Zero-Downtime Deployment

### Blue-Green Deployment Script (`scripts/blue-green-deploy.sh`)

```bash
#!/bin/bash
set -e

ENV=${1:-staging}
VERSION=${2:-latest}
SERVICE=${3:-all}

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

# Configuration
HEALTH_CHECK_RETRIES=30
HEALTH_CHECK_INTERVAL=5

log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
    exit 1
}

success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Check current active environment
get_active_env() {
    docker service ls --format "table {{.Name}}" | grep -E "${SERVICE}.*-(blue|green)" | head -1 | grep -oE "(blue|green)" || echo "blue"
}

# Deploy new version to inactive environment
deploy_to_env() {
    local target_env=$1
    local service_name=$2
    
    log "Deploying ${service_name} version ${VERSION} to ${target_env} environment"
    
    # Update service with new image
    docker service update \
        --image ${DOCKER_REGISTRY}/dno-crawler/${service_name}:${VERSION} \
        --update-parallelism 1 \
        --update-delay 10s \
        --update-failure-action rollback \
        --update-monitor 30s \
        --update-max-failure-ratio 0.1 \
        ${service_name}-${target_env}
}

# Health check
health_check() {
    local service_name=$1
    local env=$2
    local endpoint=$3
    
    log "Running health checks for ${service_name}-${env}"
    
    for i in $(seq 1 $HEALTH_CHECK_RETRIES); do
        if curl -sf "http://${service_name}-${env}:${endpoint}/health/ready" > /dev/null; then
            success "Health check passed for ${service_name}-${env}"
            return 0
        fi
        
        log "Health check attempt $i/$HEALTH_CHECK_RETRIES failed, retrying in ${HEALTH_CHECK_INTERVAL}s..."
        sleep $HEALTH_CHECK_INTERVAL
    done
    
    error "Health check failed for ${service_name}-${env}"
}

# Run smoke tests
smoke_test() {
    local env=$1
    
    log "Running smoke tests against ${env} environment"
    
    # Run basic API tests
    docker run --rm \
        --network dno-network \
        -v $PWD/tests/smoke:/tests \
        postman/newman:alpine \
        run /tests/api-smoke-tests.json \
        --environment /tests/${ENV}-${env}.json \
        --bail
}

# Switch traffic to new environment
switch_traffic() {
    local from_env=$1
    local to_env=$2
    
    log "Switching traffic from ${from_env} to ${to_env}"
    
    # Update HAProxy configuration
    docker service update \
        --config-rm haproxy.cfg \
        --config-add source=haproxy-${to_env}.cfg,target=/usr/local/etc/haproxy/haproxy.cfg \
        haproxy
    
    # Reload HAProxy
    docker exec $(docker ps -q -f name=haproxy) haproxy -f /usr/local/etc/haproxy/haproxy.cfg -sf $(cat /var/run/haproxy.pid)
}

# Main deployment flow
main() {
    log "Starting blue-green deployment for ${SERVICE} in ${ENV}"
    
    # Determine active and target environments
    ACTIVE_ENV=$(get_active_env)
    TARGET_ENV=$([[ "$ACTIVE_ENV" == "blue" ]] && echo "green" || echo "blue")
    
    log "Active environment: ${ACTIVE_ENV}, Target environment: ${TARGET_ENV}"
    
    # Deploy services
    if [[ "$SERVICE" == "all" ]]; then
        deploy_to_env $TARGET_ENV "api-server"
        deploy_to_env $TARGET_ENV "website-server"
        deploy_to_env $TARGET_ENV "crawler-service"
        
        # Health checks
        health_check "api-server" $TARGET_ENV "3000"
        health_check "website-server" $TARGET_ENV "8000"
    else
        deploy_to_env $TARGET_ENV $SERVICE
        
        # Service-specific health check
        case $SERVICE in
            api-server)
                health_check $SERVICE $TARGET_ENV "3000"
                ;;
            website-server)
                health_check $SERVICE $TARGET_ENV "8000"
                ;;
        esac
    fi
    
    # Run smoke tests
    smoke_test $TARGET_ENV
    
    # Switch traffic
    switch_traffic $ACTIVE_ENV $TARGET_ENV
    
    success "Deployment completed! Traffic now served by ${TARGET_ENV} environment"
    
    # Optional: Keep old environment running for quick rollback
    log "Old environment (${ACTIVE_ENV}) kept running for potential rollback"
    log "To remove old environment, run: ./scripts/cleanup-env.sh ${ACTIVE_ENV}"
}

# Execute main function
main
```

### Rollback Script (`scripts/rollback.sh`)

```bash
#!/bin/bash
set -e

ENV=${1:-prod}
ROLLBACK_TO=${2:-previous}

log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $1"
}

# Get deployment history
get_deployment_history() {
    docker service ls --format "table {{.Name}}\t{{.Image}}\t{{.UpdatedAt}}" | grep -E "api-server|website-server|crawler-service"
}

# Rollback service
rollback_service() {
    local service=$1
    
    log "Rolling back ${service}..."
    
    if [[ "$ROLLBACK_TO" == "previous" ]]; then
        docker service rollback ${service}
    else
        docker service update --image ${ROLLBACK_TO} ${service}
    fi
}

# Main rollback process
main() {
    log "Starting rollback for environment: ${ENV}"
    
    # Show current deployment state
    log "Current deployment state:"
    get_deployment_history
    
    # Confirm rollback
    read -p "Are you sure you want to rollback? (yes/no): " confirm
    if [[ "$confirm" != "yes" ]]; then
        log "Rollback cancelled"
        exit 0
    fi
    
    # Perform rollback
    rollback_service "api-server"
    rollback_service "website-server"
    rollback_service "crawler-service"
    
    # Wait for services to stabilize
    log "Waiting for services to stabilize..."
    sleep 30
    
    # Run health checks
    ./scripts/health-check.sh $ENV
    
    log "Rollback completed!"
}

main
```

## Backup & Disaster Recovery

### Comprehensive Backup Script (`scripts/backup.sh`)

```bash
#!/bin/bash
set -e

BACKUP_TYPE=${1:-all}
BACKUP_TAG=${2:-scheduled}
S3_BUCKET=${S3_BUCKET:-s3://dno-crawler-backups}
LOCAL_BACKUP_DIR="/backups"
RETENTION_DAYS=${RETENTION_DAYS:-30}

# Backup functions
backup_database() {
    log "Starting database backup..."
    
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    BACKUP_FILE="${LOCAL_BACKUP_DIR}/db/postgres_${BACKUP_TAG}_${TIMESTAMP}.sql.gz"
    
    # Create backup with custom format for faster restore
    docker exec postgres-primary pg_dump \
        -U ${POSTGRES_USER} \
        -d ${POSTGRES_DB} \
        --format=custom \
        --compress=9 \
        --no-owner \
        --no-privileges \
        --verbose \
        --file=/tmp/backup.dump
    
    # Copy backup from container
    docker cp postgres-primary:/tmp/backup.dump ${BACKUP_FILE%.gz}
    gzip ${BACKUP_FILE%.gz}
    
    # Upload to S3
    aws s3 cp ${BACKUP_FILE} ${S3_BUCKET}/database/ \
        --storage-class STANDARD_IA \
        --metadata "timestamp=${TIMESTAMP},tag=${BACKUP_TAG}"
    
    # Verify backup
    if gunzip -t ${BACKUP_FILE}; then
        success "Database backup completed: ${BACKUP_FILE}"
    else
        error "Database backup verification failed!"
    fi
}

backup_files() {
    log "Starting file storage backup..."
    
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    BACKUP_FILE="${LOCAL_BACKUP_DIR}/files/storage_${BACKUP_TAG}_${TIMESTAMP}.tar.gz"
    
    # Create incremental backup using rsync
    rsync -avz --delete \
        /app/storage/ \
        ${LOCAL_BACKUP_DIR}/files/current/
    
    # Create tar archive
    tar -czf ${BACKUP_FILE} \
        -C ${LOCAL_BACKUP_DIR}/files \
        current/
    
    # Upload to S3
    aws s3 cp ${BACKUP_FILE} ${S3_BUCKET}/files/ \
        --storage-class GLACIER \
        --metadata "timestamp=${TIMESTAMP},tag=${BACKUP_TAG}"
    
    success "File storage backup completed: ${BACKUP_FILE}"
}

backup_configuration() {
    log "Starting configuration backup..."
    
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    BACKUP_FILE="${LOCAL_BACKUP_DIR}/config/config_${BACKUP_TAG}_${TIMESTAMP}.tar.gz"
    
    # Backup all configuration files
    tar -czf ${BACKUP_FILE} \
        docker-compose.yml \
        docker-compose.*.yml \
        .env.* \
        docker/ \
        scripts/ \
        migrations/
    
    # Encrypt sensitive configuration
    gpg --encrypt --recipient backup@yourdomain.com ${BACKUP_FILE}
    
    # Upload to S3
    aws s3 cp ${BACKUP_FILE}.gpg ${S3_BUCKET}/config/ \
        --storage-class STANDARD \
        --server-side-encryption AES256
    
    success "Configuration backup completed: ${BACKUP_FILE}"
}

backup_volumes() {
    log "Starting Docker volumes backup..."
    
    TIMESTAMP=$(date +%Y%m%d_%H%M%S)
    
    # Backup each volume
    for volume in $(docker volume ls -q | grep dno); do
        BACKUP_FILE="${LOCAL_BACKUP_DIR}/volumes/${volume}_${BACKUP_TAG}_${TIMESTAMP}.tar.gz"
        
        # Create volume backup using temporary container
        docker run --rm \
            -v ${volume}:/data \
            -v ${LOCAL_BACKUP_DIR}/volumes:/backup \
            alpine \
            tar -czf /backup/$(basename ${BACKUP_FILE}) -C /data .
        
        # Upload to S3
        aws s3 cp ${BACKUP_FILE} ${S3_BUCKET}/volumes/
    done
    
    success "Docker volumes backup completed"
}

# Cleanup old backups
cleanup_old_backups() {
    log "Cleaning up old backups..."
    
    # Local cleanup
    find ${LOCAL_BACKUP_DIR} -type f -mtime +${RETENTION_DAYS} -delete
    
    # S3 lifecycle policy handles cloud cleanup
    # But we can also do manual cleanup if needed
    aws s3 ls ${S3_BUCKET} --recursive | \
        awk '{print $4}' | \
        while read file; do
            mod_date=$(aws s3api head-object --bucket ${S3_BUCKET##*/} --key "$file" --query LastModified --output text)
            if [[ $(date -d "$mod_date" +%s) -lt $(date -d "${RETENTION_DAYS} days ago" +%s) ]]; then
                aws s3 rm "${S3_BUCKET}/${file}"
            fi
        done
}

# Disaster recovery test
test_restore() {
    log "Testing disaster recovery..."
    
    # Create test environment
    docker-compose -f docker-compose.test.yml up -d
    
    # Restore latest backup
    ./scripts/restore.sh test latest
    
    # Run validation tests
    docker run --rm \
        --network test_network \
        -v $PWD/tests/dr:/tests \
        alpine/httpie \
        sh /tests/validate-restore.sh
    
    # Cleanup test environment
    docker-compose -f docker-compose.test.yml down -v
}

# Main execution
case $BACKUP_TYPE in
    db|database)
        backup_database
        ;;
    files|storage)
        backup_files
        ;;
    config)
        backup_configuration
        ;;
    volumes)
        backup_volumes
        ;;
    all)
        backup_database
        backup_files
        backup_configuration
        backup_volumes
        cleanup_old_backups
        ;;
    test)
        test_restore
        ;;
    *)
        echo "Usage: $0 [db|files|config|volumes|all|test] [tag]"
        exit 1
        ;;
esac

# Send notification
if command -v notify &> /dev/null; then
    notify "Backup completed" "Type: ${BACKUP_TYPE}, Tag: ${BACKUP_TAG}"
fi
```

### Disaster Recovery Runbook (`docs/disaster-recovery.md`)

```markdown
# Disaster Recovery Runbook

## Overview

This runbook provides step-by-step instructions for recovering the DNO Crawler system from various failure scenarios.

## Recovery Time Objectives (RTO) and Recovery Point Objectives (RPO)

- **RTO**: 2 hours
- **RPO**: 1 hour for database, 24 hours for files

## Failure Scenarios

### 1. Single Service Failure

**Symptoms**: One service (API, Website, or Crawler) is down

**Recovery Steps**:
1. Check service health: `docker service ps <service-name>`
2. View logs: `docker service logs <service-name>`
3. Restart service: `docker service update --force <service-name>`
4. If persistent, rollback: `docker service rollback <service-name>`

### 2. Database Failure

**Symptoms**: Database connection errors, data inconsistency

**Recovery Steps**:
1. Check PostgreSQL status: `./scripts/health-check.sh prod`
2. If primary failed, promote replica:
   ```bash
   docker exec postgres-replica pg_ctl promote
   docker service update --env-add PRIMARY_HOST=postgres-replica api-server
   ```
3. Restore from backup if needed:
   ```bash
   ./scripts/restore.sh prod database latest
   ```

### 3. Complete System Failure

**Symptoms**: All services down, infrastructure failure

**Recovery Steps**:
1. Provision new infrastructure:
   ```bash
   cd terraform/
   terraform init
   terraform apply -auto-approve
   ```

2. Restore from backups:
   ```bash
   ./scripts/restore.sh prod all latest
   ```

3. Verify system integrity:
   ```bash
   ./scripts/post-deploy-test.sh prod
   ```

### 4. Data Corruption

**Symptoms**: Invalid data, integrity constraint violations

**Recovery Steps**:
1. Identify corruption scope:
   ```sql
   -- Check for integrity violations
   SELECT * FROM pg_stat_database_conflicts;
   ```

2. Restore affected tables:
   ```bash
   ./scripts/restore.sh prod database latest --tables=affected_table
   ```

3. Run data validation:
   ```bash
   docker exec api-server ./validate-data
   ```

## Monitoring During Recovery

1. Watch service status:
   ```bash
   watch -n 5 'docker service ls'
   ```

2. Monitor logs:
   ```bash
   docker service logs -f api-server
   ```

3. Check metrics:
   - Grafana: https://grafana.yourdomain.com
   - Prometheus: https://prometheus.yourdomain.com

## Post-Recovery Validation

1. Run health checks:
   ```bash
   ./scripts/health-check.sh prod
   ```

2. Execute smoke tests:
   ```bash
   ./scripts/smoke-test.sh prod
   ```

3. Verify data integrity:
   ```bash
   ./scripts/verify-data.sh
   ```

4. Check monitoring alerts

## Communication Plan

1. **Initial Alert**: Notify on-call engineer via PagerDuty
2. **Status Updates**: Post to #incidents Slack channel every 30 minutes
3. **Stakeholder Communication**: Email updates to stakeholders list
4. **Post-Mortem**: Schedule within 48 hours of incident resolution

## Contact Information

- **On-Call Engineer**: See PagerDuty schedule
- **Database Admin**: dba@yourdomain.com
- **Infrastructure Team**: infra@yourdomain.com
- **Management**: management@yourdomain.com
```

## API Documentation

### OpenAPI Specification Integration

```rust
// crates/api/src/docs.rs
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        health::health_check,
        health::ready_check,
        auth::login,
        auth::refresh,
        auth::logout,
        users::get_users,
        users::get_user,
        users::create_user,
        users::update_user,
        users::delete_user,
        dno::search_dno,
        dno::get_dno_data,
        crawler::trigger_crawl,
        crawler::get_crawl_status,
    ),
    components(
        schemas(
            User,
            UserCreate,
            UserUpdate,
            LoginRequest,
            LoginResponse,
            DnoSearchRequest,
            DnoSearchResponse,
            DnoData,
            CrawlRequest,
            CrawlStatus,
            ErrorResponse,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management"),
        (name = "dno", description = "DNO data operations"),
        (name = "crawler", description = "Crawler operations"),
        (name = "health", description = "Health checks"),
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::ApiKey(ApiKey {
                location: ApiKeyLocation::Header,
                name: "Authorization".to_string(),
                description: Some("Bearer token authentication".to_string()),
            }),
        );
    }
}

// Serve OpenAPI documentation
pub fn openapi_routes() -> Router {
    Router::new()
        .route("/api/docs/openapi.json", get(openapi_json))
        .route("/api/docs", get(swagger_ui))
        .route("/api/docs/redoc", get(redoc))
}

async fn openapi_json() -> impl IntoResponse {
    Json(ApiDoc::openapi())
}

async fn swagger_ui() -> impl IntoResponse {
    Html(include_str!("../assets/swagger-ui.html"))
}

async fn redoc() -> impl IntoResponse {
    Html(include_str!("../assets/redoc.html"))
}
```

### API Versioning Strategy

```rust
// crates/api/src/versioning.rs
use axum::{
    extract::{Path, Query},
    http::header,
    middleware::{self, Next},
    response::Response,
};

#[derive(Debug, Clone)]
pub struct ApiVersion(pub String);

pub async fn version_middleware(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<Response, StatusCode> {
    // Extract version from header or path
    let version = req
        .headers()
        .get("API-Version")
        .and_then(|v| v.to_str().ok())
        .or_else(|| {
            req.uri()
                .path()
                .split('/')
                .find(|s| s.starts_with("v"))
        })
        .unwrap_or("v1")
        .to_string();
    
    // Add version to request extensions
    req.extensions_mut().insert(ApiVersion(version));
    
    Ok(next.run(req).await)
}

// Route versioning
pub fn versioned_routes() -> Router {
    Router::new()
        .nest("/v1", v1::routes())
        .nest("/v2", v2::routes())
        .layer(middleware::from_fn(version_middleware))
}
```

## Operational Runbooks

### Daily Operations Checklist

```markdown
# Daily Operations Checklist

## Morning Checks (9:00 AM)

- [ ] Check system health dashboard
- [ ] Review overnight alerts
- [ ] Verify backup completion
- [ ] Check disk space usage
- [ ] Review error logs for anomalies
- [ ] Verify crawler job completion rate
- [ ] Check database replication lag
- [ ] Review security alerts

## Midday Checks (2:00 PM)

- [ ] Monitor API response times
- [ ] Check memory usage trends
- [ ] Verify cache hit rates
- [ ] Review user-reported issues
- [ ] Check external service status

## End of Day (6:00 PM)

- [ ] Review daily metrics summary
- [ ] Check scheduled job status
- [ ] Verify no stuck crawler jobs
- [ ] Review capacity planning metrics
- [ ] Update on-call handover notes
```

### Incident Response Procedure

```markdown
# Incident Response Procedure

## Severity Levels

- **P1 (Critical)**: Complete service outage, data loss risk
- **P2 (High)**: Significant functionality impaired
- **P3 (Medium)**: Performance degradation, non-critical features affected
- **P4 (Low)**: Minor issues, cosmetic problems

## Response Times

- P1: Immediate response, 15-minute acknowledgment
- P2: 30-minute response, 1-hour acknowledgment  
- P3: 2-hour response, 4-hour acknowledgment
- P4: Next business day

## Incident Commander Responsibilities

1. **Assess** the situation and declare incident severity
2. **Assemble** response team based on severity
3. **Communicate** status updates every 30 minutes
4. **Coordinate** technical response efforts
5. **Document** timeline and actions taken
6. **Conduct** post-mortem after resolution

## Communication Templates

### Initial Alert
```
INCIDENT DECLARED: [P1/P2/P3/P4]
Time: [timestamp]
Summary: [brief description]
Impact: [affected services/users]
IC: [incident commander name]
Status Page: [updated/will update]
Next Update: [time]
```

### Status Update
```
INCIDENT UPDATE: [incident ID]
Time: [timestamp]
Current Status: [investigating/identified/monitoring/resolved]
Actions Taken: [list]
Next Steps: [list]
ETA: [if available]
Next Update: [time]
```

### Resolution Notice
```
INCIDENT RESOLVED: [incident ID]
Time Resolved: [timestamp]
Duration: [total time]
Root Cause: [brief summary]
Impact: [final assessment]
Post-Mortem: [scheduled date/time]
```
```

## Getting Started Guide

### Prerequisites

1. **Infrastructure Requirements**:
   - Docker 24.0+ with Swarm mode or Kubernetes 1.28+
   - 3+ nodes for HA (8 CPU, 32GB RAM each minimum)
   - 500GB+ SSD storage per node
   - Load balancer (HAProxy/NGINX)
   - SSL certificates (or Let's Encrypt setup)

2. **External Services**:
   - AWS S3 or compatible object storage
   - SMTP server for notifications
   - Sentry account (optional)
   - PagerDuty account (optional)

3. **Development Tools**:
   - Rust 1.75+
   - Docker & Docker Compose
   - PostgreSQL client tools
   - Redis client tools

### Initial Setup

```bash
# 1. Clone repository
git clone https://github.com/yourusername/rust-dno-crawler.git
cd rust-dno-crawler

# 2. Setup environment
cp .env.example .env.prod
# Edit .env.prod with your values

# 3. Generate secrets
./scripts/generate-secrets.sh

# 4. Setup infrastructure
./scripts/setup-infra.sh prod

# 5. Initialize database
./scripts/db-migrate.sh prod up

# 6. Deploy services
./scripts/deploy.sh prod

# 7. Verify deployment
./scripts/health-check.sh prod
./scripts/smoke-test.sh prod

# 8. Setup monitoring
./scripts/setup-monitoring.sh

# 9. Configure backups
./scripts/setup-backups.sh

# 10. Setup SSL
./scripts/ssl-management.sh yourdomain.com admin@yourdomain.com
```

### First-Time Configuration

1. **Create admin user**:
   ```bash
   docker exec api-server ./create-admin --email admin@yourdomain.com
   ```

2. **Configure crawler schedules**:
   ```bash
   docker exec crawler-service ./schedule-jobs --config /app/crawler-schedule.yml
   ```

3. **Import initial DNO data**:
   ```bash
   docker exec api-server ./import-dnos --file /app/data/dnos.csv
   ```

## Maintenance & Troubleshooting

### Common Issues and Solutions

1. **High Memory Usage**
   ```bash
   # Check memory usage
   docker stats
   
   # Restart service with memory limit
   docker service update --limit-memory 4G api-server
   ```

2. **Database Connection Pool Exhausted**
   ```sql
   -- Check active connections
   SELECT count(*) FROM pg_stat_activity;
   
   -- Kill idle connections
   SELECT pg_terminate_backend(pid) 
   FROM pg_stat_activity 
   WHERE state = 'idle' 
   AND state_change < current_timestamp - INTERVAL '1 hour';
   ```

3. **Crawler Jobs Stuck**
   ```bash
   # Check job queue
   docker exec redis-node-1 redis-cli -a $REDIS_PASSWORD LLEN crawler_jobs
   
   # Clear dead letter queue
   docker exec crawler-service ./clear-dlq --confirm
   ```

## Performance Tuning Guide

### Database Optimization

```sql
-- Create indexes for common queries
CREATE INDEX CONCURRENTLY idx_dno_data_year_type 
ON dno_data(year, data_type);

CREATE INDEX CONCURRENTLY idx_users_email 
ON users(email);

-- Analyze tables
ANALYZE dno_data;
ANALYZE users;

-- Check slow queries
SELECT query, calls, mean_exec_time
FROM pg_stat_statements
ORDER BY mean_exec_time DESC
LIMIT 10;
```

### Redis Optimization

```bash
# Monitor Redis performance
redis-cli -a $REDIS_PASSWORD INFO stats

# Optimize memory usage
redis-cli -a $REDIS_PASSWORD CONFIG SET maxmemory-policy allkeys-lru

# Enable RDB + AOF persistence
redis-cli -a $REDIS_PASSWORD CONFIG SET save "900 1 300 10 60 10000"
redis-cli -a $REDIS_PASSWORD CONFIG SET appendonly yes
```

## Security Hardening Checklist

- [ ] Enable firewall rules
- [ ] Configure fail2ban
- [ ] Enable audit logging
- [ ] Implement RBAC
- [ ] Rotate all secrets
- [ ] Enable 2FA for admin accounts
- [ ] Configure CORS properly
- [ ] Implement rate limiting
- [ ] Enable HTTPS only
- [ ] Regular security scans
- [ ] Dependency updates
- [ ] Penetration testing

## Compliance & Regulations

### GDPR Compliance

- Implement data retention policies
- Provide data export functionality
- Enable data deletion (right to be forgotten)
- Maintain processing activity records
- Implement consent management
- Regular privacy impact assessments

### Security Standards

- Follow OWASP Top 10 guidelines
- Implement CIS Docker Benchmark
- Regular vulnerability scanning
- Maintain security incident log
- Annual security audit

This enhanced deployment plan provides a production-ready blueprint for your DNO crawler system with enterprise-grade features including high availability, comprehensive monitoring, automated deployments, and robust security measures.</content>
</invoke>