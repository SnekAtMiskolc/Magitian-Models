use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub owner_id: String,
    pub description: Option<String>,
    pub org_logo: Option<String>,
}