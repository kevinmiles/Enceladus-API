use serde_json::json;

#[inline]
#[rocket::get("/")]
pub fn meta() -> String {
    json!({
        "version": env!("CARGO_PKG_VERSION"),
        "version_major": env!("CARGO_PKG_VERSION_MAJOR"),
        "repository": env!("CARGO_PKG_REPOSITORY"),
    })
    .to_string()
}
