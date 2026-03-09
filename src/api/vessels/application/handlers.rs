use std::sync::Arc;

use axum::{extract::State, Json};

use crate::api::vessels::{Vessel, VesselsService};

#[derive(Clone)]
pub struct VesselsHandlers {
    service: Arc<VesselsService>,
}
impl VesselsHandlers {
    pub fn new(service: Arc<VesselsService>) -> Self {
        Self { service }
    }

    pub async fn get_vessels(State(handler): State<VesselsHandlers>) -> Json<Vec<Vessel>> {
        Json(handler.service.get_vessels().await)
    }
}