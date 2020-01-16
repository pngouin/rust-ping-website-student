#![feature(proc_macro_hygiene, decl_macro)]

extern crate reqwest;
extern crate stopwatch;
#[macro_use] extern crate rocket;

use stopwatch::{Stopwatch};
use std::collections::HashMap;
use rocket_contrib::templates::Template;

fn main() {
    rocket::ignite()
        .mount("/", routes![index, average_request_time])
        .attach(Template::fairing())
        .launch();
}

#[get("/")]
fn index() -> Template {
    let context = HashMap::<String, String>::new();
    return Template::render("index", context);
}

fn chrono(domain: &str) -> i64 {
    let sw = Stopwatch::start_new();
    let _ = reqwest::blocking::get(domain);
    return sw.elapsed_ms();
}

#[get("/average?<domain>&<iterations>")]
fn average_request_time(domain: String, iterations: i64) -> String {
    let mut accumulator = 0;
    for _ in 0..iterations {
        accumulator += chrono(&*domain);
    }
    return format!("{}ms", accumulator/iterations);
}

// https://leonardoce.github.io/2018-03-15/rocket-tutorial-3