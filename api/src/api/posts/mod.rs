mod all_categories;
mod create_category;
mod create_post;
mod create_reply;
mod posts_from_category;
mod replies_from_post;

pub use all_categories::route as all_categories_route;
pub use create_category::route as create_category_route;
pub use create_post::route as create_post_route;
pub use create_reply::route as create_reply_route;
pub use posts_from_category::route as posts_from_category_route;
pub use replies_from_post::route as replies_from_post_route;
