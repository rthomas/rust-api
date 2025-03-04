use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Debug)]
pub struct CreateUserRequest {
    pub name: String,
    pub age: i32,
    pub gender: String,
}

#[derive(Deserialize)]
pub struct GetUserRequest {
    pub id: i32,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub gender: Option<String>,
}
