use sqlx::SqliteConnection;
use std::error::Error;
use sqlx::Row;
use sqlx::types::chrono::NaiveDateTime;


pub struct Todo {
    pub title: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
}


pub(crate) struct TodoStore {
    db: SqliteConnection,
}


impl TodoStore {
    pub fn new(db: SqliteConnection) -> TodoStore {
        TodoStore {
            db
        }
    }

    pub async fn add(&mut self, title: String) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO todos (title) VALUES ($1)";
        sqlx::query(query)
            .bind(title)
            .execute(&mut self.db).await?;
        Ok(())
    }

    pub async fn list(&mut self) -> Result<Vec<Todo>, Box<dyn Error>> {
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