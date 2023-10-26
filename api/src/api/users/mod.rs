mod edit_user;
mod edit_user_permission;
mod login;
mod register;
mod user_from_id;
mod user_from_session;

pub use edit_user::route as edit_user_route;
pub use edit_user_permission::route as edit_user_permission_route;
pub use login::route as login_route;
pub use register::route as register_route;
pub use user_from_id::route as user_from_id_route;
pub use user_from_session::route as user_from_session_route;
