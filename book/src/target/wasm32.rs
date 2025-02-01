use js_sys::Date;

/// Returns The Current Epoch Unix Timestamp
/// Number of seconds January 1, 1970 00:00:00 UTC.
pub(crate) fn get_timestamp() -> u64 {
    return (Date::now() / 1000.0) as u64;
}

pub(crate) fn uuid4() -> String {
    let crypto = web_sys::window()
        .expect("No window object")
        .crypto()
        .expect("Crypto no present");
    crypto.random_uuid()
}
