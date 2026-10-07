use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum CurrentUserResponse {
    LoggedOut,
    User {
        name: String,
        id: i32
    }

}
#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub contents: String,
    pub complete: bool
}