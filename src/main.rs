mod routs;

#[macro_use] extern crate rocket;
// use rocket::get;
// use rocket::launch;
// use rocket::routes;
use rocket::serde::json::{json, Json, Value};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::get("/foo")]
fn foo() {
}

#[post("/")]
fn bar() {

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}


// fn main() {
//     println!("Hello, world!");
// }
