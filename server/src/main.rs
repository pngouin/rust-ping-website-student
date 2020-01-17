/*
    Reference: https://leonardoce.github.io/2018-03-15/rocket-tutorial-3
*/

#![feature(proc_macro_hygiene, decl_macro)]

extern crate reqwest;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod ping;

fn rocket() -> rocket::Rocket {
    return rocket::ignite().mount("/", routes![crate::ping::ping_endpoint]);
}

fn main() {
    rocket().launch();
}
