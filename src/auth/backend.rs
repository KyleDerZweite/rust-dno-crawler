use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        database::Database,
        errors::AppError,
        models::{User, LoginRequest},
    },
    auth::password::PasswordService,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub id: String,
    pub email: String,
    pub role: String,
}

impl std::fmt::Display for AuthenticatedUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.email)
    }
}

impl AuthUser for AuthenticatedUser {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.email.as_bytes()
    }
}

impl From<User> for AuthenticatedUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            role: user.role,
        }
    }
}

impl AuthenticatedUser {
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }

    pub fn can_access_api(&self) -> bool {
        self.role == "user" || self.role == "admin"
    }

    // Add these new helper methods
    pub fn can_manage_users(&self) -> bool {
        self.is_admin()
    }

    pub fn has_role(&self, required_role: &str) -> bool {
        match required_role {
            "admin" => self.is_admin(),
            "user" => self.can_access_api(),
            _ => false,
        }
    }

    pub fn can_access_dashboard(&self) -> bool {
        self.can_access_api()
    }
}

#[derive(Debug, Clone)]
pub struct AuthBackend {
    database: Database,
    password_service: PasswordService,
}

impl AuthBackend {
    pub fn new(database: Database) -> Self {
        Self {
            database,
            password_service: PasswordService::new(),
        }
    }
}

#[async_trait]
impl AuthnBackend for AuthBackend {
    type User = AuthenticatedUser;
    type Credentials = LoginRequest;
    type Error = AppError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        // Benutzer aus Datenbank laden
        let user = match self.database.get_user_by_email(&creds.email).await? {
            Some(user) => user,
            None => return Ok(None), // Benutzer existiert nicht
        };

        // Passwort verifizieren
        if !user.verify_password(&creds.password)? {
            return Ok(None); // Falsches Passwort
        }

        // Überprüfe ob Benutzer nicht als Guest eingeloggt ist (für Session-basierte Auth erlaubt)
        Ok(Some(AuthenticatedUser::from(user)))
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = self.database.get_user_by_id(user_id).await?;
        Ok(user.map(AuthenticatedUser::from))
    }
}