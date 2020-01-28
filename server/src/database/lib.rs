use diesel::prelude::*;
use std::env;

use super::models::NewPingHistoryItem;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_ping_history_item<'a>(
    conn: &SqliteConnection,
    url: &'a str,
    iteration: &'a i32,
    ping: &'a i32,
) -> usize {
    use super::schema::ping_history;

    let new_history_item = NewPingHistoryItem {
        url,
        iteration,
        ping,
    };

    diesel::insert_into(ping_history::table)
        .values(&new_history_item)
        .execute(conn)
        .expect("Error saving new post")
}
