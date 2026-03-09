use sea_orm::DatabaseConnection;

use super::Vessel;

pub struct VesselsRepository {
    connection: DatabaseConnection,
}
impl VesselsRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn get_all_vessels(&self) -> Vec<Vessel> {
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