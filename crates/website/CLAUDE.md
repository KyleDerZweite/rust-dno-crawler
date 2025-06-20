### Development Workflow with SSR

#### Quick Start (Development)
```bash
# Install Dioxus CLI
cargo install dioxus-cli

# Run with SSR in development (with auth skip)
SKIP_AUTH=true cargo run

# Or configure in .env.development
echo "SKIP_AUTH=true" >> .env.development
cargo run

# For hot reload during development
cargo watch -x run
```

#### Project Structure for SSR
```
dno-crawler/
├── Cargo.toml
├── src/
│   ├── main.rs              # Axum server with Dioxus SSR
│   ├── app.rs              # Root Dioxus component
│   ├── routes.rs           # Route definitions
│   ├── components/
│   │   ├── mod.rs
│   │   ├── navbar.rs
│   │   ├── query_input.rs
│   │   └── data_table.rs
│   ├── pages/
│   │   ├── mod.rs
│   │   ├── home.rs
│   │   ├── dashboard.rs
│   │   └── admin.rs
│   ├── server_fns/         # Server functions
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   └── query.rs
│   ├── api/               # Shared API logic
│   │   ├── mod.rs
│   │   └── client.rs
│   └── middleware/
│       ├── mod.rs
│       └── auth.rs
├── assets/
│   ├── tailwind.css
│   └── logo.svg
├── templates/
│   └── index.html          # Base HTML template
└── public/                 # Static files
```

#### Integrated SSR App Structure
```rust
// src/main.rs
use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Router,
};
use dioxus::prelude::*;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    // Shared state between API and SSR
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let app_state = AppState::new();
    
    let app = Router::new()
        // API routes
        .route("/api/v1/auth/login", post(api::auth::login))
        .route("/api/v1/search/dno", post(api::search::search_by_dno))
        .route("/api/v1/search/year", post(api::search::search_by_year))
        
        // Static assets
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/public", ServeDir::new("public"))
        
        // Dioxus SSR for all other routes
        .fallback(dioxus_handler)
        .with_state(app_state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
        
    println!("Server running at http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}

async fn dioxus_handler(
    State(state): State<AppState>,
    request: Request<Body>,
) -> impl IntoResponse {
    // Create a new VirtualDom for each request
    let mut vdom = VirtualDom::new_with_props(
        App,
        AppProps {
            route: extract_route_from_request(&request),
            initial_state: state.clone(),
        },
    );
    
    // Render to string
    let _ = vdom.rebuild();
    let html = dioxus_ssr::render(&vdom);
    
    Html(format!(
        r#"<!DOCTYPE html>
        <html lang="de">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <link rel="stylesheet" href="/assets/tailwind.css">
            <title>DNO Crawler</title>
            <script>
                // Hydration script for interactive components
                window.__INITIAL_STATE__ = {};
            </script>
        </head>
        <body class="bg-bg-primary text-text-primary">
            <div id="app">{}</div>
            <script type="module" src="/assets/app.js"></script>
        </body>
        </html>"#,
        html
    ))
}
```

#### Auth Guard with SSR
```rust
// src/middleware/auth.rs
use dioxus::prelude::*;

#[component]
pub fn RequireAuth(children: Element) -> Element {
    let auth = use_context::<AuthState>();
    let router = use_router();
    
    // Check auth state
    match auth.user.read().as_ref() {
        Some(_) => {
            // User is authenticated
            rsx! { {children} }
        }
        None => {
            // In development with SKIP_AUTH, auto-login
            if cfg!(debug_assertions) && std::env::var("SKIP_AUTH").is_ok() {
                use_effect(move || {
                    auth.user.set(Some(User {
                        id: "dev-user".to_string(),
                        email: "dev@localhost".to_string(),
                        role: Role::Admin,
                    }
        }
    }
}

// Usage in pages
#[component]
fn DashboardPage() -> Element {
    rsx! {
        RequireAuth {
            div { class: "dashboard-container",
                NavBar {}
                // Dashboard content
            }
        }
    }
}
```

#### Server Functions for Data Fetching
```rust
// src/server_fns/query.rs
use dioxus::prelude::*;

#[server(ProcessQuery)]
pub async fn process_query(query: String) -> Result<QueryResult, ServerFnError> {
    // This runs on the server with full database access
    let pool = use_context::<PgPool>();
    
    // Skip complex auth in dev mode
    if cfg!(debug_assertions) && std::env::var("SKIP_AUTH").is_ok() {
        // Return mock data for development
        return Ok(QueryResult {
            interpretation: Interpretation {
                dno: "Netze BW".to_string(),
                year: 2024,
                data_type: DataType::Netzentgelte,
                confidence: 0.95,
            },
            data: serde_json::json!({
                "hs": {"leistung": 58.21, "arbeit": 1.26}
            }),
            source: Source {
                file: "mock-data.pdf".to_string(),
                page: 1,
            },
        });
    }
    
    // Production query processing
    let result = sqlx::query_as!(
        QueryResult,
        "SELECT * FROM process_natural_query($1)",
        query
    )
    .fetch_one(&pool)
    .await?;
    
    Ok(result)
}

#[server(GetDashboardStats)]
pub async fn get_dashboard_stats() -> Result<DashboardStats, ServerFnError> {
    // Server-side data fetching
    let pool = use_context::<PgPool>();
    
    let stats = sqlx::query_as!(
        DashboardStats,
        r#"
        SELECT 
            COUNT(DISTINCT q.id) as queries_today,
            COUNT(DISTINCT d.id) as total_dnos,
            AVG(q.response_time_ms) as avg_response_time
        FROM query_logs q
        CROSS JOIN dnos d
        WHERE q.created_at > CURRENT_DATE
        "#
    )
    .fetch_one(&pool)
    .await?;
    
    Ok(stats)
}
```

#### Dashboard Page with SSR Data
```rust
// src/pages/dashboard.rs
use dioxus::prelude::*;

#[component]
fn Dashboard() -> Element {
    // Server functions automatically work with SSR
    let stats = use_resource(|| async { get_dashboard_stats().await });
    let mut query_input = use_signal(|| String::new());
    let mut query_results = use_signal(|| None);
    
    rsx! {
        div { class: "min-h-screen bg-bg-primary",
            NavBar {}
            
            div { class: "container mx-auto px-4 py-8",
                // Welcome section
                div { class: "mb-8",
                    h1 { class: "text-3xl font-bold text-text-primary",
                        "Guten Tag!"
                    }
                    p { class: "text-text-secondary mt-2",
                        "Was möchten Sie über DNO-Daten wissen?"
                    }
                }
                
                // Stats cards
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8",
                    match &*stats.read() {
                        Some(Ok(stats)) => rsx! {
                            MetricCard {
                                title: "Queries heute",
                                value: stats.queries_today as f64,
                                icon: "chart-bar"
                            }
                            MetricCard {
                                title: "Aktive DNOs",
                                value: stats.total_dnos as f64,
                                change: Some(2.5),
                                icon: "building-office"
                            }
                            MetricCard {
                                title: "Ø Response Time",
                                value: stats.avg_response_time,
                                icon: "clock"
                            }
                            MetricCard {
                                title: "Cache Hit Rate",
                                value: 82.5,
                                change: Some(5.2),
                                icon: "lightning-bolt"
                            }
                        },
                        Some(Err(_)) => rsx! { ErrorMessage { message: "Fehler beim Laden der Statistiken" } },
                        None => rsx! { LoadingCards {} }
                    }
                }
                
                // Query section
                QueryInput {}
                
                // Results display
                if let Some(results) = query_results.read().as_ref() {
                    QueryResultsDisplay { results: results.clone() }
                }
            }
        }
    }
}
```

#### Build and Deployment

##### Development Build
```bash
# Development with hot reload
cargo watch -x run

# Or with environment override
SKIP_AUTH=true cargo run --features dev
```

##### Production Build
```bash
# Build for production
cargo build --release

# Docker build
docker build -t dno-crawler:latest .
```

##### Dockerfile for SSR
```dockerfile
# Build stage
FROM rust:1.75 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY assets ./assets
COPY templates ./templates
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/dno-crawler .
COPY --from=builder /app/assets ./assets
COPY --from=builder /app/templates ./templates
COPY public ./public

ENV RUST_LOG=info
EXPOSE 8080

CMD ["./dno-crawler"]
```

##### Nginx Configuration for Production
```nginx
server {
    listen 80;
    server_name dno-crawler.kylehub.dev;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name dno-crawler.kylehub.dev;
    
    ssl_certificate /etc/letsencrypt/live/dno-crawler.kylehub.dev/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/dno-crawler.kylehub.dev/privkey.pem;
    
    # Proxy all requests to the Dioxus SSR app
    location / {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }
    
    # Cache static assets
    location ~ ^/(assets|public)/ {
        proxy_pass http://localhost:8080;
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

### Performance Optimizations for SSR
- Static asset caching with long expiry
- Streaming SSR for faster time to first byte
- Component-level caching for expensive queries
- Database connection pooling
- Redis caching for frequently accessed data
- CDN integration for assets
- Brotli compression for HTML responses

### SEO Benefits of SSR
- Full HTML rendered on first request
- Meta tags properly set before serving
- Faster initial page load
- Better crawlability for search engines
- Open Graph tags work properly
- No JavaScript required for initial render));
  });
  rsx! { {children} }
  } else {
  // Redirect to auth page
  use_effect(move || {
  router.push(Route::Auth {});
  });
  rsx! {
  div { class: "flex items-center justify-center h-screen",
  Spinner { size: "lg" }
  }
  }
  }# DNO Crawler Website Development Specification

## Design System

### Color Palette
```css
/* Primary Colors - Dark Theme */
--color-bg-primary: #0a0a0a;        /* Main background - Almost black */
--color-bg-secondary: #121212;      /* Card backgrounds */
--color-bg-tertiary: #1a1a1a;       /* Elevated surfaces */
--color-bg-hover: #232323;          /* Hover states */

/* Text Colors */
--color-text-primary: #e5e5e5;      /* Primary text */
--color-text-secondary: #a3a3a3;    /* Secondary text */
--color-text-muted: #737373;        /* Muted text */

/* Accent Colors - Vibrant Green */
--color-accent-primary: #22c55e;    /* Primary green */
--color-accent-hover: #16a34a;      /* Darker green on hover */
--color-accent-light: #4ade80;      /* Light green for highlights */
--color-accent-dark: #15803d;       /* Dark green for active states */
--color-accent-glow: rgba(34, 197, 94, 0.2); /* Green glow effect */

/* Secondary Colors - Amber/Orange */
--color-secondary: #f59e0b;         /* Amber */
--color-secondary-hover: #d97706;   /* Darker amber */
--color-secondary-light: #fbbf24;   /* Light amber */
--color-warning: #ef4444;           /* Red for errors */
--color-info: #3b82f6;             /* Blue for info */

/* Borders & Dividers */
--color-border: #262626;
--color-border-hover: #404040;
```

### Typography
```css
/* Font Stack */
--font-sans: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
--font-mono: 'JetBrains Mono', 'Fira Code', monospace;

/* Font Sizes */
--text-xs: 0.75rem;
--text-sm: 0.875rem;
--text-base: 1rem;
--text-lg: 1.125rem;
--text-xl: 1.25rem;
--text-2xl: 1.5rem;
--text-3xl: 1.875rem;
--text-4xl: 2.25rem;
```

### Component Styles
```css
/* Buttons */
.btn-primary {
  background: linear-gradient(135deg, var(--color-accent-primary), var(--color-accent-dark));
  box-shadow: 0 0 20px var(--color-accent-glow);
  transition: all 0.3s ease;
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 0 30px var(--color-accent-glow);
}

/* Cards */
.card {
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: 12px;
  backdrop-filter: blur(10px);
}

/* Inputs */
.input {
  background: var(--color-bg-tertiary);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  transition: all 0.2s ease;
}

.input:focus {
  border-color: var(--color-accent-primary);
  box-shadow: 0 0 0 3px var(--color-accent-glow);
}
```

## Pages Structure

### 1. Landing Page (/)
**Purpose**: Marketing page to attract users and explain the service

**Sections**:
- **Hero Section**
    - Dark gradient background with subtle grid pattern
    - Main headline: "Automatisierte DNO-Datenerfassung"
    - Subheadline: "Netzentgelte und HLZF-Daten von 850+ Netzbetreibern"
    - CTA buttons: "Kostenlos starten" (green) and "Demo ansehen" (outlined)
    - Animated data visualization showing crawling process

- **Features Grid** (3x2)
    - Natural Language Queries (with chat icon)
    - Real-time Data Updates (with refresh icon)
    - API Access (with code icon)
    - Data Verification (with check icon)
    - Automated Crawling (with robot icon)
    - Export Options (with download icon)

- **Live Demo Section**
    - Interactive query input: "Probieren Sie es aus!"
    - Example: "Zeige mir die Netzentgelte von Netze BW"
    - Shows real response with syntax highlighting

- **Statistics Section**
    - Counter animations:
        - 850+ DNOs
        - 15,000+ Data Points
        - 99.9% Uptime
        - 24/7 Automated Updates

- **Pricing Section**
    - Free tier / Pro tier / Enterprise
    - Green accent for recommended plan

- **Footer**
    - Links to documentation, API, privacy, terms
    - Dark background with subtle dividers

### 2. Login/Register Page (/auth)
**Layout**: Split screen design
- Left side: Login/Register form
- Right side: Feature highlights with animated graphics

**Login Form**:
```
┌─────────────────────────────────┐
│  Logo                           │
│                                 │
│  Willkommen zurück             │
│  Melden Sie sich an            │
│                                 │
│  Email                         │
│  [________________]            │
│                                 │
│  Passwort                      │
│  [________________]            │
│                                 │
│  □ Angemeldet bleiben          │
│                                 │
│  [Anmelden] (green button)     │
│                                 │
│  Noch kein Konto? Registrieren │
└─────────────────────────────────┘
```

**Register Form**:
- Name, Email, Password, Password confirmation
- Terms acceptance checkbox
- Green primary button
- OAuth options (optional)

### 3. Dashboard (/dashboard)
**Layout**: Sidebar navigation + Main content area

**Sidebar** (collapsible):
```
┌─────────────────────┐
│ DNO Crawler         │
│                     │
│ 🏠 Dashboard        │
│ 💬 Query Console    │
│ 📊 Analytics        │
│ 🕐 History          │
│ ⚙️ Settings         │
│ 👤 Account          │
│                     │
│ ─────────────────   │
│                     │
│ 📚 Documentation    │
│ 🔧 API Keys         │
│ 💳 Billing          │
└─────────────────────┘
```

**Main Dashboard Content**:
- **Welcome Header**: "Guten Tag, [Name]"
- **Quick Stats Cards** (4 cards in grid):
    - Queries Today (with trend arrow)
    - Active DNOs (with percentage)
    - Cache Hit Rate (with circular progress)
    - API Usage (with bar chart)

- **Natural Language Query Section**:
  ```
  ┌─────────────────────────────────────────────┐
  │ Was möchten Sie wissen?                     │
  │ ┌─────────────────────────────────────────┐ │
  │ │ Eingabe...                              │ │
  │ └─────────────────────────────────────────┘ │
  │ [Beispiele ▼] [Verlauf] [Suchen →]         │
  └─────────────────────────────────────────────┘
  ```

- **Results Display Area**:
    - Toggle between JSON/Table/Chart view
    - Copy button for data
    - Download options
    - Source verification badge

- **Recent Activity Feed**:
    - Last 5 queries with timestamps
    - Quick re-run buttons
    - Status indicators (cached/fresh)

### 4. Query Console (/dashboard/query)
**Purpose**: Advanced query interface with history

**Layout**:
- **Split View**:
    - Left: Query input & options
    - Right: Results & visualization

**Features**:
- Syntax highlighting for natural language
- Query templates dropdown
- Save/Load queries
- Export results (JSON, CSV, Excel)
- Full-screen mode
- Query performance metrics

### 5. Settings/Account Page (/dashboard/settings)
**Tabs Layout**:
1. **Profile Tab**:
    - Avatar upload (drag & drop)
    - Name, Email fields
    - Language preference
    - Timezone selection

2. **Security Tab**:
    - Change password form
    - Two-factor authentication
    - Active sessions list
    - Security log

3. **API Keys Tab**:
   ```
   ┌─────────────────────────────────────────┐
   │ API Keys                                │
   │                                         │
   │ [+ Create New Key] (green button)      │
   │                                         │
   │ ┌─────────────────────────────────────┐ │
   │ │ Production Key                      │ │
   │ │ dnk_live_...xyz                     │ │
   │ │ Created: 2024-01-15                 │ │
   │ │ Last used: 2 hours ago              │ │
   │ │ [Copy] [Regenerate] [Delete]        │ │
   │ └─────────────────────────────────────┘ │
   └─────────────────────────────────────────┘
   ```

4. **Notifications Tab**:
    - Email preferences
    - Webhook configuration
    - Alert thresholds

5. **Danger Zone Tab**:
    - Export all data
    - Delete account (red button)

### 6. Admin Dashboard (/admin)
**Access**: Admin role only

**Layout**: Enhanced sidebar + Main area

**Admin Sidebar** (additional items):
```
│ 👥 User Management  │
│ 📊 System Metrics   │
│ 🔍 Data Verification│
│ 🤖 Crawl Control    │
│ 📝 System Logs      │
│ ⚡ Cache Management │
```

**Main Sections**:

1. **System Overview**:
    - Real-time metrics dashboard
    - System health indicators
    - Active jobs queue visualization
    - Resource usage graphs

2. **User Management** (/admin/users):
   ```
   ┌─────────────────────────────────────────┐
   │ Users (1,523 total)                     │
   │                                         │
   │ [Search...] [Filter ▼] [Export]         │
   │                                         │
   │ ┌─────────────────────────────────────┐ │
   │ │ Name     Email      Role    Status  │ │
   │ │ John D.  john@...   User    Active  │ │
   │ │ [View] [Edit] [Suspend]             │ │
   │ └─────────────────────────────────────┘ │
   └─────────────────────────────────────────┘
   ```

3. **Data Verification** (/admin/verification):
    - Split screen: PDF viewer + Data editor
    - Verification queue
    - Bulk actions toolbar
    - Confidence scores
    - History timeline

4. **Metrics Dashboard** (/admin/metrics):
    - Grafana-style graphs:
        - Query volume (line chart)
        - Response times (histogram)
        - Cache performance (gauge)
        - DNO coverage heatmap
        - Error rates (area chart)
    - Export to Prometheus format

5. **Crawl Control** (/admin/crawl):
    - Active crawlers status
    - Queue management
    - Manual trigger interface
    - Schedule configuration
    - Error logs with retry options

## Component Library

### 1. Navigation Bar
```jsx
<nav class="navbar">
  <div class="navbar-brand">
    <img src="/logo.svg" alt="DNO Crawler" />
  </div>
  <div class="navbar-menu">
    <a href="/dashboard">Dashboard</a>
    <a href="/docs">API Docs</a>
  </div>
  <div class="navbar-end">
    <div class="user-menu">
      <img src="/avatar.jpg" class="avatar" />
      <span>John Doe</span>
      <ChevronDown />
    </div>
  </div>
</nav>
```

### 2. Data Table Component
```jsx
<DataTable
  columns={[
    { key: 'dno', label: 'DNO', sortable: true },
    { key: 'year', label: 'Jahr', sortable: true },
    { key: 'status', label: 'Status', badge: true },
    { key: 'actions', label: '', actions: true }
  ]}
  data={entries}
  onSort={handleSort}
  onFilter={handleFilter}
  bulkActions={['verify', 'export', 'delete']}
/>
```

### 3. Query Input Component
```jsx
<QueryInput
  placeholder="Fragen Sie nach DNO-Daten..."
  suggestions={true}
  history={true}
  voice={true}
  onSubmit={handleQuery}
/>
```

### 4. Metric Card Component
```jsx
<MetricCard
  title="Queries Today"
  value={156}
  change={+12.5}
  icon={<Activity />}
  color="green"
  sparkline={[10, 15, 13, 20, 18, 25]}
/>
```

### 5. Status Badge Component
```jsx
<StatusBadge 
  status="verified"
  size="sm"
  dot={true}
/>
```

## Animations & Interactions

### Micro-animations
- Button hover: Scale up + glow effect
- Card hover: Slight elevation + border glow
- Page transitions: Smooth fade-in
- Loading states: Skeleton screens with shimmer
- Success states: Green checkmark with particle effect
- Data updates: Number counter animations

### Interactive Elements
- Drag-and-drop file uploads
- Tooltip on hover for help text
- Command palette (Cmd+K) for quick actions
- Keyboard shortcuts for power users
- Real-time search with debouncing
- Infinite scroll for data tables

## Responsive Design

### Breakpoints
```css
/* Mobile: < 640px */
/* Tablet: 640px - 1024px */
/* Desktop: 1024px - 1280px */
/* Wide: > 1280px */
```

### Mobile Adaptations
- Hamburger menu for navigation
- Stack cards vertically
- Full-width buttons
- Swipeable tabs
- Bottom sheet for filters
- Simplified data tables (key info only)

## Technical Implementation

### Frontend Stack with SSR
```json
{
  "framework": "Dioxus with SSR (Server-Side Rendering)",
  "rendering": "Full-stack with Axum integration",
  "styling": "TailwindCSS + Custom CSS",
  "components": "Custom Dioxus components",
  "icons": "Heroicons / Lucide Icons (SVG)",
  "charts": "Charming (server-compatible) / Custom SVG",
  "state": "Dioxus hooks (use_signal, use_context)",
  "api": "Server functions with Dioxus",
  "websocket": "axum-tungstenite",
  "animations": "CSS transitions + View Transitions API"
}
```

### Key Libraries (Cargo.toml)
```toml
[dependencies]
# Dioxus with SSR
dioxus = { version = "0.6", features = ["fullstack", "router"] }
dioxus-ssr = "0.6"

# Web server
axum = { version = "0.7", features = ["macros"] }
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["fs", "cors", "compression"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Database & Auth (shared with API)
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls"] }
jsonwebtoken = "9"

# Utils
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["serde", "v4"] }

# Charts (server-compatible)
charming = "0.3"

# Development
dotenv = "0.15"
tracing = "0.1"
tracing-subscriber = "0.3"
```

### Full-Stack Architecture with SSR

#### Main Application Entry (src/main.rs)
```rust
use dioxus::prelude::*;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // Load environment
    dotenvy::dotenv().ok();
    
    let app = Router::new()
        // Serve the Dioxus app with SSR
        .fallback(get(serve_dioxus_app));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
        
    axum::serve(listener, app).await.unwrap();
}

async fn serve_dioxus_app() -> impl IntoResponse {
    let mut vdom = VirtualDom::new(App);
    vdom.rebuild_in_place();
    
    let html = dioxus_ssr::render(&vdom);
    
    Html(format!(
        r#"<!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <link rel="stylesheet" href="/assets/tailwind.css">
            <title>DNO Crawler</title>
        </head>
        <body>
            <div id="app">{}</div>
            <script src="/assets/app.js"></script>
        </body>
        </html>"#,
        html
    ))
}
```

#### Development vs Production Auth

##### Development Mode (Skip Auth)
```rust
// src/auth.rs
use dioxus::prelude::*;

#[derive(Clone, Debug)]
pub struct AuthState {
    pub user: Signal<Option<User>>,
    pub loading: Signal<bool>,
}

impl AuthState {
    pub fn new() -> Self {
        // Skip auth in development
        let user = if cfg!(debug_assertions) {
            Signal::new(Some(User {
                id: "dev-user".to_string(),
                email: "dev@localhost".to_string(),
                role: Role::Admin,
            }))
        } else {
            Signal::new(None)
        };
        
        Self {
            user,
            loading: Signal::new(false),
        }
    }
}

// Server function for production auth
#[server(LoginFn, "/api/auth/login")]
pub async fn login(email: String, password: String) -> Result<User, ServerFnError> {
    // In development, return mock user
    if cfg!(debug_assertions) && std::env::var("SKIP_AUTH").is_ok() {
        return Ok(User {
            id: "dev-user".to_string(),
            email: email,
            role: Role::Admin,
        });
    }
    
    // Production auth logic here
    // ... validate against database
}
```

### Development Workflow

#### Quick Start (Development)
```bash
# Install Dioxus CLI
cargo install dioxus-cli

# Run with hot reload and skip auth
SKIP_AUTH=true dx serve --hot-reload

# Or configure in .env.development
echo "SKIP_AUTH=true" >> .env.development
dx serve
```

#### Project Structure
```
frontend/
├── Cargo.toml
├── Dioxus.toml
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── components/
│   │   ├── mod.rs
│   │   ├── navbar.rs
│   │   ├── query_input.rs
│   │   └── data_table.rs
│   ├── pages/
│   │   ├── mod.rs
│   │   ├── home.rs
│   │   ├── dashboard.rs
│   │   └── admin.rs
│   ├── api/
│   │   ├── mod.rs
│   │   ├── client.rs
│   │   └── auth.rs
│   └── utils/
│       ├── mod.rs
│       └── auth_guard.rs
├── assets/
│   ├── tailwind.css
│   └── logo.svg
└── dist/
```

#### Auth Skip in Development
```rust
// src/utils/auth_guard.rs
use dioxus::prelude::*;

#[cfg(debug_assertions)]
pub fn use_auth(cx: &ScopeState) -> AuthState {
    // Return mock auth state in development
    AuthState {
        user: Some(User {
            id: "dev-123".to_string(),
            email: "dev@localhost".to_string(),
            name: "Dev User".to_string(),
            role: Role::Admin,
        }),
        token: Some("dev-token".to_string()),
        loading: false,
    }
}

#[cfg(not(debug_assertions))]
pub fn use_auth(cx: &ScopeState) -> AuthState {
    // Real auth implementation for production
    let auth = use_context::<AuthState>(cx).unwrap();
    auth.clone()
}

// Usage in components
#[component]
fn ProtectedPage(cx: Scope) -> Element {
    let auth = use_auth(cx);
    
    if auth.user.is_none() {
        return cx.render(rsx! {
            Redirect { to: "/auth" }
        });
    }
    
    // Render protected content
    cx.render(rsx! {
        div { "Protected content here" }
    })
}
```

#### API Client with Dev Mode
```rust
// src/api/client.rs
impl ApiClient {
    pub fn new() -> Self {
        let base_url = if cfg!(debug_assertions) {
            std::env::var("DEV_API_URL")
                .unwrap_or_else(|_| "http://localhost:3000/api/v1".to_string())
        } else {
            "https://dno-crawler.kylehub.dev/api/v1".to_string()
        };
        
        Self {
            client: reqwest::Client::new(),
            base_url,
            auth_token: None,
        }
    }
    
    #[cfg(debug_assertions)]
    pub async fn mock_login(&mut self) -> Result<AuthResponse> {
        // Return mock auth response in dev mode
        Ok(AuthResponse {
            user: User {
                id: "dev-123".to_string(),
                email: "dev@localhost".to_string(),
                name: "Dev User".to_string(),
                role: Role::Admin,
            },
            tokens: Tokens {
                access_token: "dev-access-token".to_string(),
                refresh_token: "dev-refresh-token".to_string(),
                expires_in: 3600,
            },
        })
    }
}
```

#### Build Commands
```bash
# Development build with hot reload
dx build --features dev

# Production build (optimized)
dx build --release

# Serve production build locally
dx serve --release --port 8080
```

#### Integration with Axum Backend
```rust
// Backend: Skip auth middleware in dev mode
// src/middleware/auth.rs (Axum)
pub fn auth_middleware() -> impl Middleware {
    if cfg!(debug_assertions) && env::var("SKIP_AUTH").is_ok() {
        // Return no-op middleware
        Box::new(|req: Request, next: Next| async {
            next.run(req).await
        })
    } else {
        // Return real auth middleware
        Box::new(require_auth)
    }
}

### API Integration with Axum Backend
```rust
// src/api/client.rs
use reqwest::{Client, RequestBuilder};

pub struct ApiClient {
    client: Client,
    base_url: String,
    auth_token: Option<String>,
}

impl ApiClient {
    pub async fn login(&mut self, email: &str, password: &str) -> Result<AuthResponse> {
        let response = self.client
            .post(&format!("{}/auth/login", self.base_url))
            .json(&LoginRequest { email, password })
            .send()
            .await?;
        
        let auth_data = response.json::<AuthResponse>().await?;
        self.auth_token = Some(auth_data.access_token.clone());
        Ok(auth_data)
    }
    
    // Interceptor for authenticated requests
    fn authenticated(&self, request: RequestBuilder) -> RequestBuilder {
        match &self.auth_token {
            Some(token) => request.bearer_auth(token),
            None => request,
        }
    }
}
```

### Performance Optimizations
- Lazy components with `dioxus::prelude::lazy`
- Virtual DOM diffing (built into Dioxus)
- WASM bundle optimization with `wasm-opt`
- Route-based code splitting
- Compressed assets with Brotli
- CDN for static assets

### Build Configuration

#### Dioxus.toml
```toml
[application]
name = "dno-crawler"
default_platform = "web"

[web.app]
title = "DNO Crawler"
base_path = "/"

[web.watcher]
watch_path = ["src", "assets"]
reload_html = true

[web.resource]
style = ["assets/tailwind.css"]
script = []

[web.resource.dev]
style = ["assets/tailwind.css", "assets/dev.css"]
script = ["assets/dev.js"]
```

#### Development Features
- Hot reload with `dx serve`
- Mock data for offline development
- Debug overlay with performance metrics
- Component inspector
- State debugging tools

## Accessibility

- ARIA labels on all interactive elements
- Keyboard navigation support
- Focus indicators (green outline)
- Screen reader announcements
- Color contrast ratio > 4.5:1
- Reduced motion option
- Alt text for all images
- Semantic HTML structure

## SEO & Meta

```html
<meta name="description" content="Automatisierte Erfassung von Netzentgelten und HLZF-Daten deutscher Netzbetreiber">
<meta property="og:title" content="DNO Crawler - Netzbetreiber Daten API">
<meta property="og:image" content="/og-image.png">
<link rel="canonical" href="https://dno-crawler.kylehub.dev">
```

## Error States

### 404 Page
- Animated illustration
- "Oops! Seite nicht gefunden"
- Suggestions for common pages
- Search bar
- Return home button (green)

### Error Boundaries
- Graceful error handling
- User-friendly messages
- Retry mechanisms
- Error reporting option
- Fallback UI components

## Loading States

### Skeleton Screens
```html
<div class="skeleton-card">
  <div class="skeleton-line w-3/4"></div>
  <div class="skeleton-line w-1/2"></div>
  <div class="skeleton-line w-full"></div>
</div>
```

### Progress Indicators
- Linear progress for determinate operations
- Circular spinners for indeterminate
- Step indicators for multi-stage processes
- Real-time percentage for file uploads

## Security Considerations

- Content Security Policy headers
- HTTPS enforcement
- XSS protection
- CSRF tokens
- Rate limiting UI feedback
- Secure cookie handling
- Input sanitization
- File upload restrictions