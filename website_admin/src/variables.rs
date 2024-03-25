use std::fmt::Debug;
use std::str::FromStr;

fn variable<T: FromStr>(key: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    spin_sdk::variables::get(key)
        .expect("Could not get variable")
        .parse::<T>()
        .expect("Invalid variable value")
}

pub(crate) fn admin_enable() -> bool {
    variable("enable")
}

pub(crate) fn admin_password() -> String {
    variable("password")
}

pub(crate) fn admin_username() -> String {
    variable("username")
}

pub(crate) fn admin_session_duration() -> time::Duration {
    time::Duration::days(variable("session_duration"))
}

pub(crate) fn admin_session_key_cookie_name() -> String {
    variable("session_key_cookie_name")
}

pub(crate) fn admin_session_key_length() -> usize {
    variable("session_key_length")
}

pub(crate) fn admin_session_renew_interval() -> time::Duration {
    time::Duration::days(variable("session_renew_interval"))
}
