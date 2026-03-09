mod api;
mod context_containers;
pub use crate::api::add_vessel_routes;

use axum::{Router, routing::get};
use tracing::info;

use crate::context_containers::InfraContextContainer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    info!("starting fuel_monitoring_service");

    let infra = InfraContextContainer::new().await?;
    info!("database pool initialized");

    let app = Router::new()
        .route("/health", get(|| async { "Still alive" }))
        .route("/ready", get(|| async { "Ready to accept traffic" }))
        .merge(add_vessel_routes(infra.db.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
