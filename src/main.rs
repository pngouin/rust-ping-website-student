/* 
    Reference: https://leonardoce.github.io/2018-03-15/rocket-tutorial-3
*/
#![feature(proc_macro_hygiene, decl_macro)]

extern crate reqwest;
extern crate stopwatch;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod domain_ping_response_average;
mod schema; 
mod sql;


use crate::domain_ping_response_average::ping_domain;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

use crate::models::NewPingHistoryItem;

thread_local!(static ODB: SqliteConnection = sql::establish_connection() );
//thread_local!(static ODB: RefCell<sqlite::database::Database> = RefCell::new(sqlite::open("test.db"));

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

    use schema::ping_history; 

    let ping_average = ping_domain(&*domain, iterations);
    
    let mut context = HashMap::<String, String>::new();
    context.insert("domain".to_string(), domain.to_string());
    context.insert("iterations".to_string(), iterations.to_string());
    context.insert("result".to_string(), ping_average.to_string());

    sql::create_post(ODB, &*domain, iterations as i32, ping_average as i32);

    return Template::render("domain_ping_response_average/result", context);
}
