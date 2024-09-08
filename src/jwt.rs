use actix_web::HttpResponse;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use crate::config;
use serde_json::json;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: ObjectId,
    pub email: String,
    pub exp: i64,
}

pub fn generate_token(user_id: ObjectId, email: String) -> String {
    // Generate a JWT token
    encode(
        &Header::default(),
        &json!({"id": user_id, "email": email, "exp": Utc::now().timestamp() + 60 * 60 * 24 * 365 }),
        &EncodingKey::from_secret(config::SECRET_KEY.as_ref()),
    ).unwrap()
}

pub fn get_claims_from_token(req: &actix_web::HttpRequest) -> Result<Claims, HttpResponse> {
    // Get JWT token from request header
    let auth_header = req.headers().get("Authorization");
    if auth_header.is_none() {
        return Err(HttpResponse::Unauthorized().body("Authorization header missing"));
    }

    let token = auth_header.unwrap().to_str().unwrap().replace("Bearer ", "");
    // println!("Token: {}", token);
    let decoding_key = DecodingKey::from_secret(config::SECRET_KEY.as_ref());
    let token_data = decode::<Claims>(&token, &decoding_key, &Validation::default());

    match token_data {
        Ok(claims) => Ok(claims.claims),
        Err(e) => {
            println!("Error decoding token: {}", e);
            Err(HttpResponse::Unauthorized().body("Invalid token"))
        }
    }
}
