#![feature(proc_macro_hygiene, decl_macro)]

extern crate r2d2;
extern crate r2d2_redis;
#[macro_use]
extern crate rocket;

use controllers::items;
use db::redis::pool;
use rocket::config::{ConfigBuilder, Environment};
use rocket::Config;

mod db;
mod controllers;

fn main() {
    let config = Config::build(Environment::Staging)
        .address("0.0.0.0")
        .finalize();
    rocket::custom(config.unwrap())
        .mount("/", routes![items::create, items::index])
        .manage(pool())
        .launch();
}