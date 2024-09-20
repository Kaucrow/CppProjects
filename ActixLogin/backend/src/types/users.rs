use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub is_superuser: bool,
    pub thumbnail: Option<String>,
    pub date_joined: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct UserVisible {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub is_superuser: bool,
    pub thumbnail: Option<String>,
    pub date_joined: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}