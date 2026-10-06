use rocket::{data, fairing::AdHoc, http::{Cookie, CookieJar}, serde::{Deserialize, Serialize, json::Json}};

use crate::db::{TodoData};

pub fn api_adhoc() -> AdHoc {
    AdHoc::on_ignite("api", |rocket| async move {
        rocket.mount("api", routes![login, register, logout, current_user])
    })
}

#[derive(Serialize, Deserialize)]
enum CurrentUserResponse {
    LoggedOut,
    User {
        name: String,
        id: i32
    }

}
#[derive(Serialize, Deserialize)]
struct Credentials {
    pub username: String,
    pub password: String
}
#[post("/login", data="<user_info>")]
async fn login(cookies: &CookieJar<'_>, user_info: Json<Credentials>, database: &TodoData) -> Json<CurrentUserResponse> {
    let Some(user) = database.get_user_from_name(&user_info.username).await else {
        cookies.remove_private("user_id");
        return Json(CurrentUserResponse::LoggedOut);
    };
    if user.check_password(&user_info.password) {
        cookies.add_private(Cookie::new("user_id", format!("{}", user.id)));
        Json(CurrentUserResponse::User { name: user.username, id: user.id })
    } else {
        cookies.remove_private("user_id");
        Json(CurrentUserResponse::LoggedOut)
    }
}

#[post("/register", data="<user_info>")]
async fn register(cookies: &CookieJar<'_>, user_info: Json<Credentials>, database: &TodoData) -> Json<CurrentUserResponse> {
    let Ok(user) = database.add_user(&user_info.username, &user_info.password).await else {
        return Json(CurrentUserResponse::LoggedOut);
    };
    cookies.add_private(Cookie::new("user_id", format!("{}", user.id)));
    Json(CurrentUserResponse::User { name: user.username, id: user.id })
}

#[post("/logout")]
async fn logout(cookies: &CookieJar<'_>) -> Json<CurrentUserResponse> {
    cookies.remove_private("user_id");
    Json(CurrentUserResponse::LoggedOut)
}

#[get("/user")]
async fn current_user(cookies: &CookieJar<'_>, database: &TodoData) -> Json<CurrentUserResponse> {
    let Some(id) = cookies.get_private("user_id").map(|cookie| cookie.value().parse::<i32>().ok()).flatten() else {
        return Json(CurrentUserResponse::LoggedOut)
    };
    let Some(user) = database.get_user_from_id(id).await else {
        return Json(CurrentUserResponse::LoggedOut)
    };
    Json(CurrentUserResponse::User{name: user.username, id: id})
}
