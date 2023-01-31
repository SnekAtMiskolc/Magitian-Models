use git2::Commit as RawCommit;

/// # Commit
/// The commit struct represents a commit in the database.
/// They are meant to be created from RawCommits which are extracted from incoming packfiles.
/// They are both extracted on the http side and the ssh side.
/// Only create them manually if the user uses the webapp to push to a repository.
pub struct Commit<'a> {
    pub id: String,
    pub name: String,
    pub email: String,
    pub message: Option<&'a str>,
    pub commit: String,
}

impl<'a> Commit<'a> {
    pub fn new(id: String, name: String, email: String, message: Option<&'a str>,  commit: String) -> Self {
        Self {
            id,
            name,
            email,
            message,
            commit,
        }
    }
}

impl<'a> From<RawCommit<'static>> for Commit<'a> {
    fn from(c: RawCommit) -> Self {
        // Return a Commit struct from the RawCommit.
        Self {
            // Assign an UUID to the commit that will be stored in the database.
            id: uuid::Uuid::new_v4().to_string(),
            name: c.committer().to_string(),
            email: todo!(),
            // Extract the message from the commit.
            message: c.message(),
            commit: c.id().to_string(),
        }
    }
}