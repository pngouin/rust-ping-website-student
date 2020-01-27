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
    iterations: i32,
}

#[derive(Serialize)]
pub struct PingResponse {
    ping: i32,
}

#[post("/ping", format="application/x-www-form-urlencoded", data = "<user_input>")]
pub fn ping_endpoint(user_input: Form<PingRequest>) -> Json<PingResponse>{
    let result = ping_domain(user_input.domain.as_str(), user_input.iterations);

    // TODO: Insert in database

    Json(PingResponse { ping: result })
}

fn ping_domain(domain: &str, iterations: i32) -> i32 {
    let mut accumulator = 0;
    for _ in 0..iterations {
        accumulator += ping_domain_once(domain);
    }

    accumulator / iterations
}

fn ping_domain_once(domain: &str) -> i32 {
    let sw = Stopwatch::start_new();
    let _ = reqwest::blocking::get(domain);

    sw.elapsed_ms() as i32
}
