mod routs;

#[macro_use] extern crate rocket;

use rocket::serde::de::Unexpected::Str;
// use rocket::get;
// use rocket::launch;
// use rocket::routes;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::json::serde_json::error::Category::Data;

struct UserData {
    name: String,
    creditCard: String,
    id: String,
    age: u32,
}

fn getData() -> UserData {
    let data = UserData {
      name: String::from(""),
        creditCard: String::from(""),
        id: String::from(""),
        age: 1
    };

    return data;
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::get("/foo")]
fn foo() {
}

#[post("/")]
fn bar() {
    let d = getData();
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}


// fn main() {
//     println!("Hello, world!");
// }
