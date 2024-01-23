use std::error::Error;
use sqlx::Row;
use sqlx::types::chrono::NaiveDateTime;
use crate::todos::todo_model::{Todo, TodoCreate, TodoUpdate};
use crate::traits::storage::Storage;

pub(crate) struct TodoStore {
    pub(crate) db: sqlx::SqlitePool,
}

impl TodoStore {
    pub fn new(db: sqlx::SqlitePool) -> TodoStore {
        TodoStore { db }
    }
}

impl Storage<Todo, TodoCreate, TodoUpdate> for TodoStore {
    async fn find_all(&mut self) -> Result<Vec<Todo>, Box<dyn Error>> {
        let query = "SELECT id, title, created_at, completed FROM todos";
        let todos = sqlx::query(query)
            .fetch_all(&self.db)
            .await?;

        let mut result = Vec::new();
        for row in todos {
            let id: i32 = row.try_get("id")?;
            let title: String = row.try_get("title")?;
            let completed: bool = row.try_get("completed")?;
            let created_at: NaiveDateTime = row.try_get("created_at")?;

            let todo = Todo { id, title, completed, created_at };
            result.push(todo);
        }

        Ok(result)
    }

    async fn create(&mut self, item: TodoCreate) -> Result<(), Box<dyn Error>> {
        let query = "INSERT INTO todos (title) VALUES ($1)";
        sqlx::query(query)
            .bind(item.title)
            .execute(&self.db).await?;
        Ok(())
    }

    async fn update(&mut self, item: TodoUpdate) -> Result<(), Box<dyn Error>> {
        let query = "UPDATE todos SET completed = $1 WHERE id = $2";
        sqlx::query(query)
            .bind(item.completed)
            .bind(item.id)
            .execute(&self.db).await?;
        Ok(())
    }

    async fn delete_one(&mut self, id: &i32) -> Result<(), Box<dyn Error>> {
        let query = "DELETE FROM todos WHERE id = $1";
        sqlx::query(query)
            .bind(id)
            .execute(&self.db).await?;
        Ok(())
    }

    async fn delete_all(&mut self) -> Result<(), Box<dyn Error>> {
        let query = "DELETE FROM todos";
        sqlx::query(query)
            .execute(&self.db).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::db::connection::get_database_connection_pool;
    use crate::todos::todo_model::{TodoCreate, TodoUpdate};
    use crate::todos::todo_store::{Storage, TodoStore};

    async fn setup_todo_store() -> TodoStore {
        let db = get_database_connection_pool("sqlite::memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&db).await.unwrap();
        TodoStore::new(db)
    }

    #[tokio::test]
    async fn test_add_and_list_todo() {
        let mut todo_store = setup_todo_store().await;

        let todo = TodoCreate { title: "test".to_string() };
        todo_store.create(todo).await.unwrap();

        let todos = todo_store.find_all().await.unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].title, "test")
    }

    #[tokio::test]
    async fn test_update_todo() {
        let mut todo_store = setup_todo_store().await;

        let todo = TodoCreate { title: "test".to_string() };
        todo_store.create(todo).await.unwrap();

        let todos = todo_store.find_all().await.unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].title, "test");
        assert_eq!(todos[0].completed, false);

        let todo = todos[0].clone();
        let todo_update = TodoUpdate { id: todo.id, completed: true };
        todo_store.update(todo_update).await.unwrap();

        let todos = todo_store.find_all().await.unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].title, "test");
        assert_eq!(todos[0].completed, true);
    }

    #[tokio::test]
    async fn test_delete_todo() {
        let mut todo_store = setup_todo_store().await;

        let todo = TodoCreate { title: "test".to_string() };
        todo_store.create(todo).await.unwrap();

        let todos = todo_store.find_all().await.unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].title, "test");

        let todo = todos[0].clone();
        todo_store.delete_one(&todo.id).await.unwrap();

        let todos = todo_store.find_all().await.unwrap();
        assert_eq!(todos.len(), 0);
    }

    #[tokio::test]
    async fn test_delete_all_todos() {
        let mut todo_store = setup_todo_store().await;

        let todo = TodoCreate { title: "test".to_string() };
        todo_store.create(todo).await.unwrap();

        let todos = todo_store.find_all().await.unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].title, "test");

        todo_store.delete_all().await.unwrap();

        let todos = todo_store.find_all().await.unwrap();
        assert_eq!(todos.len(), 0);
    }
}