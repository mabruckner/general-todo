use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("todo_database")]
pub struct TodoData(sqlx::PgPool);

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pass_hash: String,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub contents: String,
    pub complete: bool
}

impl From<Task> for todo_shared::Task {
    fn from(value: Task) -> Self {
        todo_shared::Task {
            id: value.id,
            contents: value.contents,
            complete: value.complete
        }
    }
}

/// password hashing info from https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
pub fn hash_password(pass: &str) -> Option<String> {

    let argon = Argon2::new(
        Algorithm::Argon2i,
        Version::V0x13,
        Params::default()
    );
    Some(argon.hash_password(pass.as_bytes()).ok()?.to_string())
}

impl User {
    pub fn check_password(&self, pass: &str) -> bool {
        let Ok(parsed_hash) = PasswordHash::new(&self.pass_hash) else { return false };
        Argon2::default().verify_password(pass.as_bytes(), &parsed_hash).is_ok()
    }
}
impl TodoData {
    pub async fn get_user_from_id(&self, id: i32) -> Option<User> {
        let user: User = sqlx::query_as("SELECT id, username, pass_hash FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.0).await.ok()?;
        Some(user)
    }

    pub async fn get_user_from_name(&self, name: &str) -> Option<User> {
        let user: User = sqlx::query_as("SELECT id, username, pass_hash FROM users WHERE username = $1")
            .bind(name)
            .fetch_one(&self.0).await.ok()?;
        Some(user)
    }

    pub async fn add_user(&self, name: &str, password: &str) -> Result<User, sqlx::Error> {
        let pass_hash = hash_password(password).unwrap();
        sqlx::query_as("INSERT INTO users (username, pass_hash) VALUES ($1, $2) RETURNING *")
            .bind(name)
            .bind(pass_hash)
            .fetch_one(&self.0).await
    }

    pub async fn all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM users")
            .fetch_all(&self.0).await
    }

    pub async fn add_task(&self, uid: i32, contents: String) -> Result<Task, sqlx::Error> {
        sqlx::query_as("INSERT INTO tasks (user_id, contents, complete) VALUES ($1, $2, false) RETURNING *")
            .bind(uid)
            .bind(contents)
            .fetch_one(&self.0).await
    }

    pub async fn get_all_tasks(&self, uid: i32) -> Result<Vec<Task>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM tasks WHERE user_id = $1")
            .bind(uid)
            .fetch_all(&self.0).await
    }

}