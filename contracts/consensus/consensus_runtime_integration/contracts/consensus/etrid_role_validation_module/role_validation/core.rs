use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleAssignment {
    pub role: String,
    pub credentials: String,
}

pub fn validate_role(assignment: &RoleAssignment) -> bool {
    assignment.role == "voter" && assignment.credentials == "valid"
}