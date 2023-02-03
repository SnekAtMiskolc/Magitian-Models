use serde_derive::{Deserialize, Serialize};

/*
Used for notifications.
If Someone pushes we: 
 - write to the ODB
 - Extract the added commits
 - Assign commits that have been pushed in one request a pushID
*/ 
#[derive(Serialize, Deserialize)]
pub struct Push {
    pub id: String,
    // Optional since the user might not have an email address.
    pub user_id: Option<String>,
}