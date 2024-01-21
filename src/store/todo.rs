use sqlx::types::chrono::NaiveDateTime;

pub struct Todo {
    pub title: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
}
pub struct TodoCreate {
    pub title: String,
}

