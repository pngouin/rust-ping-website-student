extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;
extern crate stopwatch;

use rocket::request::Form;
use rocket_contrib::json::Json;
use stopwatch::Stopwatch;

#[derive(FromForm)]
pub struct PingRequest {
    domain: String,
    iterations: i64,
}

#[derive(Serialize)]
pub struct PingResponse {
    ping: i64,
}

#[post("/ping", format="application/x-www-form-urlencoded", data = "<user_input>")]
pub fn ping_endpoint(user_input: Form<PingRequest>) -> i64{
    let result = ping_domain(user_input.domain.as_str(), user_input.iterations);
    let response = PingResponse { ping: result };
    return response;
}

pub fn ping_domain(domain: &str, iterations: i64) -> i64 {
    let mut accumulator = 0;
    for _ in 0..iterations {
        accumulator += ping_domain_once(domain);
    }

    return accumulator / iterations;
}

pub fn ping_domain_once(domain: &str) -> i64 {
    let sw = Stopwatch::start_new();
    let _ = reqwest::blocking::get(domain);

    return sw.elapsed_ms();
}
