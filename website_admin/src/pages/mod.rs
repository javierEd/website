mod edit_post_page;
mod home_page;
mod login_page;
mod new_post_page;
mod not_found_page;
mod posts_page;

#[allow(unused_imports)]
pub use edit_post_page::{AttemptToUpdatePost, EditPostPage, GetPost};

pub use home_page::HomePage;

#[allow(unused_imports)]
pub use login_page::{AttemptToLogin, LoginPage};

#[allow(unused_imports)]
pub use new_post_page::{AttemptToCreatePost, NewPostPage};

pub use not_found_page::NotFoundPage;

#[allow(unused_imports)]
pub use posts_page::{AttemptToDeletePost, GetPosts, PostsPage};
