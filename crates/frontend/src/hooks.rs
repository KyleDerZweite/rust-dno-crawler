// Custom hooks for state management and side effects
use dioxus::prelude::*;
use shared::AppError;

// Future hook for API calls
pub fn use_api_call<T, F>(api_call: F) -> Resource<Result<T, AppError>>
where
    T: 'static,
    F: Fn() -> Result<T, AppError> + 'static,
{
    use_resource(move || api_call())
}