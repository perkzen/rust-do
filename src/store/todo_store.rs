use sqlx::SqliteConnection;
use std::error::Error;
use sqlx::Row;
use sqlx::types::chrono::NaiveDateTime;
use crate::store::storage::Storage;
use crate::store::todo::{Todo, TodoCreate};


pub(crate) struct TodoStore {
    pub(crate) db: SqliteConnection,
}


impl TodoStore {
    pub fn new(db: SqliteConnection) -> TodoStore {
        TodoStore { db }
    }
}

impl Storage<Todo, TodoCreate> for TodoStore {
    async fn add(&mut self, item: TodoCreate) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO todos (title) VALUES ($1)";
        sqlx::query(query)
            .bind(item.title)
            .execute(&mut self.db).await?;
        Ok(())
    }

    async fn list(&mut self) -> Result<Vec<Todo>, Box<dyn Error>> {
        let query = "SELECT id, title, created_at, completed FROM todos";
        let todos = sqlx::query(query)
            .fetch_all(&mut self.db)
            .await?;

        let mut result = Vec::new();
        for row in todos {
            let title: String = row.try_get("title")?;
            let completed: bool = row.try_get("completed")?;
            let created_at: NaiveDateTime = row.try_get("created_at")?;

            let todo = Todo { title, completed, created_at };
            result.push(todo);
        }

        Ok(result)
    }
}

