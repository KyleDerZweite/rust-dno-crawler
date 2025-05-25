use crate::core::models::User;
use async_trait::async_trait;
use axum_session_auth::{Authentication, HasPermission};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionUser {
    pub id: String,
    pub email: String,
    pub role: String,
}

impl From<User> for SessionUser {
    fn from(user: User) -> Self {
        SessionUser {
            id: user.id,
            email: user.email,
            role: user.role,
        }
    }
}

#[async_trait]
impl Authentication<SessionUser, String, String> for SessionUser {
    async fn load_user(userid: String, _pool: Option<&String>) -> Result<SessionUser, anyhow::Error> {
        Err(anyhow::anyhow!("User loading from session not implemented"))
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        true
    }

    fn is_anonymous(&self) -> bool {
        false
    }
}

#[async_trait]
impl HasPermission<SessionUser> for SessionUser {
    async fn has(&self, perm: &str, _user: &Option<SessionUser>) -> bool {
        match perm {
            "admin" => self.role == "admin",
            "user" => self.role == "user" || self.role == "admin",
            "guest" => true,
            _ => false,
        }
    }
}

impl SessionUser {
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }

    pub fn is_user(&self) -> bool {
        self.role == "user" || self.role == "admin"
    }

    pub fn can_access_api(&self) -> bool {
        self.role == "user" || self.role == "admin"
    }

    pub fn can_manage_users(&self) -> bool {
        self.role == "admin"
    }
}