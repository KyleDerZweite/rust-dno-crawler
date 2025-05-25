use crate::{
    auth::session::SessionUser,
    core::{database::Database, errors::AppError, models::{CreateUserRequest, LoginRequest}},
    website::routes::{
        dashboard::{Dashboard, DashboardProps},
        error_404::Error404,
        login::Login,
        register::Register,
        user_management::{UserManagement, UserManagementProps},
    },
};
use axum::{
    extract::State,
    response::{Html, Redirect},
    Form,
};
use axum_session_auth::AuthSession;
use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Clone)]
pub struct WebState {
    pub database: Database,
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

pub async fn create_website_routes(database: Database, session_secret: &str) -> Result<axum::Router, AppError> {
    use axum::routing::{get, post};
    use axum_session::{SessionConfig, SessionLayer, SessionStore};
    use axum_session_auth::{AuthConfig, AuthSessionLayer};

    // Session Store Setup
    let session_config = SessionConfig::default()
        .with_table_name("sessions")
        .with_key(session_secret);

    let session_store = SessionStore::<SessionUser>::new(None, session_config)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Session store creation failed: {}", e)))?;

    let web_state = WebState { database };

    let router = axum::Router::new()
        .route("/", get(|| async { axum::response::Redirect::to("/login") }))
        .route("/register", get(register_page))
        .route("/register", post(register))
        .route("/login", get(login_page))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/dashboard", get(dashboard))
        .route("/user-management", get(user_management_page))
        .route("/privacy", get(privacy_page))
        .route("/terms", get(terms_page))
        .route("/contact", get(contact_page))
        .fallback(error_404_page)
        .with_state(web_state)
        .layer(AuthSessionLayer::<SessionUser, String>::new(Some(
            session_store.clone(),
        )))
        .layer(SessionLayer::new(session_store))
        .layer(AuthConfig::default().layer());

    Ok(router)
}

// Page handlers that render Dioxus components to HTML
pub async fn register_page() -> Html<String> {
    let mut app = VirtualDom::new(Register);
    let html = dioxus_ssr::render(&mut app);
    Html(format!("<!DOCTYPE html>{}", html))
}

pub async fn login_page() -> Html<String> {
    let mut app = VirtualDom::new(Login);
    let html = dioxus_ssr::render(&mut app);
    Html(format!("<!DOCTYPE html>{}", html))
}

pub async fn user_management_page(auth_session: AuthSession<SessionUser>) -> Result<Html<String>, Redirect> {
    if let Some(user) = auth_session.user {
        let props = UserManagementProps {
            current_user_role: user.role.clone(),
        };

        let mut app = VirtualDom::new_with_props(UserManagement, props);
        let html = dioxus_ssr::render(&mut app);
        Ok(Html(format!("<!DOCTYPE html>{}", html)))
    } else {
        Err(Redirect::to("/login"))
    }
}

pub async fn privacy_page() -> Html<String> {
    let mut app = VirtualDom::new(crate::website::routes::privacy::Privacy);
    let html = dioxus_ssr::render(&mut app);
    Html(format!("<!DOCTYPE html>{}", html))
}

pub async fn terms_page() -> Html<String> {
    let mut app = VirtualDom::new(crate::website::routes::terms::Terms);
    let html = dioxus_ssr::render(&mut app);
    Html(format!("<!DOCTYPE html>{}", html))
}

pub async fn contact_page() -> Html<String> {
    let mut app = VirtualDom::new(crate::website::routes::contact::Contact);
    let html = dioxus_ssr::render(&mut app);
    Html(format!("<!DOCTYPE html>{}", html))
}

pub async fn error_404_page() -> Html<String> {
    let route = vec!["unknown".to_string()];
    let mut app = VirtualDom::new_with_props(Error404, route);
    let html = dioxus_ssr::render(&mut app);
    Html(format!("<!DOCTYPE html>{}", html))
}

// Form handlers
pub async fn register(
    mut auth_session: AuthSession<SessionUser>,
    State(state): State<WebState>,
    Form(form): Form<RegisterForm>,
) -> Result<Redirect, AppError> {
    if state
        .database
        .get_user_by_email(&form.email)
        .await?
        .is_some()
    {
        return Err(AppError::BadRequest("User already exists".to_string()));
    }

    let request = CreateUserRequest {
        email: form.email,
        password: form.password,
        role: None,
    };

    let user = state.database.create_user(request).await?;
    let session_user = SessionUser::from(user);
    auth_session
        .login(&session_user)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    Ok(Redirect::to("/dashboard"))
}

pub async fn login(
    mut auth_session: AuthSession<SessionUser>,
    State(state): State<WebState>,
    Form(form): Form<LoginForm>,
) -> Result<Redirect, AppError> {
    let user = state
        .database
        .get_user_by_email(&form.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    if !user.verify_password(&form.password)
        .map_err(|e| AppError::InternalServerError(e.to_string()))?
    {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    let session_user = SessionUser::from(user);
    auth_session
        .login(&session_user)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    Ok(Redirect::to("/dashboard"))
}

pub async fn logout(mut auth_session: AuthSession<SessionUser>) -> Redirect {
    auth_session.logout().await;
    Redirect::to("/login")
}

pub async fn dashboard(auth_session: AuthSession<SessionUser>) -> Result<Html<String>, Redirect> {
    if let Some(user) = auth_session.user {
        let props = DashboardProps {
            email: user.email.clone(),
            role: user.role.clone(),
        };

        let mut app = VirtualDom::new_with_props(Dashboard, props);
        let html = dioxus_ssr::render(&mut app);
        Ok(Html(format!("<!DOCTYPE html>{}", html)))
    } else {
        Err(Redirect::to("/login"))
    }
}