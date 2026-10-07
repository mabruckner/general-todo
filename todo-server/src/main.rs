#[macro_use] extern crate rocket;

mod db;
mod api;
use db::*;
use rocket::fs::FileServer;
use rocket_db_pools::Database;


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(TodoData::init())
        .attach(api::api_adhoc())
        .mount("/", FileServer::from("../todo-client/dist"))
}