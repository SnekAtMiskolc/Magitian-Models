use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub issue_id: String,
    pub user_id: String,
    pub body: String,
}