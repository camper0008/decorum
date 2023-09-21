use crate::db::models::Permission;

pub fn is_allowed(user_permission: &Permission, required_permission: &Permission) -> bool {
    use Permission::*;
    match (user_permission, required_permission) {
        (Unverified, Unverified) => true,
        (Unverified, User | Admin | Root) => false,
        (User, Unverified | User) => true,
        (User, Admin | Root) => false,
        (Admin, Unverified | User | Admin) => true,
        (Admin, Root) => false,
        (Root, Unverified | Admin | User | Root) => true,
    }
}
