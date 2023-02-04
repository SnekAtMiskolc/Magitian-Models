use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}