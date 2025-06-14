// Custom hooks for state management and side effects
use dioxus::prelude::*;
use shared::AppError;
use std::future::Future;

// Future hook for API calls
pub fn use_api_call<T, F, Fut>(api_call: F) -> Resource<Result<T, AppError>>
where
    T: 'static + Clone + Send + Sync,
    F: Fn() -> Fut + 'static + Send + Sync,
    Fut: Future<Output = Result<T, AppError>> + 'static + Send,
{
    use_resource(move || api_call())
}