extern crate jsonwebtoken;
extern crate mongodb;
extern crate chrono;

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use mongodb::{Client, options::ClientOptions};
use chrono::{Utc, Duration};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

impl Claims {
    fn new(user_id: &str, duration: Duration) -> Self {
        Claims {
            sub: user_id.to_owned(),
            company: "ExampleCorp".to_owned(),
            exp: (Utc::now() + duration).timestamp() as usize,
        }
    }
}

pub struct Auth {
    client: Client,
    secret_key: Vec<u8>,
}

impl Auth {
    pub async fn new(db_uri: &str, secret_key: Vec<u8>) -> mongodb::error::Result<Self> {
        let client_options = ClientOptions::parse(db_uri).await?;
        let client = Client::with_options(client_options)?;
        Ok(Auth { client, secret_key })
    }

    pub async fn generate_token(&self, user_id: &str, duration: Duration) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims::new(user_id, duration);
        encode(&Header::default(), &claims, &EncodingKey::from_secret(&self.secret_key))
    }
}
