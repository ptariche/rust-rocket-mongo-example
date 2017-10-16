use rocket_contrib::{JsonValue};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[get("/status")]
fn lookup() -> JsonValue {
  json!({
    "code": 200,
    "success": true,
    "data": {
      "status": "OK",
      "version": VERSION.to_string()
    },
    "error": ""
  })
}
