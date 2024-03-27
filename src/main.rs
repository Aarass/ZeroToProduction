use std::net::TcpListener;

use env_logger::Env;
use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let connection_string: String = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failder to connect to Postgres");

    let listener = TcpListener::bind(address).expect("Could not bind port");
    run(listener, connection_pool)?.await
}
