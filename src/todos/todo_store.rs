use std::error::Error;
use sqlx::Row;
use sqlx::types::chrono::NaiveDateTime;
use crate::todos::todo_model::{Todo, TodoCreate};

pub trait Storage<TData, TDataCreate> {
    async fn add(&mut self, item: TDataCreate) -> Result<(), Box<dyn Error>>;
    async fn list(&mut self) -> Result<Vec<TData>, Box<dyn Error>>;
}

pub(crate) struct TodoStore {
    pub(crate) db: sqlx::SqlitePool,
}

impl TodoStore {
    pub fn new(db: sqlx::SqlitePool) -> TodoStore {
        TodoStore { db }
    }
}

impl Storage<Todo, TodoCreate> for TodoStore {
    async fn add(&mut self, item: TodoCreate) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO todos (title) VALUES ($1)";
        sqlx::query(query)
            .bind(item.title)
            .execute(&self.db).await?;
        Ok(())
    }

    async fn list(&mut self) -> Result<Vec<Todo>, Box<dyn Error>> {
        let query = "SELECT id, title, created_at, completed FROM todos";
        let todos = sqlx::query(query)
            .fetch_all(&self.db)
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

#[cfg(test)]
mod tests {
    use crate::db::connection::get_database_connection_pool;
    use crate::todos::todo_model::TodoCreate;
    use crate::todos::todo_store::Storage;

    #[tokio::test]
    async fn test_add_todo() {
        let db = get_database_connection_pool("sqlite::memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&db).await.unwrap();

        let mut todo_store = super::TodoStore::new(db);

        let todo = TodoCreate { title: "test".to_string() };
        todo_store.add(todo).await.unwrap();

        let todos = todo_store.list().await.unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].title, "test")
    }
}