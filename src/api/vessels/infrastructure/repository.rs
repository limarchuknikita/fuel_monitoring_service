use mockall::automock;
use sea_orm::DatabaseConnection;

use super::Vessel;

pub struct VesselsInfrastructureRepository {
    connection: DatabaseConnection,
}
impl VesselsInfrastructureRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }

    
}
#[async_trait::async_trait]
impl VesselsRepository for VesselsInfrastructureRepository {
    async fn get_all_vessels(&self) -> Vec<Vessel> {
        let _ = &self.connection;
        vec![
            Vessel {
                id:"1".to_string(),
                name:"Vessel 1".to_string(), 
                fuel_level: 50.0 
            },
            Vessel {
                id: "2".to_string(),
                name: "Vessel 2".to_string(),
                fuel_level: 100.0,
            },
        ]
    }
}

#[async_trait::async_trait]
#[automock]
pub trait VesselsRepository {
    async fn get_all_vessels(&self) -> Vec<Vessel>;
}