use std::fmt::Display;
use sqlx::types::chrono::NaiveDateTime;
use crate::traits::has_id::HasId;

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
}

pub struct TodoCreate {
    pub title: String,
}

pub struct TodoUpdate {
    pub id: i32,
    pub completed: bool,
}


impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("ID: {} {} - {}", self.id, self.title, self.created_at))
    }
}

impl HasId for Todo {
    fn get_id(&self) -> i32 {
        self.id
    }
}
