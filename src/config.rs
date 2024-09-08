// src/config.rs
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref SECRET_KEY: String = {
        env::var("SECRET_KEY").expect("SECRET_KEY must be set")
    };
    pub static ref MONGO_URI: String = {
        env::var("MONGO_URI").expect("MONGO_URI must be set")
    };
    pub static ref MONGO_DB_NAME: String = {
        env::var("MONGO_DB").expect("MONGO_DB must be set")
    };
}