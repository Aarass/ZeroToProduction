use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::NewSubscriber;
#[derive(Deserialize)]
pub struct UserData {
    pub name: String,
    pub email: String,
}

pub async fn subscribe(form: web::Form<UserData>, connection: web::Data<PgPool>) -> impl Responder {
    let form = form.into_inner();

    let Ok(new_subscriber) = form.try_into() else {
        println!("Invalid data passed to subscribe");
        return HttpResponse::BadRequest();
    };

    let Ok(_) = insert_new_subscriber(new_subscriber, &connection).await else {
        println!("Couldn't insert new subscription");
        return HttpResponse::BadRequest();
    };

    HttpResponse::Ok()
}
pub async fn insert_new_subscriber(
    new_subscriber: NewSubscriber,
    connection: &PgPool,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    println!("{:?}", new_subscriber);
    sqlx::query!(
        "INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES($1, $2, $3, $4)",
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(connection)
    .await
}
