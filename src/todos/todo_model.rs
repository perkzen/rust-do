use std::fmt::Display;
use sqlx::types::chrono::NaiveDateTime;

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
        write!(f, "{}", format!("{} - {}", self.title, self.created_at))
    }
}

pub trait HasId {
    fn get_id(&self) -> i32;
}

impl HasId for Todo {
    fn get_id(&self) -> i32 {
        self.id
    }
}
