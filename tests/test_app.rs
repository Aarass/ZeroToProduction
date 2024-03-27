use std::net::TcpListener;

use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::{configuration::get_configuration, startup::run};

pub struct TestApp {
    pub address: String,
    pub connection_pool: PgPool,
}
pub async fn spawn_app() -> TestApp {
    let mut configuration = get_configuration().expect("Failed to retrieve configuration");

    let mut pi_connection =
        PgConnection::connect(&configuration.database.connection_string_without_db())
            .await
            .expect("Couldn't connect to postgress instancek");

    configuration.database.database_name = Uuid::new_v4().to_string();

    pi_connection
        .execute(
            format!(
                "CREATE DATABASE \"{}\";",
                configuration.database.database_name
            )
            .as_str(),
        )
        .await
        .expect("Couldn't create a database");

    let connection_string = configuration.database.connection_string();

    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Couldn't create connection pool");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Couldn't migrate");

    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind port");

    let port = listener.local_addr().unwrap().port();
    let app_address = format!("http://127.0.0.1:{}", port);

    let server = run(listener, connection_pool.clone()).expect("Failed to run server");
    let _ = tokio::spawn(server);

    TestApp {
        address: app_address,
        connection_pool: connection_pool,
    }
}
