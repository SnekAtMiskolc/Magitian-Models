use super::RawCommit;

/// # Commit
/// The commit struct represents a commit in the database.
/// They are meant to be created from RawCommits which are extracted from incoming packfiles.
/// They are both extracted on the http side and the ssh side.
/// Only create them manually if the user uses the webapp to push to a repository.
pub struct Commit<'a> {
    pub id: String,
    pub name: Option<&'a str>,
    pub email: String,
    pub message: String,
    pub commit: String,
}

impl<'a> Commit<'a> {
    pub fn new(id: String, name: Option<&'a str>, email: String, message: String, commit: String) -> Self {
        Self {
            id,
            name,
            email,
            message,
            commit,
        }
    }
}

impl<'a> From<RawCommit> for Commit<'a> {
    fn from(c: RawCommit) -> Self {
        // TODO: Change this to a Some(message) type deal
        let message: String;
        if let Some(m) = c.message() {
            message = m.to_string();
        } else {
            message = String::from("")
        }
        Self {
            // Assign an UUID to the commit that will be stored in the database.
            id: uuid::Uuid::new_v4().to_string(),
            name: c.committer().name(),
            email: String::from("EMAIL"),
            // Extract the message from the commit.
            message,
            commit: c.id().to_string(),
        }
    }
}