use rocket::{Request, data, fairing::AdHoc, http::{Cookie, CookieJar, Status, StatusClass::Success}, outcome::Outcome, request::{self, FromRequest}, response::status::BadRequest, serde::{Deserialize, Serialize, json::Json}};
use todo_shared::*;

use crate::db::{TodoData};

pub struct HasUser(crate::db::User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for HasUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        
        let db_outcome = request.guard::<&TodoData>().await;
        let jar_outcome = request.guard::<&CookieJar>().await;
        match (db_outcome, jar_outcome) {
            (Outcome::Success(database), Outcome::Success(jar)) => {
                let uid = jar.get_private("user_id").map(|cookie| cookie.value().parse::<i32>().ok()).flatten();
                if let Some(uid) = uid {
                    if let Some(user) = database.get_user_from_id(uid).await {
                        Outcome::Success(HasUser(user))
                    } else {
                        Outcome::Error((Status::BadRequest, ()))
                    }
                } else {
                    Outcome::Error((Status::Unauthorized, ()))
                }
            },
            _ => Outcome::Error((Status::InternalServerError, ()))
        }
    }
}

pub fn api_adhoc() -> AdHoc {
    AdHoc::on_ignite("api", |rocket| async move {
        rocket.mount("/api", routes![login, register, logout, current_user, get_all_tasks, add_task])
    })
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

#[get("/tasks")]
async fn get_all_tasks(user: HasUser, database: &TodoData) -> Result<Json<Vec<Task>>, ()> {
    let task_list = database.get_all_tasks(user.0.id).await.map_err(|_|())?;
    Ok(Json(task_list.into_iter().map(|t| Task::from(t)).collect()))
}

#[post("/tasks", data="<task_contents>")]
async fn add_task(user: HasUser, database: &TodoData, task_contents: Json<String>) -> Result<Json<Task>, ()> {
    Ok(Json(database.add_task(user.0.id, task_contents.0.clone()).await.map_err(|_|())?.into()))
}