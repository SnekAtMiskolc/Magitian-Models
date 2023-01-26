pub struct Commit {
    pub name: String,
    pub email: String,
    pub commit: String,
}

impl Commit {
    pub fn new(name: String, email: String, commit: String) -> Self {
        Self {
            name,
            email,
            commit,
        }
    }
}