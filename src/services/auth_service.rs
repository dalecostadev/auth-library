// src/services/auth_service.rs

use crate::models::auth_model::Auth;
use mongodb::{Client, options::ClientOptions};
use std::sync::Arc;
use chrono::Duration;

pub struct AuthService {
    auth: Auth,
}

impl AuthService {
    // Initializes AuthService by creating a MongoDB client and passing it to Auth
    pub async fn new(conn_str: &str, db_name: &str, collection: &str, secret_key: Vec<u8>, company_name: String, token_expiration_hours: i64) -> mongodb::error::Result<Self> {
        let client_options = ClientOptions::parse(conn_str).await?;
        let client = Client::with_options(client_options)?;
        let client = Arc::new(client);
        let token_expiration = Duration::hours(token_expiration_hours);
        // Instance auth class
        let auth = Auth::new(client, secret_key, company_name, token_expiration, db_name.to_string(), collection.to_string()).await;
        Ok(AuthService { auth })
    }

    pub async fn perform_login(&self, username: &str, password: &str) -> serde_json::Value {
        self.auth.login(username, password).await
    }

    pub async fn perform_token_validation(&self, token: &str) -> serde_json::Value {
        self.auth.validate_token(token).await
    }

    pub async fn perform_token_refresh(&self, token: &str) -> serde_json::Value {
        self.auth.refresh_token(token).await
    }
}
