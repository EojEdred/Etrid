
pub struct RoleAssignment {
    pub peer_id: String,
    pub is_authorized: bool,
}

pub fn validate_role(assignment: &RoleAssignment) -> bool {
    assignment.is_authorized
}
