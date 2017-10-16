use bson;
use rocket_contrib::{Json, JsonValue};

use meta;
use models;

type ID = String;

#[post("/user", format = "application/json", data="<user>")]
pub fn create(user: Json<meta::user::Post>) -> JsonValue {
  let model = models::user::Model {
    email: user.email.to_owned(),
    first_name: user.first_name.to_owned(),
    last_name: user.last_name.to_owned()
  };

  let document = model.create().unwrap();
  let result = bson::from_bson::<meta::user::PostResponse>(bson::Bson::Document(document.unwrap()));

  match result {
    Ok(user) => {
      json!({
        "code": 201,
        "success": true,
        "data": user,
        "error": ""
      })
    },
    Err (_e) => {
      json!({
        "code": 412,
        "success": false,
        "data": {},
        "error": "An error has occured"
      })
    }
  }

}

#[get("/user/<id>", format = "application/json")]
pub fn get(id: ID) -> JsonValue {
  let document = models::user::find_one(id.to_owned()).unwrap();
  let result = bson::from_bson::<meta::user::GetResponse>(bson::Bson::Document(document.unwrap()));

  match result {
    Ok(user) => {
      json!({
        "code": 200,
        "success": true,
        "data": user,
        "error": ""
      })
    },
    Err (_e) => {
      json!({
        "code": 400,
        "success": false,
        "data": {},
        "error": "An error has occured"
      })
    }
  }

}