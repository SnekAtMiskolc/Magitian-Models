use super::RawCommit;
use serde_derive::{Deserialize, Serialize};

/// # Commit
/// The commit struct represents a commit in the database.
/// They are meant to be created from ```RawCommits``` which are extracted from incoming packfiles.
/// They are both extracted on the http side and the ssh side.
/// Only create them manually if the user uses the webapp to push to a repository.
#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub id: String,
    pub name: Option<String>,
    pub email: String,
    pub message: Option<String>,
    pub commit: String,
}

impl Commit {
    /// Creates a new ```Commit```
    pub fn new(
        id: String,
        name: Option<String>,
        email: String,
        message: Option<String>,
        commit: String,
    ) -> Self {
        Self {
            id,
            name,
            email,
            message,
            commit,
        }
    }
}

impl From<RawCommit> for Commit {
    fn from(c: RawCommit) -> Self {
        let message = c.message().map(|m| m.to_string());
        let name = c.committer().name().map(|n| n.to_string());
        Self {
            // Assign an UUID to the commit that will be stored in the database.
            id: uuid::Uuid::new_v4().to_string(),
            name,
            email: String::from("EMAIL"),
            // Extract the message from the commit.
            message,
            commit: c.id().to_string(),
        }
    }
}
