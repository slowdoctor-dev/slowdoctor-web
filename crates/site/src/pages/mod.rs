//! SSR page views split by route, with the shared shell in `layout`.

mod blog;
mod cv;
mod engineer;
mod home;
mod layout;
mod links;
mod not_found;
mod physician;

pub use blog::{blog, blog_post};
pub use cv::cv;
pub use engineer::engineer;
pub use home::home;
pub use layout::RenderedPage;
pub use links::links;
pub use not_found::not_found;
pub use physician::physician;

use layout::render;
