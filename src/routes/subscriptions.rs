use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct UserData {
    name: String,
    email: String,
}

async fn subscribe(form: web::Form<UserData>) -> impl Responder {
    HttpResponse::Ok()
}
