mod commands;
mod todos;
mod db;
mod ui;
mod traits;

use clap::{Command};
use commands::todo_cmd::{add_todo, list_todos};
use todos::todo_store::{TodoStore};
use todos::todo_model::{TodoCreate};
use ui::select::Select;
use crate::commands::todo_cmd::{clear_todos, delete_todo};
use crate::db::connection::get_database_connection_pool;
use crate::todos::todo_model::{Todo, TodoUpdate};
use crate::traits::storage::Storage;


#[tokio::main]
async fn main() {
    let connection_url = "sqlite:rustdo.db";
    let db = get_database_connection_pool(connection_url).await.unwrap_or_else(|err| {
        eprintln!("Could not connect to database: {}", err);
        std::process::exit(1);
    });

    sqlx::migrate!("./migrations").run(&db).await.unwrap();


    let mut todo_store = TodoStore::new(db);


    let cmd = Command::new("rd")
        .about("Rust do is a todo list manager written in Rust")
        .subcommand_required(true)
        .subcommand(list_todos())
        .subcommand(add_todo())
        .subcommand(delete_todo())
        .subcommand(clear_todos())
        .get_matches();

    match cmd.subcommand() {
        Some(("list", _)) => {
            let todos = todo_store.find_all().await.unwrap();

            let completed_todos: Vec<Todo> = todos
                .iter()
                .filter(|todo| todo.completed)
                .map(|todo| todo.clone())
                .collect();

            let select = Select::<Todo>::new()
                .with_prompt("Select a todo")
                .items(&todos)
                .default(0)
                .set_marked(completed_todos)
                .run();

            let marked_todo_ids: Vec<i32> = select
                .get_marked()
                .iter()
                .map(|todo| todo.id).
                collect();


            for todo in todos {
                let is_completed = marked_todo_ids.contains(&todo.id);
                todo_store
                    .update(TodoUpdate {
                        id: todo.id,
                        completed: is_completed,
                    })
                    .await
                    .unwrap();
            }
        }
        Some(("add", sub_matches)) => {
            let title = sub_matches.get_one::<String>("title").unwrap();
            let new_todo = TodoCreate { title: title.to_string() };
            todo_store.create(new_todo).await.unwrap();
        }
        Some(("delete", sub_matches)) => {
            let id_str = sub_matches.get_one::<String>("id").unwrap();
            let id: i32 = id_str.parse().unwrap();
            todo_store.delete_one(&id).await.unwrap();
        }
        Some(("clear", _)) => {
            todo_store.delete_all().await.unwrap();
        }
        _ => println!("No command found"),
    }
}
