#[macro_use]
extern crate diesel;

extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::NewPingHistoryItem;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &SqliteConnection, url: &str, iteration: i32, result: i32) -> usize {
    use schema::ping_history;
    let new_item = NewPingHistoryItem{url, iteration, result};

    diesel::insert_into(posts::table)
        .values(&new_item)
        .execute(conn)
        .expect("Error saving new post")
}