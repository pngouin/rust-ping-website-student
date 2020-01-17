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

#[post("/ping", data = "<task>")]
pub fn ping_endpoint(task: Form<PingRequest>) -> Json<PingResponse> {
    let result = ping_domain(task.domain.as_str(), task.iterations);
    let response = Json(PingResponse { ping: result });
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
