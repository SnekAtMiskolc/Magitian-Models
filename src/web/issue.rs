/// Represents an issue in the database!
/// It can reference a commit to discuss in the issue but it mmust always reference a repository.
pub struct Issue {
    id: String,
    user_id: String,
    repository_id: String,
    commit_id: Option<String>,
    title: String,
    body: String,
    status: String,
}

impl Issue {
    pub fn new(
        id: String,
        user_id: String,
        repository_id: String,
        commit_id: Option<String>,
        title: String,
        body: String,
        status: String,
    ) -> Self {
        Self {
            id,
            user_id,
            repository_id,
            commit_id,
            title,
            body,
            status,
        }
    }
    pub fn get_comments() {
        todo!()
    }
    pub fn insert() {
        todo!()
    }
    pub fn delete() {
        todo!()
    }
}
