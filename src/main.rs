/* 
    Reference: https://leonardoce.github.io/2018-03-15/rocket-tutorial-3
*/

#![feature(proc_macro_hygiene, decl_macro)]

extern crate reqwest;
extern crate stopwatch;
#[macro_use]
extern crate rocket;

use rocket_contrib::templates::Template;
use std::collections::HashMap;
use stopwatch::Stopwatch;

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

fn chrono(domain: &str) -> i64 {
    let sw = Stopwatch::start_new();
    let _ = reqwest::blocking::get(domain);

    return sw.elapsed_ms();
}

#[get("/average?<domain>&<iterations>")]
fn average_request_time(domain: String, iterations: i64) -> Template {
    let mut accumulator = 0;
    for _ in 0..iterations {
        accumulator += chrono(&*domain);
    }

    let mut context = HashMap::<String, String>::new();
    context.insert("domain".to_string(), (domain).to_string());
    context.insert("iterations".to_string(), (iterations).to_string());
    context.insert("result".to_string(), (accumulator / iterations).to_string());

    return Template::render("domain_ping_response_average/result", context);
}
