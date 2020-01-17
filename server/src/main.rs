/*
    Reference: https://leonardoce.github.io/2018-03-15/rocket-tutorial-3
*/

#![feature(proc_macro_hygiene, decl_macro)]

extern crate reqwest;
#[macro_use]
extern crate rocket;

mod ping_utility;

use crate::ping_utility::ping_domain;
use rocket::request::Form;

fn main() {
    rocket::ignite().mount("/", routes![ping]).launch();
}

#[derive(FromForm)]
pub struct PingTask {
    domain: String,
    iterations: i64,
}

#[post("/ping", data = "<task>")]
fn ping(task: Form<PingTask>) -> String {
    let result = ping_domain(task.domain.as_str(), task.iterations);
    return result.to_string();
}
