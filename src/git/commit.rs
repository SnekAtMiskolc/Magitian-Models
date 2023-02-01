use std::io::Cursor;

use git2::{Repository, ObjectType};

use super::RawCommit;

/// # Commit
/// The commit struct represents a commit in the database.
/// They are meant to be created from ```RawCommits``` which are extracted from incoming packfiles.
/// They are both extracted on the http side and the ssh side.
/// Only create them manually if the user uses the webapp to push to a repository.
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

    /// Return a ```Vec<Commit>``` from commits that have not yet been stored in the database.
    /// This means that we need to iterate objects in the repo's ODB (Object Database) and see if they
    /// are a commit, if that is the case then we check if it is already in the database if so
    /// then we shall not store it otherwise we push it to the ```Vec<Commit>``` after we use ```Into<RawCommit>``` on it.
    pub fn from_unadded() -> Vec<Self> {
        todo!()
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
