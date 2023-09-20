use crate::db::models::Permission;

pub fn is_allowed(user_permission: &Permission, required_permission: &Permission) -> bool {
    use Permission::*;
    match (user_permission, required_permission) {
        (Visitor, Visitor) => true,
        (Visitor, User | Admin | Root) => false,
        (User, Visitor | User) => true,
        (User, Admin | Root) => false,
        (Admin, Visitor | User | Admin) => true,
        (Admin, Root) => false,
        (Root, Visitor | Admin | User | Root) => true,
    }
}
