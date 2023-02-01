use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Repository {
    pub id: String,
    pub owner_id: String,
    pub repo_src: String,
    pub repo_logo: String,
}