use crate::{
    core::{database::Database, errors::AppError, models::{CreateUserRequest, LoginRequest}},
    website::{
        layout::Layout,
        routes::{
            dashboard::{Dashboard, DashboardProps},
            error_404::Error404,
            login::Login,
            register::Register,
            user_management::{UserManagement, UserManagementProps},
        },
    },
};
use axum::{
    extract::State,
    response::{Html, Redirect},
    Form,
};
use axum_login::AuthSession;
use dioxus::prelude::*;
use serde::Deserialize;
use crate::website::routes::error_404::Error404Props;

type AuthContext = AuthSession<crate::auth::backend::AuthBackend>;

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

// Helper function to render with layout
fn render_with_layout(
    content: Element,
    user_role: Option<String>,
    is_authenticated: bool
) -> String {
    let mut app = VirtualDom::new_with_props(Layout, crate::website::layout::LayoutProps {
        user_role,
        is_authenticated,
        children: content,
    });

    // THIS IS THE KEY: rebuild before rendering
    let _ = app.rebuild(&mut dioxus_core::NoOpMutations);

    let html = dioxus_ssr::render(&mut app);
    format!("<!DOCTYPE html>{}", html)
}

pub async fn register_page() -> Html<String> {
    let content = rsx! { Register {} };
    let html = render_with_layout(content, None, false);
    Html(html)
}

pub async fn login_page() -> Html<String> {
    let content = rsx! { Login {} };
    let html = render_with_layout(content, None, false);
    Html(html)
}

pub async fn user_management_page(auth: AuthContext) -> Result<Html<String>, Redirect> {
    if let Some(user) = auth.user {
        let props = UserManagementProps {
            current_user_role: user.role.clone(),
        };

        let content = rsx! {
            UserManagement {
                current_user_role: props.current_user_role
            }
        };
        let html = render_with_layout(content, Some(user.role), true);
        Ok(Html(html))
    } else {
        Err(Redirect::to("/login"))
    }
}

pub async fn privacy_page(auth: AuthContext) -> Html<String> {
    let (user_role, is_authenticated) = if let Some(user) = auth.user {
        (Some(user.role), true)
    } else {
        (None, false)
    };

    let content = rsx! { crate::website::routes::privacy::Privacy {} };
    let html = render_with_layout(content, user_role, is_authenticated);
    Html(html)
}

pub async fn terms_page(auth: AuthContext) -> Html<String> {
    let (user_role, is_authenticated) = if let Some(user) = auth.user {
        (Some(user.role), true)
    } else {
        (None, false)
    };

    let content = rsx! { crate::website::routes::terms::Terms {} };
    let html = render_with_layout(content, user_role, is_authenticated);
    Html(html)
}

pub async fn contact_page(auth: AuthContext) -> Html<String> {
    let (user_role, is_authenticated) = if let Some(user) = auth.user {
        (Some(user.role), true)
    } else {
        (None, false)
    };

    let content = rsx! { crate::website::routes::contact::Contact {} };
    let html = render_with_layout(content, user_role, is_authenticated);
    Html(html)
}

pub async fn error_404_page() -> Html<String> {
    let route = vec!["unknown".to_string()];
    let content = rsx! {
        Error404 {
            route: route
        }
    };
    let html = render_with_layout(content, None, false);
    Html(html)
}

pub async fn dashboard(auth: AuthContext) -> Result<Html<String>, Redirect> {
    if let Some(user) = auth.user {
        let props = DashboardProps {
            email: user.email.clone(),
            role: user.role.clone(),
        };

        let content = rsx! {
            Dashboard {
                email: props.email,
                role: props.role.clone()
            }
        };
        let html = render_with_layout(content, Some(user.role), true);
        Ok(Html(html))
    } else {
        Err(Redirect::to("/login"))
    }
}

// Form handlers remain the same
pub async fn register(
    mut auth: AuthContext,
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
        email: form.email.clone(),
        password: form.password.clone(),
        role: None,
    };

    let _user = state.database.create_user(request).await?;

    let login_creds = LoginRequest {
        email: form.email,
        password: form.password,
    };

    if let Ok(Some(user)) = auth.authenticate(login_creds).await {
        auth.login(&user).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
        state.database.update_last_login(&user.id).await?;
    }

    Ok(Redirect::to("/dashboard"))
}

pub async fn login(
    mut auth: AuthContext,
    State(state): State<WebState>,
    Form(form): Form<LoginForm>,
) -> Result<Redirect, AppError> {
    let creds = LoginRequest {
        email: form.email,
        password: form.password,
    };

    let user = match auth.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(AppError::Unauthorized("Invalid credentials".to_string())),
        Err(e) => return Err(AppError::InternalServerError(e.to_string())),
    };

    auth.login(&user).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
    state.database.update_last_login(&user.id).await?;

    Ok(Redirect::to("/dashboard"))
}

pub async fn logout(mut auth: AuthContext) -> Redirect {
    let _ = auth.logout().await;
    Redirect::to("/login")
}