// src/models/auth_model.rs

use mongodb::{Client, options::ClientOptions, bson::doc};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

impl Claims {
    pub fn new(user_id: &str, company: &str, exp: usize) -> Self {
        Claims {
            sub: user_id.to_owned(),
            company: company.to_owned(),
            exp: exp,
        }
    }
}

pub struct Auth {
    pub client: Arc<Client>,
    pub secret_key: Vec<u8>,
    pub company_name: String,
    pub token_expiration: Duration,
    pub db_name: String,
    pub collection_name: String,
}

impl Auth {
    pub async fn new(client: Arc<Client>, secret_key: Vec<u8>, company_name: String, token_expiration: Duration, db_name: String, collection_name: String) -> Self {
        Auth { client, secret_key, company_name, token_expiration, db_name, collection_name }
    }

    pub async fn generate_token(&self, user_id: &str) -> Result<String, JwtError> {
        let expiration_time = (Utc::now() + self.token_expiration).timestamp() as usize;
        let claims = Claims::new(user_id, &self.company_name, expiration_time);
        encode(&Header::default(), &claims, &EncodingKey::from_secret(&self.secret_key))
    }

    pub async fn validate_token(&self, token: &str) -> serde_json::Value {
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret_key),
            &Validation::default()
        ) {
            Ok(token_data) => json!({
                "success": true,
                "code": 200,
                "message": "Token verification successful",
                "data": token_data.claims
            }),
            Err(e) => json!({
                "success": false,
                "code": 500,
                "message": e.to_string(),
                "data": {}
            })
        }
    }

    pub async fn refresh_token(&self, token: &str) -> serde_json::Value {
        let validation = self.validate_token(token).await;
        if validation["success"] == true && validation["data"]["exp"].as_u64().unwrap_or(0) > Utc::now().timestamp() as u64 {
            if let Some(sub) = validation["data"]["sub"].as_str() {
                return self.generate_token(sub).await
                    .map(|new_token| {
                        let expiration_time = (Utc::now() + self.token_expiration).to_rfc3339();
                        json!({
                            "success": true,
                            "code": 200,
                            "message": "Token refreshed successfully",
                            "data": {
                                "token": new_token,
                                "expired_in": expiration_time
                            }
                        })
                    })
                    .unwrap_or_else(|e| json!({
                        "success": false,
                        "code": 500,
                        "message": e.to_string(),
                        "data": {}
                    }));
            }
        }
        json!({
            "success": false,
            "code": 401,
            "message": "Token is invalid or expired",
            "data": {}
        })
    }

    pub async fn login(&self, username: &str, password: &str) -> serde_json::Value {
        let user_collection = self.client.database(&self.db_name).collection::<UserInfo>(&self.collection_name);
        let user = user_collection.find_one(doc! {"username": username, "password": password}, None).await;

        match user {
            Ok(Some(_user_data)) => {
                match self.generate_token(username).await {
                    Ok(token) => {
                        let expiration_time = (Utc::now() + self.token_expiration).to_rfc3339();
                        json!({
                            "success": true,
                            "code": 200,
                            "message": "Login executed correctly",
                            "data": {
                                "token": token,
                                "expired_in": expiration_time
                            }
                        })
                    },
                    Err(e) => json!({
                        "success": false,
                        "code": 500,
                        "message": e.to_string(),
                        "data": {}
                    }),
                }
            },
            Ok(None) => json!({
                "success": false,
                "code": 401,
                "message": "Invalid credentials",
                "data": {}
            }),
            Err(e) => json!({
                "success": false,
                "code": 500,
                "message": format!("Exception error: {}", e),
                "data": {}
            }),
        }
    }
}
