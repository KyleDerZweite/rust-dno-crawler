use crate::core::errors::AppError;

fn sanitize_dno_name(name: &str) -> Result<String, AppError> {
    // Remove any potentially dangerous characters
    let re = regex::Regex::new(r"[^a-zA-Z0-9_-]").unwrap();
    let sanitized = re.replace_all(name.trim(), "").to_string();

    if sanitized.is_empty() {
        return Err(AppError::BadRequest("Invalid dno-name parameter".into()));
    }

    if sanitized.len() > 100 {
        return Err(AppError::BadRequest("dno-name parameter too long".into()));
    }

    Ok(sanitized)
}
