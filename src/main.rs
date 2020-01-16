#![feature(proc_macro_hygiene, decl_macro)]

extern crate reqwest;
extern crate stopwatch;
#[macro_use] extern crate rocket;

use stopwatch::{Stopwatch};

fn main() {
    rocket::ignite().attach(Template::fairing()).mount("/", routes![index, average_request_time]).launch();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn chrono(domain: &str) -> i64 {
    let sw = Stopwatch::start_new();
    let _ = reqwest::blocking::get(domain);
    return sw.elapsed_ms();
}

#[get("/average/<domain>/<number>")]
fn average_request_time(domain: String, number: i64) -> String {
    let mut accumullator = 0;
    for _ in 0..number {
        accumullator += chrono(&*domain)
    }
    return format!("{}ms", accumullator/number)
}

// https://leonardoce.github.io/2018-03-15/rocket-tutorial-3