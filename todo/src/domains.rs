use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Clone, Validate)]
pub struct User {
    pub id: u32,
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Clone)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub deadline: String,
    pub completed: bool,
}
