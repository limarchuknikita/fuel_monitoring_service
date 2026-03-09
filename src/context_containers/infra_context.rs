use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub struct InfraContextContainer {
    pub db: DatabaseConnection,
}
impl InfraContextContainer {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:password@localhost:5434/fuel_monitoring".to_string());
        let mut options = ConnectOptions::new(database_url);

        options.max_connections(100)
            .min_connections(5)
            .connect_timeout(std::time::Duration::from_secs(30))
            .idle_timeout(std::time::Duration::from_secs(300))
            .sqlx_logging(true);

        let db = Database::connect(options).await?;
        Ok(Self { db })
    }
}