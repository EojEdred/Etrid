pub mod core;

use core::{RoleAssignment, validate_role};

pub fn is_valid_assignment(assignment: &RoleAssignment) -> bool {
    validate_role(assignment)
}