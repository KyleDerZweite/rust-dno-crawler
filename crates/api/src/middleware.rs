use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use tracing::info;
use std::time::Instant;

pub async fn logging(request: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    info!(
        method = %method,
        uri = %uri,
        status = %status,
        duration_ms = duration.as_millis(),
        "HTTP request processed"
    );
    
    response
}