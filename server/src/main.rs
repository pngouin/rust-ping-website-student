/* 
    Reference: https://leonardoce.github.io/2018-03-15/rocket-tutorial-3
*/

#![feature(proc_macro_hygiene, decl_macro)]

extern crate reqwest;
extern crate stopwatch;
#[macro_use]
extern crate rocket;

mod domain_ping_response_average;

use crate::domain_ping_response_average::ping_domain;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

fn main() {
    rocket::ignite()
        .mount("/", routes![index, average_request_time])
        .attach(Template::fairing())
        .launch();
}

#[get("/")]
fn index() -> Template {
    let context = HashMap::<String, String>::new();

    return Template::render("domain_ping_response_average/form", context);
}

#[get("/average?<domain>&<iterations>")]
fn average_request_time(domain: String, iterations: i64) -> Template {
    let ping_average = ping_domain(&*domain, iterations);

    let mut context = HashMap::<String, String>::new();
    context.insert("domain".to_string(), domain.to_string());
    context.insert("iterations".to_string(), iterations.to_string());
    context.insert("result".to_string(), ping_average.to_string());

    return Template::render("domain_ping_response_average/result", context);
}
