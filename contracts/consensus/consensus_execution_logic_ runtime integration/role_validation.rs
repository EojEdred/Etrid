
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Role {
    Admin,
    Director,
    FlareNode,
    ValidityNode,
    CommonStakePeer,
    Anonymous,
}

pub struct RoleValidator {
    pub permissions: HashMap<Role, Vec<String>>, // role -> allowed actions
}

impl RoleValidator {
    pub fn new() -> Self {
        let mut permissions = HashMap::new();
        permissions.insert(Role::Admin, vec!["mint".into(), "slash".into()]);
        permissions.insert(Role::Director, vec!["propose".into(), "vote".into()]);
        permissions.insert(Role::CommonStakePeer, vec!["vote".into()]);
        Self { permissions }
    }

    pub fn can(&self, role: Role, action: &str) -> bool {
        self.permissions.get(&role)
            .map(|actions| actions.contains(&action.to_string()))
            .unwrap_or(false)
    }
}
