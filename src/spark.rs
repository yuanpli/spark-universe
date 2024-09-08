use actix_web::{web, HttpResponse, Responder};
use futures::stream::StreamExt;
use mongodb::{bson::oid::ObjectId, bson::doc, options::FindOptions, Cursor};
use serde::{Deserialize, Serialize};
use crate::db;
use crate::jwt;
use crate::user;
use chrono::Utc;
use serde_json::json;


#[derive(Serialize, Deserialize, Debug)]
pub struct Spark {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub content: String,
    pub create_by: ObjectId,
    pub create_at: String,
    pub like_count: i32,
    pub like_users: Vec<ObjectId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewSpark {
    pub content: String,
}

pub async fn get_sparks_by_date(state: web::Data<db::AppState>) -> impl Responder {
    let collection = state.db.collection::<Spark>("sparks");

    let find_options = FindOptions::builder()
        .sort(doc! { "create_at": -1 })  
        .limit(100) 
        .build();

    let mut cursor: Cursor<Spark> = collection.find(None, find_options).await.unwrap();
    let mut sparks: Vec<Spark> = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(spark) => sparks.push(spark),
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        }
    }

    HttpResponse::Ok().json(sparks)
}

pub async fn get_sparks_by_star(state: web::Data<db::AppState>) -> impl Responder {
    let collection = state.db.collection::<Spark>("sparks");
     let find_options = FindOptions::builder()
        .sort(doc! { "like_count": -1 })
        .limit(100) 
        .build();

    let mut cursor: Cursor<Spark> = collection.find(None, find_options).await.unwrap();
    let mut sparks: Vec<Spark> = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(spark) => sparks.push(spark),
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        }
    }

    HttpResponse::Ok().json(sparks)
}

pub async fn add_spark(spark: web::Json<NewSpark>, state: web::Data<db::AppState>, req: actix_web::HttpRequest) -> impl Responder {
    let collection = state.db.collection::<Spark>("sparks");

    let claims = match jwt::get_claims_from_token(&req) {
        Ok(claims) => claims,
        Err(error_response) => return error_response,
    };

    // find user
    let user_collection = state.db.collection::<user::User>("users");
    let user_doc = user_collection.find_one(doc! {"email": &claims.email}, None).await.unwrap();
    if user_doc.is_none() {
        return HttpResponse::Unauthorized().json(json!({"msg":"User flied!😭"}));
    }

    let user = user_doc.unwrap();

    // create a new spark document
    let new_spark = Spark {
        id: ObjectId::new(),
        content: spark.content.clone(),
        create_by: user.id,
        create_at: Utc::now().to_rfc3339(),
        like_count: 0,
        like_users: vec![],
    };

    collection.insert_one(new_spark, None).await.unwrap();

    HttpResponse::Ok().json(json!({"msg":"Spark added"}))
}

pub async fn like_spark(path: web::Path<String>, state: web::Data<db::AppState>, req: actix_web::HttpRequest) -> impl Responder {
    let collection = state.db.collection::<Spark>("sparks");

    let claims = match jwt::get_claims_from_token(&req) {
        Ok(claims) => claims,
        Err(error_response) => return error_response,
    };

    let spark_id = path.into_inner();
    let object_id = ObjectId::parse_str(&spark_id).unwrap();

    let spark = collection.find_one(doc! {"_id": object_id}, None).await.unwrap();
    match spark {
        Some(mut spark) => {
            // check if has liked
            if spark.like_users.contains(&claims.id) {
                return HttpResponse::Ok().body("")
            }
            // update like_users
            // update like_count
            spark.like_users.push(claims.id);
            spark.like_count += 1;
            collection.replace_one(doc! {"_id": object_id}, spark, None).await.unwrap();
            HttpResponse::Ok().body("👍")
        },
        None =>  HttpResponse::NotFound().json(json!({"msg":"Spark gone 😭"}))
    }
}