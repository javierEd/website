use leptos::*;

use website_core::models::Post;

#[server]
pub async fn get_posts(page: i16) -> Result<(Vec<Post>, i16), ServerFnError> {
    let posts = Post::paginate(page, true)?;
    let pages_count = Post::pages_count(true)?;

    Ok((posts, pages_count))
}
