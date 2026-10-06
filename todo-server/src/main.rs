#[macro_use] extern crate rocket;

mod db;
mod api;
use db::*;
use rocket_db_pools::Database;

#[get("/")]
async fn index(db: &TodoData) -> String {
    println!("{:?}", db.add_user("user".into(), "name".into()).await);
    format!("{:?}", db.all_users().await)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(TodoData::init())
        .attach(api::api_adhoc())
        .mount("/", routes![index])
}