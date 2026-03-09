use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::api::{VesselsHandlers, VesselsRepository, VesselsService};

pub struct VesselsContextContainer {
    pub handler: VesselsHandlers,
}
impl VesselsContextContainer {
    pub fn new(db: DatabaseConnection) -> Self {
        let repository = Arc::new(VesselsRepository::new(db));
        let service = Arc::new(VesselsService::new(Arc::clone(&repository)));
        let handler = VesselsHandlers::new(Arc::clone(&service));

        Self { handler }
    }
}