#[cfg(feature = "ssr")]
lazy_static::lazy_static! {
    pub static ref CONNECTION: spin_sdk::sqlite::Connection = spin_sdk::sqlite::Connection::open_default().expect("Could not open connection");
    pub static ref DATETIME_FORMAT: &'static [time::format_description::BorrowedFormatItem<'static>] = time::macros::format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    pub static ref REGEX_SLUG: regex::Regex = regex::Regex::new(r"\A[a-z0-9]+(?:-[a-z0-9]+)*\z").unwrap();
}

mod post;

pub use post::Post;
