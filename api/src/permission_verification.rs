use crate::db::models::Permission;

pub fn is_allowed(user_permission: &Permission, required_permission: &Permission) -> bool {
    use Permission::{Admin, Root, Unverified, User};
    match (user_permission, required_permission) {
        (Unverified, Unverified)
        | (User, Unverified | User)
        | (Admin, Unverified | User | Admin)
        | (Root, Unverified | Admin | User | Root) => true,

        (Unverified, User | Admin | Root) | (User, Admin | Root) | (Admin, Root) => false,
    }
}
