use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: Option<String>,
    pub password: String,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub status: Option<Status>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub id: String,
    pub delete_on: Option<String>,
    pub body: String,
    pub label: Option<String>,
}