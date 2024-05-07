pub async fn verify_token(&self, token: &str) -> Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(&self.secret_key),
        &Validation::default(),
    )
}

pub async fn refresh_token(&self, token: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let token_data = self.verify_token(token).await?;
    if Utc::now().timestamp() as usize > token_data.claims.exp {
        return Err(jsonwebtoken::errors::Error::ExpiredSignature);
    }
    self.generate_token(&token_data.claims.sub, Duration::hours(1)).await
}

pub async fn add_user(&self, user_info: &UserInfo) -> mongodb::error::Result<()> {
    let collection = self.client.database("your_db_name").collection("users");
    collection.insert_one(user_info, None).await?;
    Ok(())
}
