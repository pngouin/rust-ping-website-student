/*
    Reference: https://leonardoce.github.io/2018-03-15/rocket-tutorial-3
*/

#![feature(proc_macro_hygiene, decl_macro)]

extern crate reqwest;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod ping_utility;

use crate::ping_utility::ping_domain;
use rocket::request::Form;
use rocket_contrib::json::Json;

fn main() {
    rocket::ignite().mount("/", routes![ping]).launch();
}

#[derive(FromForm)]
struct PingRequest {
    domain: String,
    iterations: i64,
}

#[derive(Serialize)]
struct PingResponse {
    ping: i64,
}

#[post("/ping", data = "<task>")]
fn ping(task: Form<PingRequest>) -> Json<PingResponse> {
    let result = ping_domain(task.domain.as_str(), task.iterations);
    let response = Json(PingResponse { ping: result });
    return response;
}
