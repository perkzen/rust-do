use clap::{Command};

pub fn add_todo() -> Command {
    Command::new("add")
        .about("Add a new todo")
        .short_flag('a')
        .arg(
            clap::Arg::new("title")
                .required(true)
        )
}


pub fn list_todos() -> Command {
    Command::new("list")
        .about("List all todos")
        .short_flag('l')
}