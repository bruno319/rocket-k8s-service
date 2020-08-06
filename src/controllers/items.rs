use rocket::*;
use std::ops::{Deref, DerefMut};

use r2d2;
use r2d2_redis::redis::Commands;
use r2d2_redis::RedisConnectionManager;
use rocket::http::RawStr;

use db::redis::RedisConnection;

const DB_KEY: &'static str = "items";

impl Deref for RedisConnection {
    type Target = r2d2::PooledConnection<RedisConnectionManager>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RedisConnection {
    fn deref_mut(&mut self) -> &mut r2d2::PooledConnection<RedisConnectionManager> {
        &mut self.0
    }
}


#[post("/<item>")]
pub fn create(item: &RawStr, mut connection: RedisConnection) -> String {
    let _: () = connection.lpush(DB_KEY, item.as_str()).unwrap();

    format!("OK")
}

#[get("/")]
pub fn index(mut connection: RedisConnection) -> String {
    let items: Vec<String> = connection.lrange(DB_KEY, 0, -1).unwrap();

    items.join(", ")
}
