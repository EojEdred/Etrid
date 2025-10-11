use std::collections::HashSet;

pub enum Role {
    CommonPeer,
    StakePeer,
    ValidityNode,
    FlareNode,
    DecentralizedDirector,
}

pub struct RoleAssignment {
    pub role: String,
}

pub fn validate_role(input: &RoleAssignment) -> bool {
    input.role == "ValidRole"
}