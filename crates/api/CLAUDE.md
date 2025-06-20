# DNO Crawler API Specification

## Overview
RESTful API for the DNO (Distribution Network Operator) data crawler system. All endpoints return JSON unless specified otherwise.

**Base URL:** `https://api.dno-crawler.de/v1`  
**Authentication:** Bearer token in `Authorization` header

## User Roles & Authentication

### User Roles
- **pending**: New registered users awaiting admin approval or payment verification
- **user**: Verified users with access to dashboard, queries, and account management
- **admin**: Full system access including user management, data verification, and system settings

### Authentication Requirements
- **Public endpoints**: `/auth/*`, `/health`, `/ready`
- **User endpoints**: Require `user` or `admin` role
- **Admin endpoints**: Require `admin` role only
- **Pending users**: Can only access `/auth/logout` and `/account/profile` (read-only)

## Core Endpoints

### Health & Status
```http
GET    /health                - API health check (public)
GET    /ready                 - API readiness check (public)
```

### Authentication

```http
POST   /auth/register          - Register new user
POST   /auth/login            - Login user
POST   /auth/refresh          - Refresh access token
POST   /auth/logout           - Logout user
```

### Search & Dashboard (User Auth Required)

```http
POST   /search/dno            - Search by DNO name or ID
POST   /search/year           - Search by year
POST   /search/data-type      - Search by data type
GET    /search/               - Search with filters
GET    /dashboard/stats       - Dashboard statistics
GET    /dashboard/history     - Query history
DELETE /dashboard/history/{id} - Delete history entry
```

### User Account

```http
GET    /account/profile       - Get profile (pending users: read-only)
PATCH  /account/profile       - Update profile (user/admin only)
POST   /account/change-email  - Change email (user/admin only)
POST   /account/change-password - Change password (user/admin only)
POST   /account/profile-picture - Upload picture (user/admin only)
DELETE /account/profile-picture - Delete picture (user/admin only)
POST   /account/api-keys      - Create API key (user/admin only)
GET    /account/api-keys      - List API keys (user/admin only)
DELETE /account/api-keys/{id} - Delete API key (user/admin only)
DELETE /account              - Delete account (user/admin only)
```

### Admin - User Management (Admin Auth Required)

```http
GET    /admin/overview        - System overview
GET    /admin/users          - List users
PATCH  /admin/users/{id}     - Update user (including role changes)
DELETE /admin/users/{id}     - Delete user
POST   /admin/users/{id}/approve - Approve pending user
POST   /admin/users/{id}/reject  - Reject pending user
```

### Admin - Data Verification

```http
GET    /admin/data-entries    - List all data entries with sources
GET    /admin/data-entries/{id} - Get single entry with source
GET    /admin/data-entries/{id}/source - Get source file/preview
POST   /admin/data-entries/{id}/verify - Verify data entry
PATCH  /admin/data-entries/{id} - Update data entry
DELETE /admin/data-entries/{id} - Delete data entry
POST   /admin/data-entries/bulk - Bulk operations
```

### Admin - System Management

```http
GET    /admin/crawl-settings - Get settings
PATCH  /admin/crawl-settings - Update settings
GET    /admin/queries        - Query tracking
GET    /admin/cache/status   - Cache status
POST   /admin/cache/clear    - Clear cache
GET    /admin/jobs/automated - List automated jobs
POST   /admin/jobs/automated - Create automated job
GET    /admin/logs          - System logs
POST   /admin/crawl/trigger - Manual crawl
```

### Admin - Metrics & Monitoring (Admin Auth Required)

```http
GET    /metrics              - Prometheus-compatible metrics
GET    /admin/metrics/dashboard - Dashboard metrics data
POST   /admin/metrics/query  - Custom metric queries
GET    /admin/metrics/export - Export metrics
GET    /admin/metrics/timeseries - Time series data
```

### Files & WebSocket

```http
GET    /files/{type}/{id}    - Download file
WSS    /ws                   - WebSocket connection
```

## Request/Response Examples

### Register User
```json
POST /auth/register
{
  "email": "user@example.com",
  "password": "password123",
  "name": "John Doe"
}

Response 201:
{
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "name": "John Doe",
    "role": "pending",
    "created_at": "2024-01-15T09:00:00Z",
    "verification_status": "awaiting_approval"
  },
  "tokens": {
    "access_token": "eyJhbGciOiJIUzI1NiIs...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIs...",
    "expires_in": 3600
  },
  "message": "Account created successfully. Awaiting admin approval to access full features."
}
```

### Health Check
```json
GET /health

Response 200:
{
  "status": "ok",
  "timestamp": "2024-01-15T15:00:00Z",
  "version": "1.0.0"
}
```

### Readiness Check
```json
GET /ready

Response 200:
{
  "status": "ready",
  "services": {
    "database": "ok",
    "cache": "ok",
    "storage": "ok"
  },
  "timestamp": "2024-01-15T15:00:00Z"
}
```

### Search by DNO
```json
POST /search/dno
{
  "dno_name": "Netze BW",
  "year": 2024,
  "data_type": "netzentgelte"
}

Response 200:
{
  "total": 1,
  "results": [{
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "dno": {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "name": "Netze BW",
      "slug": "netze-bw",
      "region": "Baden-Württemberg"
    },
    "year": 2024,
    "data_type": "netzentgelte",
    "status": "verified",
    "data": {
      "netzentgelte": {
        "hs": {"leistung": 58.21, "arbeit": 1.26},
        "ms": {"leistung": 109.86, "arbeit": 1.73}
      }
    },
    "source": {
      "id": "660e8400-e29b-41d4-a716-446655440000",
      "file_type": "pdf",
      "file_url": "/files/pdf/660e8400-e29b-41d4-a716-446655440000",
      "page": 12,
      "extracted_at": "2024-01-15T10:00:00Z"
    },
    "last_updated": "2024-01-15T10:00:00Z"
  }],
  "filters_applied": {
    "dno_name": "Netze BW",
    "year": 2024,
    "data_type": "netzentgelte"
  },
  "available_years": [2022, 2023, 2024],
  "available_dnos": [{
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "name": "Netze BW",
    "slug": "netze-bw",
    "region": "Baden-Württemberg"
  }]
}
```

### Search by Year
```json
POST /search/year
{
  "year": 2024,
  "data_type": "netzentgelte"
}

Response 200:
{
  "total": 2,
  "results": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "dno": {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "name": "Netze BW",
        "slug": "netze-bw",
        "region": "Baden-Württemberg"
      },
      "year": 2024,
      "data_type": "netzentgelte",
      "status": "verified",
      "data": {
        "netzentgelte": {
          "hs": {"leistung": 58.21, "arbeit": 1.26},
          "ms": {"leistung": 109.86, "arbeit": 1.73}
        }
      },
      "source": {
        "id": "660e8400-e29b-41d4-a716-446655440000",
        "file_type": "pdf",
        "file_url": "/files/pdf/660e8400-e29b-41d4-a716-446655440000",
        "page": 12,
        "extracted_at": "2024-01-15T10:00:00Z"
      },
      "last_updated": "2024-01-15T10:00:00Z"
    }
  ],
  "filters_applied": {
    "year": 2024,
    "data_type": "netzentgelte"
  },
  "available_years": [2022, 2023, 2024],
  "available_dnos": [
    {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "name": "Netze BW",
      "slug": "netze-bw",
      "region": "Baden-Württemberg"
    }
  ]
}
```

### Search with Filters
```json
GET /search/?dno_name=Netze BW&year=2024&data_type=netzentgelte&limit=10&offset=0

Response 200:
{
  "total": 150,
  "results": [{
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "dno": {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "name": "Netze BW",
      "slug": "netze-bw",
      "region": "Baden-Württemberg"
    },
    "year": 2024,
    "data_type": "netzentgelte",
    "status": "verified",
    "data": {
      "netzentgelte": {
        "hs": {"leistung": 58.21, "arbeit": 1.26},
        "ms": {"leistung": 109.86, "arbeit": 1.73}
      }
    },
    "source": {
      "id": "660e8400-e29b-41d4-a716-446655440000",
      "file_type": "pdf",
      "file_url": "/files/pdf/660e8400-e29b-41d4-a716-446655440000",
      "page": 12,
      "extracted_at": "2024-01-15T10:00:00Z"
    },
    "last_updated": "2024-01-15T10:00:00Z"
  }],
  "pagination": {
    "limit": 10,
    "offset": 0,
    "total": 150,
    "has_more": true
  },
  "filters_applied": {
    "dno_name": "Netze BW",
    "year": 2024,
    "data_type": "netzentgelte",
    "limit": 10,
    "offset": 0
  },
  "available_filters": {
    "years": [2022, 2023, 2024],
    "data_types": ["netzentgelte", "hlzf"],
    "regions": ["Baden-Württemberg", "Bayern", "Nordrhein-Westfalen"]
  }
}
```

### Dashboard Statistics
```json
GET /dashboard/stats

Response 200:
{
  "user_stats": {
    "queries_today": 12,
    "queries_this_month": 156,
    "last_query": "2024-01-15T14:30:00Z",
    "favorite_dnos": ["Netze BW", "Bayernwerk"]
  },
  "system_stats": {
    "total_dnos": 850,
    "total_data_entries": 15420,
    "data_coverage": {
      "2024": 782,
      "2023": 845,
      "2022": 850
    },
    "last_system_update": "2024-01-15T03:00:00Z"
  },
  "active_jobs": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "dno": "Netze BW",
      "year": 2024,
      "progress": 65,
      "status": "extracting"
    }
  ]
}
```

### List Data Entries (Admin)
```json
GET /admin/data-entries?dno_id=123e4567&year=2024&status=unverified

Response 200:
{
  "total": 1234,
  "entries": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "dno": {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "name": "Netze BW",
        "slug": "netze-bw"
      },
      "year": 2024,
      "data_type": "netzentgelte",
      "status": "unverified",
      "storage": {
        "hs": {
          "leistung": 58.21,
          "arbeit": 1.26
        }
      },
      "source": {
        "id": "660e8400-e29b-41d4-a716-446655440000",
        "type": "pdf",
        "file_url": "/admin/storage-entries/550e8400/source",
        "page": 12,
        "confidence": 0.98
      },
      "verification": {
        "status": "unverified",
        "verified_by": null,
        "verified_at": null
      }
    }
  ]
}
```

### Verify Data Entry (Admin)
```json
POST /admin/data-entries/{entry_id}/verify
{
  "status": "verified",
  "notes": "Manually checked against source PDF page 12"
}

Response 200:
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "verification": {
    "status": "verified",
    "verified_by": "admin@example.com",
    "verified_at": "2024-01-15T15:00:00Z",
    "notes": "Manually checked against source PDF page 12"
  }
}
```

### Admin Overview
```json
GET /admin/overview

Response 200:
{
  "system_health": {
    "status": "healthy",
    "uptime_seconds": 864000,
    "crawler_status": "active",
    "queue_size": 5,
    "active_workers": 3
  },
  "statistics": {
    "total_users": 1523,
    "active_users_24h": 234,
    "total_queries_24h": 3421,
    "cache_hit_rate": 0.82,
    "average_crawl_time": 87.5,
    "failed_crawls_24h": 12
  },
  "verification_stats": {
    "total_entries": 25420,
    "verified": 21450,
    "unverified": 3736,
    "disputed": 234
  }
}
```

### Prometheus Metrics
```http
GET /metrics

Response 200:
Content-Type: text/plain; version=0.0.4

# HELP dno_crawler_queries_total Total number of queries
# TYPE dno_crawler_queries_total counter
dno_crawler_queries_total{status="success",cache="hit"} 2805
dno_crawler_queries_total{status="success",cache="miss"} 616

# HELP dno_crawler_data_entries Total data entries in system
# TYPE dno_crawler_data_entries gauge
dno_crawler_data_entries{type="netzentgelte",verified="true"} 12450
dno_crawler_data_entries{type="netzentgelte",verified="false"} 2970
```

## Error Handling

All errors follow this format:
```json
{
  "error": "error_code",
  "message": "Human readable message",
  "details": {},
  "request_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### Authentication Errors

**Pending User Access Denied (403):**
```json
{
  "error": "access_denied",
  "message": "Account pending approval. Contact admin for verification.",
  "details": {
    "role": "pending",
    "verification_status": "awaiting_approval"
  },
  "request_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Admin Only Access (403):**
```json
{
  "error": "admin_required",
  "message": "This endpoint requires admin privileges",
  "details": {
    "required_role": "admin",
    "current_role": "user"
  },
  "request_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Status Codes:**
- 200: Success
- 201: Created
- 204: No Content
- 400: Bad Request
- 401: Unauthorized (invalid/missing token)
- 403: Forbidden (insufficient permissions, pending approval, admin only)
- 404: Not Found
- 429: Too Many Requests
- 500: Internal Server Error

## Headers

**Request Headers:**
```
Authorization: Bearer <token>
Content-Type: application/json
```

**Response Headers:**
```
X-Request-ID: 550e8400-e29b-41d4-a716-446655440000
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 995
X-RateLimit-Reset: 1642370400
```

## WebSocket Events

**Client Subscribe:**
```json
{
  "type": "subscribe",
  "events": ["job_progress", "job_complete"]
}
```

**Server Events:**
```json
{
  "type": "job_progress",
  "data": {
    "job_id": "550e8400-e29b-41d4-a716-446655440000",
    "progress": 75,
    "status": "extracting_data"
  }
}
```

## Implementation Notes

### Database Tables Required
- users (id, email, password_hash, name, role, created_at)
- sessions (id, user_id, token, expires_at)
- api_keys (id, user_id, name, key_hash, created_at)
- queries (id, user_id, query, interpretation, result, timestamp)
- crawl_jobs (id, dno, year, status, progress, created_at)
- dno_data (id, dno_id, year, data_type, data, source_url, extracted_at)
- netzentgelte_data (with verification fields)
- hlzf_data (with verification fields)
- data_entry_history (audit trail)
- metrics (prometheus data)
- cache_entries (key, value, expires_at)

### Core Dependencies
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls", "uuid", "chrono"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
jsonwebtoken = "9"
uuid = { version = "1", features = ["serde", "v4"] }
chrono = { version = "0.4", features = ["serde"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }
bcrypt = "0.15"
redis = { version = "0.24", features = ["tokio-comp"] }
prometheus = "0.13"
```

### Environment Variables
```env
DATABASE_URL=postgresql://user:pass@localhost/dno_crawler
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-secret-key
API_PORT=3000
```