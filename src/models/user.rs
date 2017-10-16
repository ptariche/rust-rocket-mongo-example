use std;
use std::io;
use bson;
use bson::oid::ObjectId;
use mongodb::ThreadedClient;
use mongodb::db::ThreadedDatabase;

use lib;

#[derive(Debug)]
pub struct Model {
  pub email: String,
  pub first_name: String,
  pub last_name: String
}

impl Model {
  pub fn to_bson(&self) -> bson::ordered::OrderedDocument {
    doc! { 
      "email": self.email.to_owned(),
      "first_name": self.first_name.to_owned(),
      "last_name": self.last_name.to_owned(),
    }
  }

  pub fn create(&self) -> Result<std::option::Option<bson::ordered::OrderedDocument>, io::Error> {
    let client = lib::mongo::establish_connection();
    let collection = client.db("test").collection("users");
    collection.insert_one(self.to_bson().clone(), None)
        .ok().expect("Failed to insert document.");
    
    let response_document = collection.find_one(Some(self.to_bson().clone()), None)
        .ok().expect("Failed to execute find.");
      
    Ok(response_document)
  }
}

  pub fn find_one(user_id: String) -> Result<std::option::Option<bson::ordered::OrderedDocument>, io::Error> {
    let client = lib::mongo::establish_connection();
    let collection = client.db("test").collection("users");

    let id = ObjectId::with_string(&user_id).unwrap();

    let response_document = collection.find_one(Some(doc! { "_id" => id }), None)
        .ok().expect("Failed to execute find.");
      
    Ok(response_document)
  }


