use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::api::{VesselsHandlers, VesselsInfrastructureRepository, VesselsService, VesselsRepository, VesselsApplicationService};

pub struct VesselsContextContainer {
    pub handler: VesselsHandlers,
}
impl VesselsContextContainer {
    pub fn new(db: DatabaseConnection) -> Self {
        let repository = Arc::new(
            VesselsInfrastructureRepository::new(db)
        ) as Arc<dyn VesselsRepository + Send + Sync>;
        let service = Arc::new(
            VesselsApplicationService::new(
                Arc::clone(&repository)
            )
        ) as Arc<dyn VesselsService + Send + Sync>;
        let handler = VesselsHandlers::new(
            Arc::clone(&service) as Arc<dyn VesselsService + Send + Sync>
        );

        Self { handler }
    }
}