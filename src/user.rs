use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use mongodb::{bson::oid::ObjectId, bson::doc};
use serde::{Deserialize, Serialize};
use crate::db;
use crate::jwt;
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInput {
    pub email: String,
    pub password: String,
}

pub async fn register_user(user: web::Json<UserInput>, state: web::Data<db::AppState>) -> impl Responder {
    let collection = state.db.collection("users");
    let password_hash = hash(&user.password, DEFAULT_COST).unwrap();

    let new_user = User {
        id: ObjectId::new(),
        email: user.email.clone(),
        password_hash,
    };

    let result = collection.find_one(doc! {"email": &user.email}, None).await.unwrap();
    if result.is_none() {
        // No existing user, insert new user
        collection.insert_one(new_user, None).await.unwrap();
        HttpResponse::Ok().json("Congratulations! Enjoy it! 😊")
    } else {
        // User already exists
        HttpResponse::Conflict().json(json!({"msg":"Oh, you already joined our garden. Enjoy it! 😂"}))
    }
}

pub async fn login_user(user_input: web::Json<UserInput>, state: web::Data<db::AppState>) -> impl Responder {
    let collection: mongodb::Collection<User> = state.db.collection("users");
    let result = collection.find_one(doc! {"email": &user_input.email}, None).await.unwrap();

    match result {
        Some(user) => {
            // let user: User = doc.unwrap();
            if verify( &user_input.password,&user.password_hash).unwrap() {
                // Generate a JWT token (simple implementation)
                let token = jwt::generate_token(user.id, user.email);
                HttpResponse::Ok().json(json!({ "token": token }))
            } else {
                HttpResponse::Unauthorized().json(json!({"msg":"Ops, maybe input a password. 😔 Try it again"}))
            }
        }
        None => {
            // User not found, create a new one
            let password_hash = hash(&user_input.password, DEFAULT_COST).unwrap();
            let user_id = ObjectId::new();
            let new_user = User {
                id: user_id.clone(),
                email: user_input.email.clone(),
                password_hash,
            };
            collection.insert_one(new_user, None).await.unwrap();
            // Generate a JWT token for the new user
            let token = jwt::generate_token(user_id, user_input.email.clone());
            HttpResponse::Ok().json(json!({ "token": token }))
        }
    }
}