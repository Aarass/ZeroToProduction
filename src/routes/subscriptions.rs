use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
#[derive(Deserialize)]
pub struct UserData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<UserData>, connection: web::Data<PgPool>) -> impl Responder {
    log::info!("Inserting new subscription");
    sqlx::query!(
        "INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES($1, $2, $3, $4)",
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await
    .expect("Failed to insert new subrscription");
    log::info!("New subscription inserted");
    HttpResponse::Ok()
}
