# Core Crate - Shared Types and Utilities

## Overview
The `core` crate provides shared types, models, database utilities, and configuration for the DNO Data Gatherer application. This crate is used by all other crates in the workspace to ensure consistency and avoid code duplication.

## Key Modules

### 📦 Models (`src/models.rs`)
Complete Rust types for all database entities:

#### User Management
- `User`, `CreateUser`, `UpdateUser` - User accounts with role-based access
- `UserRole` enum: `Pending`, `User`, `Admin`
- `UserSettings` - User preferences (language, timezone, notifications)
- `Session` - JWT session management
- `ApiKey` - API key management for programmatic access

#### DNO (Distribution Network Operator) Data
- `Dno` - DNO entities with metadata (name, region, website)
- `DnoCrawlConfig` - Crawling configuration per DNO
- `NetzentgelteData` - Network tariff data with verification status
- `HlzfData` - Main load time data with seasonal periods
- `DataSource` - Source tracking for extracted data

#### Jobs & Processing
- `CrawlJob`, `CrawlJobStep` - Crawling job management
- `AutomatedJob` - Scheduled automated tasks
- `QueryLog` - Natural language query logging
- `SystemLog` - Application logging

#### Data Types & Enums
- `JobStatus`: `Pending`, `Running`, `Completed`, `Failed`, `Cancelled`
- `CrawlType`: `File`, `Table`, `Api`
- `DataType`: `Netzentgelte`, `Hlzf`, `All`
- `Season`: `Winter`, `Fruehling`, `Sommer`, `Herbst`

### 🗄️ Database (`src/database.rs`)
Database connection and query utilities:
- PostgreSQL connection pooling with SQLx
- Transaction management
- Query builders for common operations
- Database migration helpers

### ⚙️ Configuration (`src/config.rs`)
Application configuration management:
- Environment-based configuration
- Database connection settings
- JWT secret management
- External service URLs (SearXNG, Ollama)
- Logging configuration

### 🚨 Error Handling (`src/error.rs`)
Centralized error types:
- `AppError` enum with thiserror integration
- Database error mapping
- API error responses
- Validation error handling

## Features

### Type Safety
All models use strong typing with:
- UUID primary keys
- Enum types for status fields
- Decimal types for monetary values
- DateTime with timezone support
- Optional fields with clear semantics

### Database Integration
- SQLx derive macros for FromRow
- Custom PostgreSQL enum types
- Automatic timestamp triggers
- Comprehensive indexing strategy

### Serialization
- Serde support for JSON APIs
- Field skipping for sensitive data (passwords, tokens)
- Custom serialization for complex types

### Validation
- Built-in constraints (CHECK constraints)
- Unique constraints where appropriate
- Foreign key relationships
- Data integrity through types

## Usage Examples

### Creating a DNO
```rust
use core::{CreateDno, DataType, Season};

let dno = CreateDno {
    slug: "netze-bw".to_string(),
    name: "Netze BW".to_string(),
    official_name: Some("Netze BW GmbH".to_string()),
    description: Some("Netzbetreiber in Baden-Württemberg".to_string()),
    region: Some("Baden-Württemberg".to_string()),
    website: Some("https://netze-bw.de".to_string()),
};
```

### Working with Netzentgelte Data
```rust
use core::{CreateNetzentgelteData, rust_decimal::Decimal};

let netzentgelte = CreateNetzentgelteData {
    dno_id: dno_uuid,
    year: 2024,
    voltage_level: "hs".to_string(),
    leistung: Some(Decimal::new(5821, 2)), // 58.21
    arbeit: Some(Decimal::new(126, 2)),    // 1.26
    leistung_unter_2500h: Some(Decimal::new(256, 2)),
    arbeit_unter_2500h: Some(Decimal::new(714, 2)),
};
```

### Authentication Models
```rust
use core::{LoginRequest, UserRole};

let login = LoginRequest {
    email: "user@example.com".to_string(),
    password: "secure_password".to_string(),
};

// User creation with pending approval
let user = CreateUser {
    email: "new@example.com".to_string(),
    password_hash: hashed_password,
    name: "John Doe".to_string(),
    role: Some(UserRole::Pending), // Awaits admin approval
};
```

## Database Schema Alignment

### Core Tables
The models directly correspond to PostgreSQL tables:
- `users` → `User` model
- `dnos` → `Dno` model  
- `netzentgelte_data` → `NetzentgelteData` model
- `hlzf_data` → `HlzfData` model
- `crawl_jobs` → `CrawlJob` model

### Custom PostgreSQL Types
```sql
CREATE TYPE user_role AS ENUM ('pending', 'user', 'admin');
CREATE TYPE job_status AS ENUM ('pending', 'running', 'completed', 'failed', 'cancelled');
CREATE TYPE data_type AS ENUM ('netzentgelte', 'hlzf', 'all');
```

### Verification & Audit Trail
All data models include:
- `verification_status` - Manual verification by admins
- `verified_by` - User who verified the data
- `verified_at` - Verification timestamp
- `created_at`/`updated_at` - Automatic timestamps

## Dependencies

### Core Dependencies
- `serde` - JSON serialization/deserialization
- `sqlx` - Database interactions with PostgreSQL
- `uuid` - Unique identifier generation
- `chrono` - Date/time handling with timezone support
- `rust_decimal` - Precise decimal arithmetic for monetary values
- `thiserror` - Structured error handling

### Database Features
- `sqlx` with features: `runtime-tokio-rustls`, `postgres`, `sqlite`, `uuid`, `chrono`
- Custom derive macros for database row mapping
- Compile-time query verification

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_role_serialization() {
        let role = UserRole::Admin;
        let json = serde_json::to_string(&role).unwrap();
        assert_eq!(json, "\"admin\"");
    }
    
    #[test]
    fn test_dno_creation() {
        let dno = CreateDno {
            slug: "test-dno".to_string(),
            name: "Test DNO".to_string(),
            // ... other fields
        };
        // Validation tests
    }
}
```

### Integration Tests
- Database model mapping tests
- Serialization/deserialization tests
- Error handling tests
- Migration compatibility tests

## Future Enhancements

### Planned Features
- GraphQL schema generation from models
- OpenAPI schema generation
- Database connection pooling optimizations
- Custom validation attributes
- Audit log automation

### Performance Optimizations
- Query result caching
- Connection pool tuning
- Index optimization
- Lazy loading for large objects

## Development Notes

### Adding New Models
1. Define the struct with appropriate derives
2. Add corresponding Create/Update DTOs
3. Implement any custom serialization logic
4. Add database migrations if needed
5. Update error handling if required

### Database Migrations
When adding new fields:
1. Create SQLx migration files
2. Update model structs
3. Add appropriate indexes
4. Update seed data if needed

### Error Handling
Use the centralized `AppError` enum:
```rust
use core::AppError;

fn database_operation() -> Result<User, AppError> {
    // Database operations
    Ok(user)
}
```

This crate forms the foundation of the entire DNO Data Gatherer application, providing consistent, type-safe data models and utilities across all services.