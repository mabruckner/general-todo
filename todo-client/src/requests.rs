use todo_shared::*;


fn set_origin(relative_url: &str) -> String {
    let origin = web_sys::window().unwrap().location().origin().unwrap();
    format!("{}{}", origin, relative_url).into()
}

pub async fn add_task(contents: String) -> reqwest::Result<Task> {
    reqwest::Client::new()
        .post(set_origin("/api/tasks"))
        .json(&contents)
        .send().await?
        .json().await
}
pub async fn get_all_tasks() -> reqwest::Result<Vec<Task>> {
    reqwest::Client::new()
        .get(set_origin("/api/tasks"))
        .send().await?
        .json().await
}
pub async fn get_current_user() -> reqwest::Result<CurrentUserResponse> {
    reqwest::get(set_origin("/api/user")).await?
        .json().await
}
pub async fn request_login(creds: Credentials) -> reqwest::Result<CurrentUserResponse> {
    reqwest::Client::new()
        .post(set_origin("/api/login"))
        .json(&creds)
        .send().await?
        .json().await
}
pub async fn request_register(creds: Credentials) -> reqwest::Result<CurrentUserResponse> {
    reqwest::Client::new()
        .post(set_origin("/api/register"))
        .json(&creds)
        .send().await?
        .json().await
}