use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

/// Establish a connection pool to PostgreSQL via SeaORM.
pub async fn connect(database_url: &str) -> DatabaseConnection {
    let mut opts = ConnectOptions::new(database_url);

    opts.max_connections(20)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(600))
        .sqlx_logging(true);

    Database::connect(opts)
        .await
        .expect("Failed to connect to database")
}
