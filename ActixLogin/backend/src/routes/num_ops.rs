// src/routes/health.rs
use actix_web::{post, web, HttpResponse};
use std::sync::{Arc, Mutex};
use serde::Deserialize;

lazy_static::lazy_static! {
    static ref NUM: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}

#[tracing::instrument]
#[actix_web::get("/get-num/")]
pub async fn get_num() -> actix_web::HttpResponse {
    tracing::event!(target: "backend", tracing::Level::DEBUG, "Accessing get-num endpoint.");
    let mut num = NUM.lock().unwrap();
    *num += 1;
    actix_web::HttpResponse::Ok().json(*num)
}

#[derive(Deserialize, Debug)]
struct InputData {
    number: i32,
}

#[tracing::instrument]
#[post("/add-num/")]
pub async fn add_num(data: web::Json<InputData>) -> HttpResponse {
    tracing::event!(target: "backend", tracing::Level::DEBUG, "Accessing add-num endpoint.");
    let mut num = NUM.lock().unwrap();
    *num += data.number;
    HttpResponse::Ok().json(*num)
}