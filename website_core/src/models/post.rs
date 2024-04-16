use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[cfg(feature = "ssr")]
use super::{CONNECTION, DATETIME_FORMAT, REGEX_SLUG};

const DEFAULT_FORMATTER: &[time::format_description::BorrowedFormatItem<'static>] =
    time::macros::format_description!("[year]-[month]-[day] [hour]:[minute]:[second] UTC");

#[derive(Clone, Deserialize, Serialize)]
pub struct Post {
    pub id: i16,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl Post {
    fn time_ago(value: OffsetDateTime) -> (i32, &'static str) {
        let time_ago_secs = (time::OffsetDateTime::now_utc() - value).as_seconds_f32().round() as i32;

        if time_ago_secs < 60 {
            (time_ago_secs, "secs")
        } else if time_ago_secs < 3600 {
            (time_ago_secs / 60, "mins")
        } else if time_ago_secs < 86400 {
            (time_ago_secs / 3600, "hours")
        } else if time_ago_secs < 2592000 {
            (time_ago_secs / 86400, "days")
        } else if time_ago_secs < 31104000 {
            (time_ago_secs / 2592000, "months")
        } else {
            (time_ago_secs / 31104000, "years")
        }
    }

    pub fn published_at_formatted(&self) -> Option<String> {
        self.published_at.and_then(|pa| pa.format(DEFAULT_FORMATTER).ok())
    }

    pub fn published_time_ago(&self) -> Option<(i32, &'static str)> {
        self.published_at.map(Self::time_ago)
    }

    pub fn created_at_formatted(&self) -> String {
        self.created_at.format(DEFAULT_FORMATTER).unwrap()
    }

    pub fn created_time_ago(&self) -> (i32, &'static str) {
        Self::time_ago(self.created_at)
    }

    pub fn updated_at_formatted(&self) -> Option<String> {
        self.updated_at.and_then(|ua| ua.format(DEFAULT_FORMATTER).ok())
    }

    pub fn updated_time_ago(&self) -> Option<(i32, &'static str)> {
        self.updated_at.map(Self::time_ago)
    }
}

#[cfg(feature = "ssr")]
impl Post {
    pub fn delete(&self) -> Result<(), spin_sdk::sqlite::Error> {
        CONNECTION
            .execute(
                "DELETE FROM posts WHERE id = ?",
                &[spin_sdk::sqlite::Value::Integer(self.id.into())],
            )
            .map(|_| ())
    }

    pub fn insert(title: &str, slug: &str, content: &str, publish: bool) -> Result<Self, spin_sdk::sqlite::Error> {
        Self::validate(title, slug, content)?;

        use spin_sdk::sqlite::Value;

        let result = CONNECTION.execute(
            "INSERT INTO posts (title, slug, content, published_at)
            VALUES (?, ?, ?, CASE ? WHEN 1 THEN CURRENT_TIMESTAMP ELSE NULL END)
            RETURNING *",
            &[
                Value::Text(title.to_owned()),
                Value::Text(slug.to_owned()),
                Value::Text(content.to_owned()),
                Value::Integer(publish.into()),
            ],
        )?;

        result
            .rows()
            .map(|row| row.into())
            .last()
            .ok_or(spin_sdk::sqlite::Error::Io("Failed to get post".to_owned()))
    }

    pub fn get_by_id(id: i16) -> Result<Self, spin_sdk::sqlite::Error> {
        let result = CONNECTION.execute(
            "SELECT * FROM posts WHERE id = ? LIMIT 1",
            &[spin_sdk::sqlite::Value::Integer(id.into())],
        )?;

        result
            .rows()
            .map(|row| row.into())
            .last()
            .ok_or(spin_sdk::sqlite::Error::Io("Failed to get post".to_owned()))
    }

    pub fn get_by_slug(slug: &str) -> Result<Self, spin_sdk::sqlite::Error> {
        let result = CONNECTION.execute(
            "SELECT * FROM posts WHERE slug = ? AND published_at IS NOT NULL LIMIT 1",
            &[spin_sdk::sqlite::Value::Text(slug.to_owned())],
        )?;

        result
            .rows()
            .map(|row| row.into())
            .last()
            .ok_or(spin_sdk::sqlite::Error::Io("Failed to get post".to_owned()))
    }

    pub fn pages_count(only_published: bool) -> Result<i16, spin_sdk::sqlite::Error> {
        let result = CONNECTION.execute(
            "SELECT CAST(p.pages_count_f AS int) + (p.pages_count_f > CAST(p.pages_count_f AS int)) AS pages_count
                FROM (
                    SELECT (COUNT(*) / 10.0) AS pages_count_f FROM posts WHERE (? IS 0 OR published_at IS NOT NULL)
                ) AS p",
            &[spin_sdk::sqlite::Value::Integer(only_published.into())],
        )?;

        result
            .rows()
            .map(|row| row.get("pages_count"))
            .last()
            .and_then(|pc| pc)
            .ok_or(spin_sdk::sqlite::Error::Io("Failed to get count".to_owned()))
    }

    pub fn paginate(page: i16, only_published: bool) -> Result<Vec<Self>, spin_sdk::sqlite::Error> {
        let result = CONNECTION.execute(
            "SELECT * FROM posts WHERE (? IS 0 OR published_at IS NOT NULL) ORDER BY published_at DESC, id DESC
            LIMIT 10 OFFSET ?",
            &[
                spin_sdk::sqlite::Value::Integer(only_published.into()),
                spin_sdk::sqlite::Value::Integer(((page - 1) * 10).into()),
            ],
        )?;

        Ok(result.rows().map(|row| row.into()).collect())
    }

    pub fn update(
        &self,
        title: &str,
        slug: &str,
        content: &str,
        publish: bool,
    ) -> Result<Self, spin_sdk::sqlite::Error> {
        Self::validate(title, slug, content)?;

        use spin_sdk::sqlite::Value;

        let result = CONNECTION.execute(
            "UPDATE posts SET title = ?, slug = ?, content = ?,
                published_at = CASE ?
                    WHEN 1 AND posts.published_at IS NOT NULL THEN posts.published_at
                    WHEN 1 THEN CURRENT_TIMESTAMP
                    ELSE NULL END,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            RETURNING *",
            &[
                Value::Text(title.to_owned()),
                Value::Text(slug.to_owned()),
                Value::Text(content.to_owned()),
                Value::Integer(publish.into()),
                Value::Integer(self.id.into()),
            ],
        )?;

        result
            .rows()
            .map(|row| row.into())
            .last()
            .ok_or(spin_sdk::sqlite::Error::Io("Failed to get post".to_owned()))
    }

    fn validate(title: &str, slug: &str, content: &str) -> Result<(), spin_sdk::sqlite::Error> {
        if title.len() == 0 || slug.len() == 0 || content.len() == 0 || !REGEX_SLUG.is_match(slug) {
            return Err(spin_sdk::sqlite::Error::Io("Failed to save post".to_owned()));
        }

        Ok(())
    }
}

#[cfg(feature = "ssr")]
impl From<spin_sdk::sqlite::Row<'_>> for Post {
    fn from(value: spin_sdk::sqlite::Row) -> Self {
        let parse_datetime = |value: &str| {
            time::PrimitiveDateTime::parse(value, &DATETIME_FORMAT)
                .map(|value| value.assume_utc())
                .ok()
        };
        let published_at = value.get::<&str>("published_at").and_then(|v| parse_datetime(v));
        let created_at = value.get::<&str>("created_at").and_then(|v| parse_datetime(v)).unwrap();
        let updated_at = value.get::<&str>("updated_at").and_then(|v| parse_datetime(v));
        Self {
            id: value.get("id").unwrap(),
            title: value.get::<&str>("title").unwrap().to_owned(),
            slug: value.get::<&str>("slug").unwrap().to_owned(),
            content: value.get::<&str>("content").unwrap().to_owned(),
            published_at,
            created_at,
            updated_at,
        }
    }
}
