use std::fmt::Display;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Clone)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
}

pub struct TodoCreate {
    pub title: String,
}


impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let completed = if self.completed { "X" } else { " " };
        write!(f, "{}", format!("{} - {}", self.title, self.created_at))
    }
}
