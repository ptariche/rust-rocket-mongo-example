#![feature(plugin, custom_attribute, decl_macro, attr_literals)]
#![plugin(rocket_codegen)]

#[macro_use(bson, doc)] extern crate bson;

extern crate mongodb;
extern crate time;
extern crate uuid;
extern crate regex;
extern crate rocket;
extern crate serde;
extern crate rand;
extern crate crypto;
extern crate serde_json;
extern crate rustc_serialize;
extern crate jsonwebtoken as jwt;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::fairing::{AdHoc};

mod lib;
mod meta;
mod middleware;
mod controllers;
mod models;

fn main() {
  rocket::ignite().mount("/", routes![
                                controllers::status::lookup,
                                controllers::user::create,
                                controllers::user::get,
                                ])
            .attach(AdHoc::on_request(|req, _res| {
              match middleware::authentication::validate() {
                true => "valid",
                false => "invalid"
              };
              println!(" => Incoming request: {:?}", req);
            }))
            .catch(catchers![controllers::not_found::lookup])
            .launch();
}

