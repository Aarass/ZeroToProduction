use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

pub struct TestApp {
    pub address: String,
    pub connection_pool: PgPool,
}
pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind port");
    let port = listener.local_addr().unwrap().port();

    let app_address = format!("http://127.0.0.1:{}", port);

    let configuration = get_configuration().expect("Failed to retrieve configuration");
    let connection_string = configuration.database.connection_string();

    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Couldn't create connection pool");

    let server = run(listener, connection_pool.clone()).expect("Failed to run server");
    let _ = tokio::spawn(server);
    TestApp {
        address: app_address,
        connection_pool: connection_pool,
    }
}
