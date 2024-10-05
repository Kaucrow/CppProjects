// src/routes/health.rs
use actix_web::{post, web, HttpResponse};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

lazy_static::lazy_static! {
    static ref NUM: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}

#[derive(Serialize, Debug)]
struct NumResponse {
    number: i32,
}

#[tracing::instrument]
#[actix_web::get("/get-num")]
pub async fn get_num() -> HttpResponse {
    tracing::event!(target: "backend", tracing::Level::DEBUG, "Accessing get-num endpoint.");
    let mut num = NUM.lock().unwrap();
    *num += 1;
    //let res = format!("{:#?}", actix_web::HttpResponse::Ok().json(*number));
    //tracing::event!(target: "backend", tracing::Level::DEBUG, res);
    let response = NumResponse{ number: *num };
    HttpResponse::Ok().json(response)
}

#[derive(Deserialize, Debug)]
struct AddNumBody {
    number: i32,
}

#[tracing::instrument]
#[post("/add-num/")]
pub async fn add_num(data: web::Json<AddNumBody>) -> HttpResponse {
    tracing::event!(target: "backend", tracing::Level::DEBUG, "Accessing add-num endpoint.");
    let add_num = data.number;
    let mut num = NUM.lock().unwrap();
    *num += add_num;
    let response = NumResponse{ number: *num };
    let test = format!("Sending {response:?}");
    tracing::event!(target: "backend", tracing::Level::DEBUG, test);
    HttpResponse::Ok().json(response)
}