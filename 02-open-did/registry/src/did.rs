use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct DIDDocument {
    pub id: String,
    pub controller: String,
    pub public_key: String,
    pub service_endpoint: String,
}

pub fn create_did(controller: &str) -> DIDDocument {
    DIDDocument {
        id: format!("did:etrid:{}", Uuid::new_v4()),
        controller: controller.to_string(),
        public_key: "04bfcab27...".to_string(), // placeholder
        service_endpoint: "https://identity.etrid.net".to_string(),
    }
}
