use std::sync::Arc;

use crate::api::vessels::{Vessel, VesselsRepository};

pub struct VesselsService {
    repository: Arc<VesselsRepository>,
}
impl VesselsService {
    pub fn new(repository: Arc<VesselsRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_vessels(&self) -> Vec<Vessel> {
        self.repository.get_all_vessels().await
    }
}