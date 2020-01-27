use super::schema::ping_history;

#[derive(Queryable)]
pub struct PingHistoryItem {
    pub id: i32,
    pub url: String,
    pub iteration: i32,
    pub ping: i32,
}

#[derive(Insertable)]
#[table_name = "ping_history"]
pub struct NewPingHistoryItem<'a> {
    pub url : &'a str,
    pub iteration: &'a i32,
    pub ping: &'a i32,
}