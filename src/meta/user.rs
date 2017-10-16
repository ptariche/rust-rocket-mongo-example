use bson; 

#[derive(Serialize, Deserialize, Debug)]
pub struct PostResponse {
  pub _id: bson::oid::ObjectId,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
  pub email: String,
  pub first_name: String,
  pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetResponse {
  pub _id: bson::oid::ObjectId,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
}
