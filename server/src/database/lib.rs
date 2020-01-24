#[macro_use]
extern crate diesel;

extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::{NewPingHistoryItem, PingHistoryItem};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
}

pub fn create_ping_history_item(
    conn: &SqliteConnection,
    url: &str,
    iteration: i64,
    result: i64,
) -> usize {
    use schema::ping_history;

    let new_history_item = NewPingHistoryItem {
        url,
        iteration,
        result,
    };

    diesel::insert_into(posts::table)
        .values(&new_history_item)
        .execute(conn)
        .expect("Error saving new post")
}
