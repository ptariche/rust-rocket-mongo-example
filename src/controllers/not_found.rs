use rocket_contrib::{JsonValue};

#[catch(404)]
fn lookup() -> JsonValue {
  json!({
    "success": false,
    "code": 404,
    "data": "",
    "error": "Resource not found"
  })
}

