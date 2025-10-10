#[cfg(test)]
mod tests {
    use super::*;
    use consensus::role_validation::{validate_role, RoleAssignment};

    #[test]
    fn test_valid_role_assignment() {
        let assignment = RoleAssignment::new("peer1", "FlareNode");
        assert!(validate_role(&assignment));
    }
}