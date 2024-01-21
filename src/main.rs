mod commands;
mod store;
mod db;

use clap::{Command};
use commands::todo_cmd::{add_todo, list_todos};
use store::todo_store::{TodoStore};
use store::todo::{TodoCreate};
use crate::db::connection::get_database_connection_pool;
use crate::store::storage::Storage;


#[tokio::main]
async fn main() {
    let connection_url = "sqlite:rustdo.db";
    let db = get_database_connection_pool(connection_url).await.unwrap_or_else(|err| {
        eprintln!("Could not connect to database: {}", err);
        std::process::exit(1);
    });

    sqlx::migrate!("./migrations").run(&db).await.unwrap();


    let mut todo_store = TodoStore::new(db);


    let arg = Command::new("rd")
        .about("Rust do is a todo list manager written in Rust")
        .subcommand_required(true)
        .subcommand(list_todos())
        .subcommand(add_todo());

    let matches = arg.get_matches();

    match matches.subcommand() {
        Some(("list", _)) => {
            let todos = todo_store.list().await.unwrap();
            for todo in todos {
                println!("{} - {}", todo.title, todo.created_at);
            }
        }
        Some(("add", sub_matches)) => {
            let title = sub_matches.get_one::<String>("title").unwrap();
            let new_todo = TodoCreate { title: title.to_string() };
            todo_store.add(new_todo).await.unwrap();
        }
        _ => println!("No command found"),
    }
}
