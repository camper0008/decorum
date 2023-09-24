use crate::db::models::Permission;

pub fn is_allowed(user_permission: &Permission, required_permission: &Permission) -> bool {
    use Permission::{Admin, Banned, Root, Unverified, User};
    match (user_permission, required_permission) {
        (Banned, Banned)
        | (Unverified, Unverified)
        | (User, Unverified | User)
        | (Admin, Unverified | User | Admin)
        | (Root, Unverified | Admin | User | Root) => true,

        (_, Banned)
        | (Banned, _)
        | (Unverified, User | Admin | Root)
        | (User, Admin | Root)
        | (Admin, Root) => false,
    }
}

pub fn permission_for_attachment_upload() -> Permission {
    Permission::User
}

pub fn permission_for_important_actions() -> Permission {
    Permission::Admin
}
