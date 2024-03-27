use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<UserData>) -> impl Responder {
    HttpResponse::Ok()
}
