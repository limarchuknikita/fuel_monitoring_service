use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;

use crate::{api::vessels::VesselsHandlers, context_containers::VesselsContextContainer};

pub fn add_vessel_routes(db: DatabaseConnection) -> Router {
    let context = VesselsContextContainer::new(db);

    Router::new()
        .route("/vessels", get(VesselsHandlers::get_vessels))
        .with_state(context.handler)
}