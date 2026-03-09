use std::sync::Arc;

use mockall::automock;

use crate::api::{VesselsRepository, vessels::Vessel};

pub struct VesselsApplicationService {
    repository: Arc<dyn VesselsRepository + Send + Sync>,
}
impl VesselsApplicationService {
    pub fn new(repository: Arc<dyn VesselsRepository + Send + Sync>) -> Self {
        Self { repository }
    }
}
#[async_trait::async_trait]
impl VesselsService for VesselsApplicationService {
    async fn get_vessels(&self) -> Vec<Vessel> {
        self.repository.get_all_vessels().await
    }
}

#[automock]
#[async_trait::async_trait]
pub trait VesselsService {
    async fn get_vessels(&self) -> Vec<Vessel>;
}

#[cfg(test)]
mod tests {
    use crate::api::vessels::infrastructure::repository::MockVesselsRepository;

    use super::*;

    #[tokio::test]
    async fn test_get_vessels() {
        let mut repository_mock = MockVesselsRepository::new();
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

        repository_mock
            .expect_get_all_vessels()
            .returning(move || {
                let result = expected_result_clone.clone();
                Box::pin(async move { result })
            })
            .times(1);

        let service = VesselsApplicationService::new(Arc::new(repository_mock));
        let vessels = service.get_vessels().await;

        assert_eq!(vessels, expected_result);
    }
}
