# Admin API Updates for Data Verification & Metrics

## Data Verification Endpoints

### List All Data Entries with Sources
```http
GET /admin/data-entries?dno_id={id}&year={year}&status={status}&limit=50&offset=0
Authorization: Bearer <token> (admin only)

Query Parameters:
- dno_id: UUID (optional) - Filter by DNO
- year: integer (optional) - Filter by year
- status: string (optional) - "verified", "unverified", "disputed"
- data_type: string (optional) - "netzentgelte", "hlzf"
- has_source: boolean (optional) - Only entries with/without source files
- limit: integer (default: 50)
- offset: integer (default: 0)

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
      "data": {
        "hs": {
          "leistung": 58.21,
          "arbeit": 1.26,
          "leistung_unter_2500h": 2.56,
          "arbeit_unter_2500h": 7.14
        },
        "ms": {
          "leistung": 109.86,
          "arbeit": 1.73,
          "leistung_unter_2500h": 4.72,
          "arbeit_unter_2500h": 10.97
        }
      },
      "source": {
        "id": "660e8400-e29b-41d4-a716-446655440000",
        "type": "pdf",
        "file_path": "assets/netze-bw/Netzentgelte Strom 2024.pdf",
        "file_url": "/admin/data-entries/550e8400/source",
        "page": 12,
        "confidence": 0.98,
        "extracted_at": "2024-01-15T10:00:00Z",
        "extraction_method": "table_recognition"
      },
      "verification": {
        "status": "unverified",
        "verified_by": null,
        "verified_at": null,
        "notes": null
      },
      "created_at": "2024-01-15T10:00:00Z",
      "updated_at": "2024-01-15T10:00:00Z"
    }
  ]
}
```

### Get Single Data Entry with Source
```http
GET /admin/data-entries/{entry_id}
Authorization: Bearer <token> (admin only)

Response 200:
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "dno": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "name": "Netze BW",
    "slug": "netze-bw",
    "website": "https://www.netze-bw.de"
  },
  "year": 2024,
  "data_type": "netzentgelte",
  "data": { ... },
  "source": {
    "id": "660e8400-e29b-41d4-a716-446655440000",
    "type": "pdf",
    "file_path": "assets/netze-bw/Netzentgelte Strom 2024.pdf",
    "file_url": "/admin/data-entries/550e8400/source",
    "page": 12,
    "extracted_region": {
      "x": 100,
      "y": 200,
      "width": 400,
      "height": 300
    },
    "ocr_text": "Original extracted text...",
    "confidence": 0.98,
    "extraction_log": [
      {
        "timestamp": "2024-01-15T10:00:00Z",
        "method": "table_recognition",
        "status": "success",
        "details": "Extracted 5 voltage levels with 4 columns each"
      }
    ]
  },
  "history": [
    {
      "version": 1,
      "changed_by": "system",
      "changed_at": "2024-01-15T10:00:00Z",
      "changes": "Initial extraction"
    }
  ]
}
```

### Get Source File/Preview
```http
GET /admin/data-entries/{entry_id}/source?page={page}&highlight=true
Authorization: Bearer <token> (admin only)

Query Parameters:
- page: integer (optional) - Specific page for PDFs
- highlight: boolean (optional) - Highlight extracted regions
- format: string (optional) - "original", "image", "text"

Response 200:
Content-Type: application/pdf (or image/png for preview)
[Binary data]
```

### Verify Data Entry
```http
POST /admin/data-entries/{entry_id}/verify
Authorization: Bearer <token> (admin only)
Content-Type: application/json

{
  "status": "verified", // or "disputed"
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

### Update Data Entry
```http
PATCH /admin/data-entries/{entry_id}
Authorization: Bearer <token> (admin only)
Content-Type: application/json

{
  "data": {
    "hs": {
      "leistung": 58.25, // Corrected value
      "arbeit": 1.26
    }
  },
  "reason": "Manual correction: OCR misread 58.21 as 58.25"
}

Response 200:
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "data": { ... updated data ... },
  "history": [
    {
      "version": 2,
      "changed_by": "admin@example.com",
      "changed_at": "2024-01-15T15:30:00Z",
      "changes": "Manual correction: OCR misread 58.21 as 58.25"
    }
  ]
}
```

### Delete Data Entry
```http
DELETE /admin/data-entries/{entry_id}
Authorization: Bearer <token> (admin only)
Content-Type: application/json

{
  "reason": "Duplicate entry",
  "soft_delete": true // Optional, default true
}

Response 200:
{
  "message": "Data entry deleted",
  "entry_id": "550e8400-e29b-41d4-a716-446655440000",
  "deleted_at": "2024-01-15T16:00:00Z"
}
```

### Bulk Operations
```http
POST /admin/data-entries/bulk
Authorization: Bearer <token> (admin only)
Content-Type: application/json

{
  "operation": "verify", // or "delete", "export"
  "entry_ids": [
    "550e8400-e29b-41d4-a716-446655440000",
    "660e8400-e29b-41d4-a716-446655440000"
  ],
  "params": {
    "status": "verified",
    "notes": "Bulk verified after manual review"
  }
}

Response 200:
{
  "operation": "verify",
  "total": 2,
  "successful": 2,
  "failed": 0,
  "results": [...]
}
```

## Prometheus-Compatible Metrics Endpoints

### Metrics Endpoint (Prometheus Format)
```http
GET /metrics
Authorization: Bearer <token> (admin only)

Response 200:
Content-Type: text/plain; version=0.0.4

# HELP dno_crawler_queries_total Total number of queries
# TYPE dno_crawler_queries_total counter
dno_crawler_queries_total{status="success",cache="hit"} 2805
dno_crawler_queries_total{status="success",cache="miss"} 616
dno_crawler_queries_total{status="error"} 45

# HELP dno_crawler_query_duration_seconds Query duration in seconds
# TYPE dno_crawler_query_duration_seconds histogram
dno_crawler_query_duration_seconds_bucket{le="0.005"} 1200
dno_crawler_query_duration_seconds_bucket{le="0.01"} 2300
dno_crawler_query_duration_seconds_bucket{le="0.025"} 2800
dno_crawler_query_duration_seconds_bucket{le="0.05"} 3000
dno_crawler_query_duration_seconds_bucket{le="0.1"} 3200
dno_crawler_query_duration_seconds_bucket{le="0.25"} 3400
dno_crawler_query_duration_seconds_bucket{le="0.5"} 3420
dno_crawler_query_duration_seconds_bucket{le="1"} 3421
dno_crawler_query_duration_seconds_bucket{le="+Inf"} 3421
dno_crawler_query_duration_seconds_sum 145.23
dno_crawler_query_duration_seconds_count 3421

# HELP dno_crawler_active_users Current number of active users
# TYPE dno_crawler_active_users gauge
dno_crawler_active_users 234

# HELP dno_crawler_crawl_jobs_total Total crawl jobs by status
# TYPE dno_crawler_crawl_jobs_total counter
dno_crawler_crawl_jobs_total{status="completed"} 1523
dno_crawler_crawl_jobs_total{status="failed"} 87
dno_crawler_crawl_jobs_total{status="cancelled"} 12

# HELP dno_crawler_data_entries Total data entries in system
# TYPE dno_crawler_data_entries gauge
dno_crawler_data_entries{type="netzentgelte",verified="true"} 12450
dno_crawler_data_entries{type="netzentgelte",verified="false"} 2970
dno_crawler_data_entries{type="hlzf",verified="true"} 8234
dno_crawler_data_entries{type="hlzf",verified="false"} 1766

# HELP dno_crawler_cache_hit_ratio Cache hit ratio
# TYPE dno_crawler_cache_hit_ratio gauge
dno_crawler_cache_hit_ratio 0.82

# HELP dno_crawler_crawler_queue_size Current crawler queue size
# TYPE dno_crawler_crawler_queue_size gauge
dno_crawler_crawler_queue_size{priority="high"} 2
dno_crawler_crawler_queue_size{priority="normal"} 8
dno_crawler_crawler_queue_size{priority="low"} 15
```

### Enhanced Metrics Dashboard Data
```http
GET /admin/metrics/dashboard
Authorization: Bearer <token> (admin only)

Response 200:
{
  "realtime": {
    "active_users": 234,
    "requests_per_minute": 145,
    "active_crawl_jobs": 5,
    "queue_sizes": {
      "high": 2,
      "normal": 8,
      "low": 15
    },
    "system_health": {
      "api": "healthy",
      "crawler": "healthy",
      "database": "healthy",
      "redis": "healthy"
    }
  },
  "timeseries": {
    "queries_per_hour": [
      {"timestamp": "2024-01-15T00:00:00Z", "value": 120},
      {"timestamp": "2024-01-15T01:00:00Z", "value": 95}
    ],
    "cache_hit_rate": [
      {"timestamp": "2024-01-15T00:00:00Z", "value": 0.85},
      {"timestamp": "2024-01-15T01:00:00Z", "value": 0.82}
    ],
    "response_times_p95": [
      {"timestamp": "2024-01-15T00:00:00Z", "value": 125},
      {"timestamp": "2024-01-15T01:00:00Z", "value": 145}
    ]
  },
  "distributions": {
    "query_types": {
      "natural_language": 2145,
      "direct_api": 1276
    },
    "data_coverage": {
      "complete": 782,
      "partial": 63,
      "missing": 5
    },
    "user_activity": {
      "daily_active": 234,
      "weekly_active": 567,
      "monthly_active": 1234
    }
  },
  "top_lists": {
    "most_queried_dnos": [
      {"name": "Netze BW", "queries": 456},
      {"name": "Bayernwerk", "queries": 389}
    ],
    "slowest_dnos": [
      {"name": "Example DNO", "avg_crawl_time_seconds": 245},
      {"name": "Another DNO", "avg_crawl_time_seconds": 198}
    ],
    "error_sources": [
      {"source": "pdf_extraction", "count": 45},
      {"source": "website_timeout", "count": 23}
    ]
  },
  "verification_stats": {
    "total_entries": 25420,
    "verified": 21450,
    "unverified": 3736,
    "disputed": 234,
    "verification_rate": 0.844,
    "pending_review": [
      {
        "dno": "Netze BW",
        "year": 2024,
        "count": 5
      }
    ]
  }
}
```

### Custom Metric Queries
```http
POST /admin/metrics/query
Authorization: Bearer <token> (admin only)
Content-Type: application/json

{
  "metric": "query_count",
  "aggregation": "sum",
  "group_by": ["dno", "hour"],
  "filters": {
    "from": "2024-01-15T00:00:00Z",
    "to": "2024-01-15T23:59:59Z",
    "cache_hit": true
  }
}

Response 200:
{
  "query": { ... },
  "results": [
    {
      "dno": "Netze BW",
      "hour": "2024-01-15T14:00:00Z",
      "value": 45
    }
  ]
}
```

### Export Metrics
```http
GET /admin/metrics/export?format=csv&from=2024-01-01&to=2024-01-31
Authorization: Bearer <token> (admin only)

Response 200:
Content-Type: text/csv
Content-Disposition: attachment; filename="metrics-2024-01.csv"

timestamp,metric,value,labels
2024-01-15T00:00:00Z,queries_total,120,"status=success,cache=hit"
2024-01-15T00:00:00Z,queries_total,15,"status=success,cache=miss"
```

## Updated SQL Schema for Data Verification

```sql
-- Add verification tracking to storage tables
ALTER TABLE netzentgelte_data ADD COLUMN verification_status VARCHAR(20) DEFAULT 'unverified';
ALTER TABLE netzentgelte_data ADD COLUMN verified_by UUID REFERENCES users(id);
ALTER TABLE netzentgelte_data ADD COLUMN verified_at TIMESTAMPTZ;
ALTER TABLE netzentgelte_data ADD COLUMN verification_notes TEXT;

ALTER TABLE hlzf_data ADD COLUMN verification_status VARCHAR(20) DEFAULT 'unverified';
ALTER TABLE hlzf_data ADD COLUMN verified_by UUID REFERENCES users(id);
ALTER TABLE hlzf_data ADD COLUMN verified_at TIMESTAMPTZ;
ALTER TABLE hlzf_data ADD COLUMN verification_notes TEXT;

-- Data entry history for audit trail
CREATE TABLE data_entry_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entry_type VARCHAR(20) NOT NULL, -- 'netzentgelte' or 'hlzf'
    entry_id UUID NOT NULL,
    version INTEGER NOT NULL,
    changed_by UUID REFERENCES users(id),
    changed_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    changes TEXT NOT NULL,
    data_before JSONB,
    data_after JSONB
);

-- Metrics table for Prometheus
CREATE TABLE metrics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    metric_name VARCHAR(255) NOT NULL,
    metric_type VARCHAR(20) NOT NULL, -- 'counter', 'gauge', 'histogram'
    value DOUBLE PRECISION NOT NULL,
    labels JSONB,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_metrics_name_timestamp ON metrics(metric_name, timestamp);
CREATE INDEX idx_metrics_labels ON metrics USING GIN(labels);

-- Add extraction details to data_sources
ALTER TABLE data_sources ADD COLUMN extraction_method VARCHAR(50);
ALTER TABLE data_sources ADD COLUMN extraction_region JSONB;
ALTER TABLE data_sources ADD COLUMN ocr_text TEXT;
ALTER TABLE data_sources ADD COLUMN extraction_log JSONB;
```

## Graph Suggestions for Admin Dashboard

1. **Real-time Activity Graph** (Line chart)
   - Queries per minute
   - Active users
   - Cache hit rate

2. **Data Coverage Heatmap**
   - X-axis: Years
   - Y-axis: DNOs
   - Color intensity: Data completeness

3. **Verification Progress** (Stacked bar chart)
   - Verified vs Unverified entries
   - By DNO or by time period

4. **System Performance** (Multi-line chart)
   - Response times (P50, P95, P99)
   - Error rates
   - Queue sizes

5. **DNO Crawl Success Rate** (Gauge charts)
   - Success rate per DNO
   - Average crawl time

6. **User Activity Distribution** (Pie/Donut chart)
   - By query type
   - By time of day
   - By DNO

These endpoints provide comprehensive data verification capabilities and Prometheus-compatible metrics for monitoring and visualization in the admin dashboard.