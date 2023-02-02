pub struct OrganizationAT {
    pub id: String,
    pub org_id: String,
    pub user_id: String,
    // Change to an elaborate struct
    pub role: Role,
    pub expires: String,
}

pub struct RepoAT {
    pub id: String,
    pub repo_id: String,
    pub user_id: String,
    // Change to an elaborate struct
    pub role: Role,
    pub expires: String,
}

pub enum Role {
    // Has the perms to accept pull request
    Manager,
    // Can actually work on the repo
    Developer,
    // Has read only access to the repo/org if it is private
    Visitor,
    // Has read only access to the repo/org if it is private
    Auditor,
}