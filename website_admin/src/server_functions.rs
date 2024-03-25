use leptos::*;

#[cfg(feature = "ssr")]
#[derive(serde::Deserialize, serde::Serialize)]
struct AdminSession {
    created_at: time::OffsetDateTime,
    expires_at: time::OffsetDateTime,
}

#[cfg(feature = "ssr")]
impl AdminSession {
    fn new(expires_at: time::OffsetDateTime) -> Self {
        Self {
            created_at: time::OffsetDateTime::now_utc(),
            expires_at,
        }
    }
}

#[cfg(feature = "ssr")]
fn admin_session_key_cookie_builder(value: &str) -> cookie::CookieBuilder {
    use cookie::{Cookie, SameSite};

    Cookie::build((crate::variables::admin_session_key_cookie_name(), value))
        .same_site(SameSite::Strict)
        .path("/admin")
        .http_only(true)
}

#[cfg(feature = "ssr")]
fn admin_session_store_key(key: &str) -> String {
    format!("admin_session_{key}")
}

#[cfg(feature = "ssr")]
fn admin_session_key_cookie() -> Option<cookie::Cookie<'static>> {
    use_context::<leptos_spin::RequestParts>()
        .and_then(|req| {
            req.headers()
                .iter()
                .find(|(key, _)| key == "cookie")
                .and_then(|(_, value)| String::from_utf8(value.to_vec()).ok())
        })
        .and_then(|value| {
            cookie::Cookie::split_parse(value).find(|cookie| {
                cookie
                    .clone()
                    .ok()
                    .map(|c| c.name() == crate::variables::admin_session_key_cookie_name())
                    .unwrap_or(false)
            })
        })
        .and_then(|value| value.ok())
}

#[cfg(feature = "ssr")]
pub fn delete_admin_session() -> Result<(), ServerFnError> {
    let key_cookie = admin_session_key_cookie().ok_or(ServerFnError::new("Cookie not found"))?;
    let admin_session_key = key_cookie.value();

    use_context::<leptos_spin::ResponseOptions>().map(|resp| {
        use time::{Duration, OffsetDateTime};

        resp.insert_header(
            "set-cookie",
            admin_session_key_cookie_builder("")
                .expires(OffsetDateTime::UNIX_EPOCH)
                .max_age(Duration::seconds(0))
                .build()
                .to_string()
                .as_bytes(),
        )
    });

    delete_admin_session_store(admin_session_key)
}

#[cfg(feature = "ssr")]
pub fn delete_admin_session_store(key: &str) -> Result<(), ServerFnError> {
    spin_sdk::key_value::Store::open_default()?
        .delete(&admin_session_store_key(key))
        .map_err(|err| ServerFnError::new(err))
}

#[server(prefix = "/admin/api")]
pub async fn is_admin_enabled() -> Result<bool, ServerFnError> {
    Ok(crate::variables::admin_enable())
}

#[server(prefix = "/admin/api")]
pub async fn is_authenticated() -> Result<bool, ServerFnError> {
    if !crate::variables::admin_enable() {
        return Ok(false);
    }

    let admin_session_key = admin_session_key_cookie();

    if admin_session_key.is_none() {
        return Ok(false);
    }

    let admin_session_key = admin_session_key.unwrap().value().trim().to_owned();

    if admin_session_key.len() != crate::variables::admin_session_key_length() {
        return Ok(false);
    }

    let admin_session = spin_sdk::key_value::Store::open_default()?
        .get_json::<AdminSession>(admin_session_store_key(&admin_session_key))
        .map_err(|err| ServerFnError::new(err))?;

    if let Some(admin_session) = admin_session {
        use time::OffsetDateTime;

        let is_not_expired = admin_session.expires_at > OffsetDateTime::now_utc();
        let renew_interval = crate::variables::admin_session_renew_interval();

        if is_not_expired
            && renew_interval.is_positive()
            && admin_session.created_at + renew_interval <= OffsetDateTime::now_utc()
        {
            let _ = delete_admin_session_store(&admin_session_key);
            let _ = start_admin_session();
        }

        return Ok(is_not_expired);
    }

    Ok(false)
}

#[cfg(feature = "ssr")]
pub fn start_admin_session() -> Result<(), ServerFnError> {
    use crate::variables::{admin_session_duration, admin_session_key_length};
    use rand::{thread_rng, Rng};
    use time::OffsetDateTime;

    let admin_session_key = thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(admin_session_key_length())
        .map(char::from)
        .collect::<String>();
    let expires_at = OffsetDateTime::now_utc() + admin_session_duration();

    spin_sdk::key_value::Store::open_default()?
        .set_json(
            &admin_session_store_key(&admin_session_key),
            &AdminSession::new(expires_at),
        )
        .map_err(|err| ServerFnError::new(err))?;

    use_context::<leptos_spin::ResponseOptions>().map(|resp| {
        resp.insert_header(
            "set-cookie",
            admin_session_key_cookie_builder(&admin_session_key)
                .expires(expires_at)
                .http_only(true)
                .to_string()
                .as_bytes(),
        )
    });

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn require_authentication() -> Result<(), ServerFnError> {
    if !is_authenticated().await? {
        leptos_spin::redirect("/");
    }

    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn require_no_authentication() -> Result<(), ServerFnError> {
    if !is_admin_enabled().await? {
        leptos_spin::redirect("/");
    }

    if is_authenticated().await? {
        leptos_spin::redirect("/admin");
    }

    Ok(())
}
