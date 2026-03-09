use std::sync::Arc;

use axum::{Json, extract::State};

use crate::api::vessels::{Vessel, VesselsService};

#[derive(Clone)]
pub struct VesselsHandlers {
    service: Arc<dyn VesselsService + Send + Sync>,
}
impl VesselsHandlers {
    pub fn new(service: Arc<dyn VesselsService + Send + Sync>) -> Self {
        Self { service }
    }

    pub async fn get_vessels(State(handler): State<VesselsHandlers>) -> Json<Vec<Vessel>> {
        Json(handler.service.get_vessels().await)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::vessels::application::service::MockVesselsService;

    use super::*;

    #[tokio::test]
    async fn test_get_vessels() {
        let mut service_mock = MockVesselsService::new();
        let expected_result = vec![
            Vessel {
                id: "1".to_string(),
                name: "Sandra".to_string(),
                fuel_level: 100.0,
            },
            Vessel {
                id: "2".to_string(),
                name: "John".to_string(),
                fuel_level: 80.0,
            },
        ];
        let expected_result_clone = expected_result.clone();
        service_mock
            .expect_get_vessels()
            .returning(move || expected_result_clone.clone())
            .times(1);

        let handler = VesselsHandlers::new(Arc::new(service_mock));
        let vessels = handler.service.get_vessels().await;

        assert_eq!(vessels, expected_result);
    }
}
