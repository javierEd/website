mod about_page;
mod blog_page;
mod home_page;
mod not_found_page;
mod show_post_page;

pub use about_page::AboutPage;
pub use blog_page::BlogPage;

pub use home_page::HomePage;
pub use not_found_page::NotFoundPage;

#[allow(unused_imports)]
pub use show_post_page::{GetPost, ShowPostPage};
